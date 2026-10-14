#!/usr/bin/env python3
"""Check real Pulse source, admit KIR, and execute it through installed workers.

The resulting evidence is experimental. It deliberately records the missing
source refinement, per-lane lifting proof and dependency-closure replay.
"""
import argparse
import hashlib
import json
from pathlib import Path
import tempfile
import time

from run_portable import ROOT, Harness, buffer, invocation

FRONTEND = ROOT / 'frontend/kuiper'
EXAMPLES = FRONTEND / 'examples'


def export(harness, source, entry, output, accepted=(0,)):
    return harness.command(['python3', FRONTEND / 'export_source.py', source,
                            '--entry', entry, '--output', output], accepted=accepted)


def source_inputs():
    paths = sorted(p for top in (FRONTEND, ROOT / 'src', ROOT / 'portable/contracts',
                                 ROOT / 'portable/core', ROOT / 'backends/spirt',
                                 ROOT / 'backends/vulkan', ROOT / 'bindings/c')
                   for p in top.rglob('*') if p.is_file() and not any(
                       part in ('target', '_build', '__pycache__') for part in p.parts)
                   and p.suffix in ('.rs', '.toml', '.lock', '.ml', '.py', '.fst', '.fsti', '.h', '.c', '.json'))
    paths += [ROOT / 'validation/run_source.py', ROOT / 'validation/run_portable.py',
              ROOT / 'validation/vulkan_devices.py', ROOT / 'validation/run_source_hardware.py',
              ROOT / 'validation/fetch_source_evidence.py', ROOT / 'scripts/install-portable-worker.py']
    return {str(p.relative_to(ROOT)): hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--fixture-output', type=Path)
    args = parser.parse_args()
    args.output.unlink(missing_ok=True)
    started = time.monotonic()
    manifests = []
    vectors = []
    inputs_before = source_inputs()
    with tempfile.TemporaryDirectory(prefix='kuiper-source-integration-') as directory:
        h = Harness(directory, True)
        packages = {}
        for case, module, entry, inputs, params, groups, expected in (
            ('increment', 'Int32', 'increment', [buffer(0, [0xdeadbeef, 0, 1, 0xfffffffe, 0xffffffff, 0xabcdefab], 1, 4)], [], 1,
             [[0xdeadbeef, 1, 2, 0xffffffff, 0, 0xabcdefab]]),
            ('copy_add', 'Int32', 'copy_add', [buffer(0, [91, 0, 1, 0xfffffffe, 0xffffffff, 92], 1, 4),
                                    buffer(1, [81, 80, 79, 78, 77, 76, 75], 2, 4)], [2], 1,
             [[91, 0, 1, 0xfffffffe, 0xffffffff, 92], [81, 80, 2, 3, 0, 1, 75]]),
            ('early_increment', 'Int32', 'early_increment', [buffer(0, [51, 0, 1, 0xfffffffe, 0xffffffff, 52], 1, 4)], [], 1,
             [[51, 3, 4, 1, 2, 52]]),
            ('max_assign', 'Int32', 'max_assign', [buffer(0, [33, 0, 7, 0x80000000, 0xffffffff, 34], 1, 4)], [7], 1,
             [[33, 7, 7, 0x80000000, 0xffffffff, 34]]),
            ('conditional_left', 'Int32', 'conditional_increment',
             [buffer(0, [99, 0, 1, 0xfffffffe, 0xffffffff, 98], 1, 4),
              buffer(1, [88, 7, 8, 0xfffffffe, 0xffffffff, 87], 1, 4)], [0], 1,
             [[99, 1, 2, 0xffffffff, 0, 98], [88, 7, 8, 0xfffffffe, 0xffffffff, 87]]),
            ('conditional_right', 'Int32', 'conditional_increment',
             [buffer(0, [99, 0, 1, 0xfffffffe, 0xffffffff, 98], 1, 4),
              buffer(1, [88, 7, 8, 0xfffffffe, 0xffffffff, 87], 1, 4)], [1], 1,
             [[99, 0, 1, 0xfffffffe, 0xffffffff, 98], [88, 9, 10, 0, 1, 87]]),
            ('nested_increment', 'Int32', 'nested_increment',
             [buffer(0, [55, 0, 9, 10, 19, 20, 0x7fffffff, 0xfffffffe, 0xffffffff, 56], 1, 8)], [], 2,
             [[55, 1, 10, 12, 21, 23, 0x80000002, 1, 2, 56]]),
            ('condition_once', 'Control', 'condition_once',
             [buffer(0, [41, 0, 1, 0xfffffffe, 0xffffffff, 42], 1, 4)], [], 1,
             [[41, 2, 4, 1, 2, 42]]),
            ('matching_returns', 'Control', 'matching_returns',
             [buffer(0, [61, 0, 1, 0x80000000, 0xffffffff, 62], 1, 4)], [], 1,
             [[61, 7, 9, 9, 9, 62]]),
            ('asymmetric_return_skip', 'Control', 'asymmetric_return',
             [buffer(0, [71, 0, 1, 0xfffffffe, 0xffffffff, 72], 1, 4)], [0], 1,
             [[71, 0, 1, 0xfffffffe, 0xffffffff, 72]]),
            ('asymmetric_return_continue', 'Control', 'asymmetric_return',
             [buffer(0, [71, 0, 1, 0xfffffffe, 0xffffffff, 72], 1, 4)], [1], 1,
             [[71, 5, 6, 3, 4, 72]]),
            ('divide_three', 'Operations', 'divide_three',
             [buffer(0, [101, 0, 1, 2, 3, 2147483647, 2147483648, 4294967294, 4294967295, 102], 1, 8)], [], 2,
             [[101, 0, 0, 0, 1, 715827882, 715827882, 1431655764, 1431655765, 102]]),
            ('remainder_three', 'Operations', 'remainder_three',
             [buffer(0, [101, 0, 1, 2, 3, 2147483647, 2147483648, 4294967294, 4294967295, 102], 1, 8)], [], 2,
             [[101, 0, 1, 2, 0, 1, 2, 2, 0, 102]]),
            ('and_mask', 'Operations', 'and_mask',
             [buffer(0, [101, 0, 1, 2, 3, 2147483647, 2147483648, 4294967294, 4294967295, 102], 1, 8)], [], 2,
             [[101, 0, 1, 2, 3, 16711935, 0, 16711934, 16711935, 102]]),
            ('or_mask', 'Operations', 'or_mask',
             [buffer(0, [101, 0, 1, 2, 3, 2147483647, 2147483648, 4294967294, 4294967295, 102], 1, 8)], [], 2,
             [[101, 2147549184, 2147549185, 2147549186, 2147549187, 4294967295, 2147549184, 4294967294, 4294967295, 102]]),
            ('xor_mask', 'Operations', 'xor_mask',
             [buffer(0, [101, 0, 1, 2, 3, 2147483647, 2147483648, 4294967294, 4294967295, 102], 1, 8)], [], 2,
             [[101, 2863311530, 2863311531, 2863311528, 2863311529, 3579139413, 715827882, 1431655764, 1431655765, 102]]),
            ('not_bits', 'Operations', 'not_bits',
             [buffer(0, [101, 0, 1, 2, 3, 2147483647, 2147483648, 4294967294, 4294967295, 102], 1, 8)], [], 2,
             [[101, 4294967295, 4294967294, 4294967293, 4294967292, 2147483648, 2147483647, 1, 0, 102]]),
            ('shift_left_one', 'Operations', 'shift_left_one',
             [buffer(0, [101, 0, 1, 2, 3, 2147483647, 2147483648, 4294967294, 4294967295, 102], 1, 8)], [], 2,
             [[101, 0, 2, 4, 6, 4294967294, 0, 4294967292, 4294967294, 102]]),
            ('shift_right_31', 'Operations', 'shift_right_31',
             [buffer(0, [101, 0, 1, 2, 3, 2147483647, 2147483648, 4294967294, 4294967295, 102], 1, 8)], [], 2,
             [[101, 0, 0, 0, 0, 0, 1, 1, 1, 102]]),
        ):
            key = (module, entry)
            if key not in packages:
                output = Path(directory) / (module + '-' + entry)
                export(h, EXAMPLES / ('Kuiper.Portable.' + module + '.fst'), entry, output)
                packages[key] = json.loads((output / 'package.json').read_text())
                manifest = json.loads((output / 'source.json').read_text())
                assert manifest['source_entry_checked_strictly']
                assert manifest['policy'] == 'kuiper.experimental-tested/1'
                assert not manifest['source_to_kir_refinement']
                assert not manifest['per_lane_lifting_proved']
                assert not manifest['proof_dependency_closure_checked']
                assert manifest['project_dependency_types_loaded_freshly']
                assert manifest['toolchain']['schema'] == 'kuiper.source-toolchain/2'
                assert 'Z3 version 4.13.3 ' in manifest['toolchain']['solver']['version']
                manifests.append(manifest)
            package = packages[key]
            full = invocation(package, inputs, params, groups)
            h.success('checked-source-' + case, package, full, expected, c=True)
            vectors.append({'name': 'checked-source-' + case, 'entry': full['entry'],
                            'invocation': full, 'expected': expected, 'guard_mask': 0})
            short = invocation(package, [buffer(b['resource'], b['words'], b['offset'], groups * 4 - 1)
                                         for b in inputs], params, groups)
            if case == 'asymmetric_return_skip':
                # The selected early return performs no access. An inaccessible
                # lane must not guard against an unselected continuation.
                h.success('checked-source-short-view-' + case, package, short, expected, c=True)
                vectors.append({'name': 'checked-source-short-view-' + case, 'entry': short['entry'],
                                'invocation': short, 'expected': expected, 'guard_mask': 0})
            else:
                h.guard('checked-source-short-view-' + case, package, short, 1, c=True)
                vectors.append({'name': 'checked-source-short-view-' + case, 'entry': short['entry'],
                                'invocation': short, 'expected': None, 'guard_mask': 1})
            if module == 'Int32' and entry == 'increment' and args.fixture_output:
                args.fixture_output.parent.mkdir(parents=True, exist_ok=True)
                captures = json.loads((output / 'capture.json').read_text())
                minimal = [r for r in captures if r['name'].rsplit('.', 1)[1]
                           in ('increment', 'add_one', 'early_increment', 'early_add_three')]
                assert len(minimal) == 4
                args.fixture_output.write_text(json.dumps(minimal, indent=2) + '\n')
        rejected = {
            'increment_u64': 'unsupported kernel parameter type',
            'admitted': 'capture contains admit',
            'fractional_reads': 'entrypoint needs static/effect specialization',
        }
        for entry, expected in rejected.items():
            output = Path(directory) / entry
            result = export(h, EXAMPLES / 'Kuiper.Portable.Reject.fst', entry, output, accepted=(1,))
            error = json.loads(result.stderr.splitlines()[-1])
            assert error['code'] == 'source-rejected' and expected in error['message'], error
            assert not output.exists(), 'Rejected source published an executable package'
            h.record('reject-source-' + entry, 'source-admission-rejection', {'diagnostic': error})
        for module, entry in (('RejectAssume', 'assumed'), ('RejectNested', 'nested_bypass')):
            output = Path(directory) / module
            result = export(h, EXAMPLES / ('Kuiper.Portable.' + module + '.fst'), entry, output, accepted=(1,))
            error = json.loads(result.stderr.splitlines()[-1])
            assert 'Explicit proof bypass in checked source' in error['message'], error
            assert not output.exists(), 'A proof bypass published an executable package'
            h.record('reject-source-' + module, 'source-admission-rejection', {'diagnostic': error})
        # The hook observes each checked declaration before the compiler checks
        # later declarations. Completion of the whole source process is required.
        source = Path(directory) / 'Kuiper.Portable.Int32.fst'
        source.write_text((EXAMPLES / source.name).read_text() + '\nlet invalid_after_capture : int = true\n')
        output = Path(directory) / 'incomplete-module'
        result = export(h, source, 'increment', output, accepted=(1,))
        error = json.loads(result.stderr.splitlines()[-1])
        assert 'source verification failed' in error['message'] and not output.exists(), error
        h.record('reject-failure-after-live-capture', 'source-admission-rejection', {'diagnostic': error})
        if source_inputs() != inputs_before:
            raise AssertionError('Measured source inputs changed during CPU replay')
        h.assert_host_inputs_unchanged()
        report = {'schema': 'kuiper.source-integration/1', 'status': 'passed',
                  'assurance': 'kuiper.experimental-tested/1', 'production_ready': False,
                  'source_to_kir_refinement': False, 'per_lane_lifting_proved': False,
                  'proof_dependency_closure_checked': False, 'dependency_closure_replayed_freshly': False,
                  'physical_gpu_qualification': False, 'devices': sorted(h.devices),
                  'vulkan_validation_layer': 'VK_LAYER_KHRONOS_validation', 'validation_errors': 0,
                  'vulkaninfo_summary': h.device_info, 'case_count': len(h.cases), 'cases': h.cases,
                  'source_exports': manifests, 'source_packages': list(packages.values()),
                  'hardware_replay_vectors': vectors,
                  'candidate_checkout_sha': h.command(['git', '-C', ROOT, 'rev-parse', 'HEAD']).stdout.decode().strip(),
                  'workers': h.manifests, 'host_binaries': h.host_binaries,
                  'elapsed_seconds': round(time.monotonic() - started, 2),
                  'sources': inputs_before,
                  'harness_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(json.dumps(report, indent=2) + '\n')
        print(f'{len(h.cases)} checked-source execution/admission cases passed. Refinement and production qualification remain open.')


if __name__ == '__main__':
    main()
