#!/usr/bin/env python3
"""Replay successful, candidate-matched checked-source vectors on physical Vulkan."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import tempfile
import time

from run_portable import ROOT, Harness, digest


def unique(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            raise ValueError('Duplicate source evidence key')
        result[key] = value
    return result


def read_report(path):
    with path.open('rb') as stream:
        raw = stream.read(16 * 1024 * 1024 + 1)
    if len(raw) > 16 * 1024 * 1024:
        raise ValueError('Source evidence exceeded its budget')
    report = json.loads(raw, object_pairs_hook=unique,
                        parse_constant=lambda value: (_ for _ in ()).throw(ValueError('Nonfinite evidence value')))
    return report, hashlib.sha256(raw).hexdigest()


def admit_report(report, root, checkout_sha):
    if (report.get('schema') != 'kuiper.source-integration/1' or report.get('status') != 'passed'
            or report.get('assurance') != 'kuiper.experimental-tested/1'):
        raise ValueError('Hardware replay requires successful experimental checked-source evidence')
    if not re.fullmatch(r'[0-9a-f]{40}', checkout_sha) or report.get('candidate_checkout_sha') != checkout_sha:
        raise ValueError('Source evidence belongs to a different candidate checkout')
    for flag in ('production_ready', 'source_to_kir_refinement', 'per_lane_lifting_proved',
                 'proof_dependency_closure_checked', 'dependency_closure_replayed_freshly'):
        if report.get(flag) is not False:
            raise ValueError('Unsupported stronger source assurance claim')
    root = root.resolve(strict=True)
    sources = report.get('sources')
    if not isinstance(sources, dict) or not sources or len(sources) > 4096:
        raise ValueError('Missing bounded source input identities')
    for relative, expected in sources.items():
        path = Path(relative)
        if path.is_absolute() or '..' in path.parts:
            raise ValueError('Source input identity escapes the candidate')
        target = (root / path).resolve(strict=True)
        if not target.is_relative_to(root) or not re.fullmatch(r'[0-9a-f]{64}', expected):
            raise ValueError('Invalid source input identity')
        if hashlib.sha256(target.read_bytes()).hexdigest() != expected:
            raise ValueError('Source evidence bytes differ from this candidate: ' + relative)
    packages = report.get('source_packages')
    exports = report.get('source_exports')
    vectors = report.get('hardware_replay_vectors')
    if (not isinstance(packages, list) or not 1 <= len(packages) <= 64
            or not isinstance(exports, list) or len(exports) != len(packages)
            or not isinstance(vectors, list) or not 1 <= len(vectors) <= 128):
        raise ValueError('Missing bounded source packages or execution vectors')
    admitted = {}
    for package in packages:
        kernels = package.get('kernels', [])
        if len(kernels) != 1 or not isinstance(kernels[0].get('name'), str):
            raise ValueError('Source replay expects one named kernel per package')
        entry = kernels[0]['name']
        if entry in admitted:
            raise ValueError('Duplicate source replay entry')
        matching = [record for record in exports if record.get('entry') == entry]
        if len(matching) != 1:
            raise ValueError('Source package has no unique checked entry evidence')
        record = matching[0]
        if (record.get('source_entry_checked_strictly') is not True
                or record.get('policy') != 'kuiper.experimental-tested/1'
                or record.get('package_digest') != digest(package)):
            raise ValueError('Source package and checked entry evidence disagree')
        for flag in ('source_to_kir_refinement', 'per_lane_lifting_proved',
                     'proof_dependency_closure_checked', 'dependency_closure_replayed_freshly'):
            if record.get(flag) is not False:
                raise ValueError('Unsupported source export proof claim')
        admitted[entry] = package
    recorded = report.get('cases')
    if (not isinstance(recorded, list) or not 1 <= len(recorded) <= 256
            or type(report.get('case_count')) is not int or report['case_count'] != len(recorded)
            or any(not isinstance(case, dict) or not isinstance(case.get('name'), str)
                   or case.get('status') != 'passed' for case in recorded)
            or len({case['name'] for case in recorded}) != len(recorded)):
        raise ValueError('Incomplete CPU source result collection')
    names = set()
    for vector in vectors:
        entry = vector.get('entry')
        name = vector.get('name')
        if entry not in admitted or not isinstance(name, str) or name in names:
            raise ValueError('Unknown entry or duplicate source vector')
        names.add(name)
        invocation = vector.get('invocation')
        if not isinstance(invocation, dict) or invocation.get('entry') != entry:
            raise ValueError('Source vector invocation and entry disagree')
        if type(vector.get('guard_mask')) is not int or vector['guard_mask'] not in (0, 1):
            raise ValueError('Unsupported source guard outcome')
        cases = [case for case in recorded if case.get('name') == name]
        if (len(cases) != 1 or cases[0].get('status') != 'passed'
                or cases[0].get('package_digest') != digest(admitted[entry])
                or cases[0].get('invocation_digest') != digest(invocation)):
            raise ValueError('Source vector does not bind a completed CPU case')
        if vector['guard_mask'] == 0:
            if (not isinstance(vector.get('expected'), list) or cases[0].get('category') != 'execution'
                    or cases[0].get('literal_expected_digest') != digest(vector['expected'])):
                raise ValueError('Source vector expected output differs from the CPU check')
        elif (vector.get('expected') is not None or cases[0].get('category') != 'guard-rejection'
              or cases[0].get('guard_mask') != vector['guard_mask']):
            raise ValueError('Source vector guard outcome differs from the CPU check')
    completed = {case['name'] for case in recorded if case.get('category') in ('execution', 'guard-rejection')}
    if completed != names or set(admitted) != {vector['entry'] for vector in vectors}:
        raise ValueError('Source hardware replay cannot omit a completed source execution')
    return admitted, vectors


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--source-report', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    args.output.unlink(missing_ok=True)
    started = time.monotonic()
    report, input_digest = read_report(args.source_report)
    with tempfile.TemporaryDirectory(prefix='kuiper-source-hardware-') as directory:
        h = Harness(directory, True, require_gpu=True)
        checkout = h.command(['git', '-C', ROOT, 'rev-parse', 'HEAD']).stdout.decode().strip()
        packages, vectors = admit_report(report, ROOT, checkout)
        for vector in vectors:
            package = packages[vector['entry']]
            if vector['guard_mask'] == 0:
                h.success(vector['name'], package, vector['invocation'], vector['expected'], c=True)
            else:
                h.guard(vector['name'], package, vector['invocation'], vector['guard_mask'], c=True)
        h.assert_host_inputs_unchanged()
        admit_report(report, ROOT, checkout)
        if hashlib.sha256(args.source_report.read_bytes()).hexdigest() != input_digest:
            raise ValueError('CPU source evidence changed during hardware replay')
        if hashlib.sha256(h.driver_input.read_bytes()).hexdigest() != h.driver_input_digest:
            raise ValueError('Selected hardware driver manifest changed during replay')
        result = {'schema': 'kuiper.checked-source-hardware/1', 'status': 'passed',
                  'assurance': 'kuiper.experimental-tested/1', 'production_ready': False,
                  'candidate_checkout_sha': checkout, 'cpu_source_report_sha256': input_digest,
                  'source_to_kir_refinement': False, 'per_lane_lifting_proved': False,
                  'proof_dependency_closure_checked': False, 'physical_gpu_qualification': False,
                  'device_kind': 'physical_gpu', 'devices': sorted(h.devices),
                  'device_types': h.device_types, 'hardware_only_driver_required': True,
                  'selected_driver_manifest': str(h.driver_input),
                  'selected_driver_manifest_sha256': h.driver_input_digest,
                  'vulkan_validation_layer': 'VK_LAYER_KHRONOS_validation', 'validation_errors': 0,
                  'vulkaninfo_summary': h.device_info, 'case_count': len(h.cases), 'cases': h.cases,
                  'workers': h.manifests, 'host_binaries': h.host_binaries,
                  'elapsed_seconds': round(time.monotonic() - started, 2),
                  'replay_harness_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
                  'source_inputs': report['sources']}
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(json.dumps(result, indent=2) + '\n')
        print(f'{len(h.cases)} physical checked-source replay cases passed; production qualification remains open.')


if __name__ == '__main__':
    main()
