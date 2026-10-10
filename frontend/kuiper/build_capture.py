#!/usr/bin/env python3
"""Build the frontend plugin against the same compiler and Pulse native units."""
import argparse
import hashlib
import json
from pathlib import Path
import shutil
import subprocess

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
LIBRARY_PATHS = ('stage3/out/lib/fstar/ulib', 'stage3/out/lib/fstar/ulib.checked',
                 'pulse/lib/common', 'pulse/build/lib.common.checked',
                 'pulse/lib/pulse', 'pulse/build/lib.pulse.checked')


def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def tree_identity(path, suffixes=None):
    path = Path(path).resolve(strict=True)
    files = {str(p.relative_to(path)): digest(p) for p in sorted(path.rglob('*'))
             if p.is_file() and (suffixes is None or p.suffix in suffixes)}
    if not files:
        raise ValueError('empty source/library input tree: ' + str(path))
    data = json.dumps(files, sort_keys=True, separators=(',', ':')).encode()
    return {'file_count': len(files), 'sha256': hashlib.sha256(data).hexdigest()}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--fstar-root', type=Path, default=ROOT / 'FStar')
    parser.add_argument('--output', type=Path, default=HERE / '_build')
    args = parser.parse_args()
    tree = args.fstar_root.resolve(strict=True)
    build = tree / 'stage3/dune/_build/default'
    paths = [build / 'fstar-guts/.fstarcompiler.objs/byte',
             build / 'fstar-guts/.fstarcompiler.objs/native',
             build / 'pulse-plugin/.pulse_plugin.objs/byte',
             build / 'pulse-plugin/.pulse_plugin.objs/native']
    interfaces = [paths[0] / 'fStarC_Syntax_Syntax.cmi', paths[2] / 'pulse_Syntax_Base.cmi',
                  paths[2] / 'pulse_RuntimeUtils.cmi']
    binary = tree / 'stage3/out/bin/fstar.exe'
    for path in [*interfaces, binary]:
        if not path.is_file():
            raise SystemExit(f'Missing coherent stage3 build artifact: {path}')
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=True)
    source = output / 'kuiper_capture.ml'
    shutil.copyfile(HERE / 'capture.ml', source)
    command = ['ocamlfind', 'ocamlopt', '-shared', '-package', 'yojson,zarith']
    for path in paths:
        command += ['-I', str(path)]
    command += [str(source), '-o', str(output / 'kuiper_capture.cmxs')]
    subprocess.run(command, cwd=output, timeout=120, check=True)
    solver_name = subprocess.check_output([str(binary), '--locate_z3', '4.13.3'],
                                          text=True, timeout=30).strip()
    solver = Path(shutil.which(solver_name) or solver_name).resolve(strict=True)
    solver_version = subprocess.check_output([str(solver), '-version'], text=True, timeout=30).strip()
    if not solver_version.startswith('Z3 version 4.13.3 '):
        raise ValueError('source toolchain requires measured Z3 4.13.3')
    report = {'schema': 'kuiper.source-toolchain/2', 'fstar': str(binary),
              'fstar_source_commit': subprocess.check_output(
                  ['git', 'rev-parse', 'HEAD'], cwd=tree, text=True).strip(),
              'fstar_sha256': digest(binary),
              'fstar_version': subprocess.check_output([str(binary), '--version'], text=True),
              'plugin_sha256': digest(output / 'kuiper_capture.cmxs'),
              'capture_source_sha256': digest(HERE / 'capture.ml'),
              'solver': {'path': str(solver), 'sha256': digest(solver), 'version': solver_version},
              'library_inputs': {p: tree_identity(tree / p) for p in LIBRARY_PATHS},
              'library_dependency_proofs_replayed_freshly': False,
              'interfaces': {str(p): digest(p) for p in interfaces},
              'hook_sources': {str(p.relative_to(tree)): digest(p) for p in [
                  tree / 'pulse/src/checker/Pulse.Main.fst',
                  tree / 'pulse/src/checker/Pulse.RuntimeUtils.fsti',
                  tree / 'pulse/src/ml/Pulse_RuntimeUtils.ml']}}
    (output / 'toolchain.json').write_text(json.dumps(report, indent=2) + '\n')
    print('Built the checked-Pulse capture plugin against one coherent stage3 toolchain.')


if __name__ == '__main__':
    try:
        main()
    except (ValueError, OSError, subprocess.SubprocessError) as error:
        raise SystemExit(str(error)) from None
