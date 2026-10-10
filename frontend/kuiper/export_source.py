#!/usr/bin/env python3
"""Capture checked Pulse, lower the closed profile, then admit neutral KIR."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import selectors
import signal
import subprocess
import tempfile
import time

from build_capture import HERE, LIBRARY_PATHS, ROOT, digest, tree_identity
from translate import Translator

MAX_BYTES = 16 * 1024 * 1024


def pairs(items):
    result = {}
    for key, value in items:
        if key in result:
            raise ValueError('duplicate JSON field: ' + key)
        result[key] = value
    return result


def read_json(path, limit=MAX_BYTES):
    with path.open('rb') as stream:
        data = stream.read(limit + 1)
    if len(data) > limit:
        raise ValueError('JSON input exceeds its frontend budget')
    return json.loads(data, object_pairs_hook=pairs)


def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(',', ':'), ensure_ascii=False).encode()


def compiler(command, env, log, timeout):
    # The compiler owns CPU work only. Reserve the leader PID until its process
    # group is terminated; a surviving child cannot change capture files later.
    with log.open('wb') as stream, selectors.DefaultSelector() as selector:
        child = subprocess.Popen(command, cwd=ROOT, env=env, stdout=subprocess.PIPE,
                                 stderr=subprocess.STDOUT, start_new_session=True)
        # Merge both streams through a bounded pipe. A compiler failure cannot
        # fill the operator's filesystem while its deadline is pending.
        output = child.stdout
        os.set_blocking(output.fileno(), False)
        selector.register(output, selectors.EVENT_READ)
        deadline = time.monotonic() + timeout
        finished = False
        received = 0
        try:
            while not finished or selector.get_map():
                for key, _ in selector.select(.01):
                    data = os.read(key.fd, 4096)
                    if not data:
                        selector.unregister(key.fileobj)
                        continue
                    received += len(data)
                    if received > 1048576:
                        raise ValueError('source verification output exceeded its evidence budget')
                    stream.write(data)
                    stream.flush()
                state = os.waitid(os.P_PID, child.pid, os.WEXITED | os.WNOHANG | os.WNOWAIT)
                finished = state is not None
                if finished:
                    try:
                        os.killpg(child.pid, signal.SIGKILL)
                    except ProcessLookupError:
                        pass
                if time.monotonic() >= deadline:
                    raise TimeoutError('strict source checking exceeded its deadline')
        finally:
            try:
                os.killpg(child.pid, signal.SIGKILL)
            except ProcessLookupError:
                pass
            code = child.wait()
            output.close()
    if code != 0:
        raise ValueError('source verification failed: ' + log.read_text()[-12000:])
    if log.stat().st_size > 1048576:
        raise ValueError('source verification output exceeded its evidence budget')
    if 'All verification conditions discharged successfully' not in log.read_text():
        raise ValueError('missing successful strict verifier completion')


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('source', type=Path)
    parser.add_argument('--entry', required=True)
    parser.add_argument('--local-size', type=int, default=4)
    parser.add_argument('--toolchain', type=Path, default=HERE / '_build/toolchain.json')
    parser.add_argument('--checker', type=Path, default=HERE / 'target/release/kuiper-source-check')
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--timeout', type=int, default=600)
    args = parser.parse_args()
    source = args.source.resolve(strict=True)
    if not re.fullmatch(r'[A-Z][A-Za-z0-9_]*(?:\.[A-Z][A-Za-z0-9_]*)+', source.stem):
        raise ValueError('source filename must carry its qualified F* module name')
    if (not re.fullmatch(r'[A-Za-z_][A-Za-z0-9_]{0,127}', args.entry)
            or not 1 <= args.timeout <= 3600 or not 1 <= args.local_size <= 256):
        raise ValueError('invalid entry or checking deadline')
    if source.stat().st_size > 1048576:
        raise ValueError('source input exceeds its frontend budget')
    target = args.output.resolve()
    if target.exists():
        raise ValueError('use a fresh output directory; source admission never reuses prior evidence')
    data = read_json(args.toolchain, 1048576)
    if data['schema'] != 'kuiper.source-toolchain/2':
        raise ValueError('incompatible coherent toolchain record')
    fstar = Path(data['fstar']).resolve(strict=True)
    plugin = args.toolchain.resolve().parent / 'kuiper_capture.cmxs'
    frontend = [HERE / name for name in ('capture.ml', 'translate.py', 'export_source.py',
                                        'build_capture.py', 'source-options.json')]
    before = {p: digest(p) for p in [source, fstar, plugin, args.checker.resolve(strict=True), *frontend]}
    if before[fstar] != data['fstar_sha256'] or before[plugin] != data['plugin_sha256']:
        raise ValueError('toolchain bytes differ from the coherent build record')
    if digest(HERE / 'capture.ml') != data['capture_source_sha256']:
        raise ValueError('capture source differs from the compiled plugin record')
    tree = fstar.parents[3]
    solver = Path(data['solver']['path']).resolve(strict=True)
    if digest(solver) != data['solver']['sha256']:
        raise ValueError('solver bytes differ from the coherent build record')
    before[solver] = data['solver']['sha256']
    libraries = data['library_inputs']
    if set(libraries) != set(LIBRARY_PATHS):
        raise ValueError('coherent build record must identify every required library input tree')
    for relative, expected in libraries.items():
        if tree_identity(tree / relative) != expected:
            raise ValueError('source library inputs differ from the coherent build record')
    project_identity = tree_identity(ROOT / 'src', ('.fst', '.fsti'))
    for relative, expected in data['hook_sources'].items():
        if digest(tree / relative) != expected:
            raise ValueError('Pulse hook sources changed after the coherent build')
        before[tree / relative] = expected
    for path, expected in data['interfaces'].items():
        if digest(path) != expected:
            raise ValueError('native interface bytes differ from the coherent build record')
        before[Path(path)] = expected
    source_options = read_json(HERE / 'source-options.json', 65536)
    if source_options['schema'] != 'kuiper.source-options/1':
        raise ValueError('incompatible strict source option record')
    flags = [str(fstar), '--include', str(ROOT / 'src'), '--smt', str(solver),
             *source_options['options']]
    with tempfile.TemporaryDirectory(prefix='kuiper-source-') as scratch:
        temporary = Path(scratch)
        for path in ('lib/common', 'lib/pulse', 'build/lib.common.checked', 'build/lib.pulse.checked'):
            flags += ['--include', str(tree / 'pulse' / path)]
        # The coherent compiler supplies its standard and Pulse libraries.
        # Project dependency types must be loaded in the fresh cache: a clean
        # checkout has no legacy obj/ cache, and stale project cache entries
        # must not silently replace changed Kuiper dependency source.
        # F* verifies explicit roots, not every automatically loaded import.
        # This does not constitute dependency proof replay or trust closure.
        flags += ['--cache_dir', scratch, '--odir', scratch,
                  '--already_cached', 'Prims,FStar,LowStar,Pulse,PulseCore,Steel',
                  '--cache_checked_modules', '--admit_smt_queries', 'false',
                  '--load_cmxs', str(plugin.with_suffix('')), str(source)]
        env = dict(os.environ, KUIPER_CAPTURE_MODULE=source.stem, KUIPER_CAPTURE_DIRECTORY=scratch)
        log = temporary / 'verification.log'
        compiler(flags, env, log, args.timeout)
        captures = []
        capture_bytes = 0
        for path in sorted(temporary.glob('*.json')):
            capture_bytes += path.stat().st_size
            if path.stat().st_size > 1048576 or len(captures) >= 64 or capture_bytes > MAX_BYTES:
                raise ValueError('capture package exceeds the frontend budget')
            record = read_json(path, 1048576)
            if record['schema'] != 'kuiper.pulse-capture/1':
                raise ValueError('incompatible capture schema')
            captures.append(record)
        records = {record['name']: record for record in captures}
        if len(records) != len(captures):
            raise ValueError('duplicate captured source declaration')
        name = source.stem + '.' + args.entry
        if name not in records:
            raise ValueError('the selected source declaration has no live checked capture')
        translator = Translator(records)
        package = translator.kernel(name, args.local_size)
        package_bytes = canonical(package)
        admission = subprocess.run([str(args.checker.resolve())], input=package_bytes,
                                   stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=30, check=True)
        package_digest = hashlib.sha256(package_bytes).hexdigest()
        if admission.stdout.decode().strip() != package_digest:
            raise ValueError('neutral admission digest does not bind this package')
        if any(digest(path) != expected for path, expected in before.items()):
            raise ValueError('source or tools changed during source admission')
        if (tree_identity(ROOT / 'src', ('.fst', '.fsti')) != project_identity
                or any(tree_identity(tree / p) != expected for p, expected in libraries.items())):
            raise ValueError('project or library inputs changed during source admission')
        manifest = {'schema': 'kuiper.experimental-source/1', 'status': 'passed',
                    'entry': name, 'package_digest': package_digest,
                    'source_sha256': before[source], 'toolchain': data,
                    'checker_sha256': before[args.checker.resolve()],
                    'frontend_sources': {p.name: before[p] for p in frontend},
                    'project_source_inputs': project_identity,
                    'project_dependency_types_loaded_freshly': True,
                    'verification': log.read_text(), 'options': flags,
                    'source_entry_checked_strictly': True,
                    'dependency_closure_replayed_freshly': False,
                    'proof_dependency_closure_checked': False,
                    'source_to_kir_refinement': False, 'per_lane_lifting_proved': False,
                    'policy': 'kuiper.experimental-tested/1',
                    'primitive_mappings': {'Kuiper.Ref.read': 'per-lane checked word load',
                        'Kuiper.Ref.write': 'per-lane checked word store',
                        'FStar.UInt32.add_mod/sub_mod/mul_mod': 'wrapping U32 arithmetic',
                        'FStar.UInt32.div/rem': 'U32 quotient/remainder with guarded invalid operands',
                        'FStar.UInt32.logand/logor/logxor/lognot': 'exact 32-bit word operations',
                        'FStar.UInt32.shift_left/shift_right': '32-bit shifts with guarded invalid counts',
                        'Pulse if': 'one condition evaluation, exclusive regions and typed result joins'},
                    'instructions': translator.mapping}
        target.parent.mkdir(parents=True, exist_ok=True)
        with tempfile.TemporaryDirectory(prefix='.kuiper-source-', dir=target.parent) as staging:
            stage = Path(staging) / 'admitted'
            stage.mkdir(mode=0o700)
            (stage / 'package.json').write_bytes(package_bytes + b'\n')
            (stage / 'source.json').write_text(json.dumps(manifest, indent=2) + '\n')
            (stage / 'capture.json').write_text(json.dumps(captures, indent=2) + '\n')
            if target.exists():
                raise ValueError('source publication target appeared during admission')
            os.rename(stage, target)
    print(f'Checked {name} and admitted executable integer KIR. Refinement remains open.')


if __name__ == '__main__':
    try:
        main()
    except (ValueError, OSError, TimeoutError, KeyError, RecursionError, subprocess.SubprocessError) as error:
        raise SystemExit(json.dumps({'code': 'source-rejected', 'message': str(error)})) from None
