"""Exercise fail-closed source admission and its CPU process boundary."""
import json
import os
from pathlib import Path
import sys
import tempfile
import time
import unittest

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / 'frontend/kuiper'))
from export_source import compiler, read_json
from build_capture import tree_identity
from translate import Translator, Unsupported, reject_bypasses


class SourceBoundary(unittest.TestCase):
    def invoke(self, program, timeout=2):
        with tempfile.TemporaryDirectory() as directory:
            log = Path(directory) / 'log'
            compiler([sys.executable, '-c', program], dict(os.environ), log, timeout)
            return log.read_text()

    def test_nonzero_after_capture_cannot_pass(self):
        with self.assertRaisesRegex(ValueError, 'source verification failed'):
            self.invoke("print('All verification conditions discharged successfully'); raise SystemExit(1)")

    def test_zero_without_verifier_completion_cannot_pass(self):
        with self.assertRaisesRegex(ValueError, 'missing successful strict verifier completion'):
            self.invoke("print('written capture')")

    def test_compiler_output_is_bounded_before_completion(self):
        with self.assertRaisesRegex(ValueError, 'evidence budget'):
            self.invoke("import os,time; os.write(1,b'x'*1200000); time.sleep(30)")

    def test_timeout_terminates_descendants_before_return(self):
        with tempfile.TemporaryDirectory() as directory:
            marker = Path(directory) / 'pid'
            code = ('import os,time; p=os.fork(); '
                    f'open({str(marker)!r},"w").write(str(p)) if p else None; '
                    'time.sleep(30)')
            with self.assertRaises(TimeoutError):
                compiler([sys.executable, '-c', code], dict(os.environ), Path(directory) / 'log', .5)
            pid = int(marker.read_text())
            # A killed child may briefly remain as a zombie until init reaps
            # it. That state cannot write another capture or keep computing.
            stat = Path(f'/proc/{pid}/stat')
            deadline = time.monotonic() + 1
            while True:
                try:
                    state = stat.read_text().split(') ', 1)[1].split()[0]
                except (FileNotFoundError, ProcessLookupError):
                    break
                if state in ('Z', 'X'):
                    break
                if time.monotonic() >= deadline:
                    self.fail('Compiler descendant still runs after timeout termination')
                time.sleep(.01)

    def test_duplicate_json_field_is_rejected(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / 'capture.json'
            path.write_text('{"status":"passed","status":"rejected"}')
            with self.assertRaisesRegex(ValueError, 'duplicate JSON field'):
                read_json(path)

    def test_dependency_identity_binds_bytes_and_relative_names(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source = root / 'Dependency.fsti'
            source.write_text('module Dependency\nval value : int\n')
            original = tree_identity(root, ('.fst', '.fsti'))
            source.write_text('module Dependency\nval value : bool\n')
            changed = tree_identity(root, ('.fst', '.fsti'))
            self.assertEqual(changed['file_count'], original['file_count'])
            self.assertNotEqual(changed['sha256'], original['sha256'])
            source.rename(root / 'Different.fsti')
            self.assertNotEqual(tree_identity(root, ('.fst', '.fsti'))['sha256'], changed['sha256'])

    def test_proof_bypasses_in_unsupported_subtrees_are_rejected(self):
        for tag in ('admit', 'unreachable', 'pragma'):
            with self.subTest(tag=tag), self.assertRaises(Unsupported):
                reject_bypasses({'tag': 'unsupported', 'constructor': 'Tm_let',
                                 'branches': [{'body': {'tag': tag}}]})

    def test_pinned_library_bypass_names_are_rejected(self):
        for name in ('Prims._assume', 'Prims.magic', 'Prims.unsafe_coerce',
                     'Pulse.Lib.Core.assume_', 'Pulse.Lib.Core.stt_admit',
                     'Pulse.Lib.Core.stt_atomic_admit', 'Pulse.Lib.Core.stt_ghost_admit'):
            with self.subTest(name=name), self.assertRaises(Unsupported):
                reject_bypasses({'tag': 'stateful_apply', 'effect': 'ghost',
                    'function': {'tag': 'symbol', 'name': name}})

    def test_ghost_input_cannot_become_an_executable_word(self):
        source = ROOT / 'validation/fixtures/source-increment-capture.json'
        captures = json.loads(source.read_text())
        declarations = {record['name']: record for record in captures}
        fn = 'Kuiper.Portable.Int32.increment'
        # Replace the checked U32 write argument by its erased initial value.
        def mutate(node):
            if isinstance(node, dict):
                if node.get('tag') == 'stateful_apply':
                    call = node['function']
                    if call.get('head', {}).get('name') == 'Kuiper.Ref.write':
                        erased = call['arguments'][-1]['value']
                        self.assertTrue(call['arguments'][-1]['implicit'])
                        explicit = [arg for arg in call['arguments'] if not arg['implicit']]
                        explicit[-1]['value'] = erased
                        return True
                return any(mutate(child) for child in node.values())
            return isinstance(node, list) and any(mutate(child) for child in node)
        self.assertTrue(mutate(declarations[fn]))
        with self.assertRaises(Unsupported):
            Translator(declarations).kernel(fn, 4)

    def mutate_early_return(self, field, replacement, diagnostic):
        source = ROOT / 'validation/fixtures/source-increment-capture.json'
        captures = json.loads(source.read_text())
        declarations = {record['name']: record for record in captures}
        fn = 'Kuiper.Portable.Int32.early_add_three'
        def mutate(node):
            if isinstance(node, dict):
                if node.get('tag') == 'jump':
                    node[field] = replacement
                    return True
                return any(mutate(child) for child in node.values())
            return isinstance(node, list) and any(mutate(child) for child in node)
        self.assertTrue(mutate(declarations[fn]))
        with self.assertRaisesRegex(Unsupported, diagnostic):
            Translator(declarations).kernel('Kuiper.Portable.Int32.early_increment', 4)

    def test_return_cannot_target_a_non_label_value(self):
        self.mutate_early_return('label', {'tag': 'unit'}, 'expected label')

    def test_return_representation_must_match_its_label(self):
        self.mutate_early_return('argument', {'tag': 'unit'}, 'label result type disagrees')


if __name__ == '__main__':
    unittest.main()
