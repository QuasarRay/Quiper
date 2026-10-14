"""Reject incomplete, stale or altered checked-source hardware evidence."""
import copy
import hashlib
import json
from pathlib import Path
import tempfile
import unittest

from run_portable import digest
from run_source_hardware import admit_report, read_report

HEAD = 'a' * 40
ENTRY = 'Kuiper.Portable.Test.increment'


def evidence(root):
    source = root / 'input.fst'
    source.write_text('module Input\n')
    package = {'schema': 'kuiper.kir/1', 'profile': 'kuiper.integer32/1',
               'kernels': [{'name': ENTRY, 'local_size': [4, 1, 1],
                            'resources': [], 'parameters': [],
                            'body': {'instructions': [], 'outputs': []}}]}
    invocation = {'entry': ENTRY, 'workgroups': [1, 1, 1], 'parameters': [], 'buffers': []}
    expected = []
    flags = {'source_to_kir_refinement': False, 'per_lane_lifting_proved': False,
             'proof_dependency_closure_checked': False, 'dependency_closure_replayed_freshly': False}
    full = {'name': 'checked-source-test', 'entry': ENTRY, 'invocation': invocation,
            'expected': expected, 'guard_mask': 0}
    short = {'name': 'checked-source-short-view-test', 'entry': ENTRY,
             'invocation': {**invocation, 'parameters': [7]}, 'expected': None, 'guard_mask': 1}
    cases = [
        {'name': full['name'], 'category': 'execution', 'status': 'passed',
         'package_digest': digest(package), 'invocation_digest': digest(invocation),
         'literal_expected_digest': digest(expected)},
        {'name': short['name'], 'category': 'guard-rejection', 'status': 'passed',
         'package_digest': digest(package), 'invocation_digest': digest(short['invocation']),
         'guard_mask': 1}]
    return {'schema': 'kuiper.source-integration/1', 'status': 'passed',
            'assurance': 'kuiper.experimental-tested/1', 'production_ready': False,
            'candidate_checkout_sha': HEAD, **flags,
            'sources': {'input.fst': hashlib.sha256(source.read_bytes()).hexdigest()},
            'source_packages': [package],
            'source_exports': [{'entry': ENTRY, 'source_entry_checked_strictly': True,
                                'policy': 'kuiper.experimental-tested/1',
                                'package_digest': digest(package), **flags}],
            'hardware_replay_vectors': [full, short], 'cases': cases, 'case_count': len(cases)}


class SourceEvidenceAdmission(unittest.TestCase):
    def setUp(self):
        self.directory = tempfile.TemporaryDirectory()
        self.addCleanup(self.directory.cleanup)
        self.root = Path(self.directory.name)
        self.report = evidence(self.root)

    def assert_rejected(self, report=None, checkout=HEAD):
        with self.assertRaises((ValueError, OSError, TypeError, KeyError)):
            admit_report(self.report if report is None else report, self.root, checkout)

    def test_complete_bound_evidence_is_admitted(self):
        packages, vectors = admit_report(self.report, self.root, HEAD)
        self.assertEqual(set(packages), {ENTRY})
        self.assertEqual(len(vectors), 2)

    def test_changed_checkout_or_source_bytes_are_rejected(self):
        self.assert_rejected(checkout='b' * 40)
        (self.root / 'input.fst').write_text('module Different\n')
        self.assert_rejected()

    def test_source_paths_cannot_escape_the_candidate(self):
        for path in ('../input.fst', '/input.fst'):
            bad = copy.deepcopy(self.report)
            bad['sources'] = {path: next(iter(bad['sources'].values()))}
            self.assert_rejected(bad)

    def test_package_and_vector_mutations_cannot_reuse_case_digests(self):
        for section in ('source_packages', 'hardware_replay_vectors'):
            bad = copy.deepcopy(self.report)
            if section == 'source_packages':
                bad[section][0]['kernels'][0]['local_size'] = [8, 1, 1]
            else:
                bad[section][0]['invocation']['parameters'] = [3]
            self.assert_rejected(bad)
        bad = copy.deepcopy(self.report)
        bad['hardware_replay_vectors'][0]['expected'] = [[1]]
        self.assert_rejected(bad)

    def test_omitted_or_duplicate_execution_cannot_pass(self):
        bad = copy.deepcopy(self.report)
        bad['hardware_replay_vectors'].pop()
        self.assert_rejected(bad)
        bad = copy.deepcopy(self.report)
        bad['hardware_replay_vectors'].append(copy.deepcopy(bad['hardware_replay_vectors'][0]))
        self.assert_rejected(bad)
        bad = copy.deepcopy(self.report)
        bad['cases'].append(copy.deepcopy(bad['cases'][0]))
        bad['case_count'] += 1
        self.assert_rejected(bad)

    def test_failed_cpu_case_or_stronger_assurance_cannot_pass(self):
        bad = copy.deepcopy(self.report)
        bad['cases'][1]['status'] = 'failed'
        self.assert_rejected(bad)
        for flag in ('production_ready', 'source_to_kir_refinement', 'per_lane_lifting_proved',
                     'proof_dependency_closure_checked', 'dependency_closure_replayed_freshly'):
            bad = copy.deepcopy(self.report)
            bad[flag] = True
            self.assert_rejected(bad)
        bad = copy.deepcopy(self.report)
        bad['source_exports'][0]['source_to_kir_refinement'] = True
        self.assert_rejected(bad)

    def test_duplicate_nonfinite_and_oversized_json_are_rejected(self):
        path = self.root / 'evidence.json'
        for raw in (b'{"status":"passed","status":"failed"}', b'{"number":NaN}',
                    b' ' * (16 * 1024 * 1024 + 1)):
            path.write_bytes(raw)
            with self.assertRaises(ValueError):
                read_report(path)

    def test_report_digest_binds_the_exact_parsed_bytes(self):
        path = self.root / 'evidence.json'
        raw = json.dumps(self.report, indent=2).encode()
        path.write_bytes(raw)
        parsed, sha = read_report(path)
        self.assertEqual(parsed, self.report)
        self.assertEqual(sha, hashlib.sha256(raw).hexdigest())


if __name__ == '__main__':
    unittest.main()
