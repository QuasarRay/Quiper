import importlib.util
from pathlib import Path
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location('installer', ROOT / 'scripts/install-portable-worker.py')
installer = importlib.util.module_from_spec(spec)
spec.loader.exec_module(installer)

class InstallerTests(unittest.TestCase):
    def worker(self, directory, body):
        path = Path(directory) / 'worker'
        path.write_text('#!/usr/bin/env python3\n' + body)
        path.chmod(0o755)
        return path

    def test_description_is_one_reply_with_unique_fields(self):
        for output in ('{"status":"description","status":"description"}', '{}\n{}'):
            with tempfile.TemporaryDirectory() as directory:
                path = self.worker(directory, 'print(' + repr(output) + ')\n')
                with self.assertRaises(ValueError):
                    installer.describe(path, 1)

    def test_deadline_includes_descendant_holding_pipe(self):
        with tempfile.TemporaryDirectory() as directory:
            path = self.worker(directory, 'import os,time\nif os.fork()==0: time.sleep(10)\nelse: os._exit(0)\n')
            with self.assertRaisesRegex(ValueError, 'deadline'):
                installer.describe(path, 0.1)

    def test_endpoint_contract_is_not_an_unchecked_claim(self):
        valid = {'status':'description', 'protocol':'kuiper.worker/1', 'id':'vendor/1',
                 'endpoints':[{'role':'runtime','consumes':['format/1'],'produces':['execution/1']}]}
        installer.validate_description(valid)
        valid['endpoints'][0]['extra'] = True
        with self.assertRaises(ValueError):
            installer.validate_description(valid)

if __name__ == '__main__':
    unittest.main()
