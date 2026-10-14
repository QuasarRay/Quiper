"""Check hardware admission without relying on device names or a GPU."""
import os
from pathlib import Path
import tempfile
import unittest
from types import SimpleNamespace
from unittest.mock import patch

from run_portable import Harness

from vulkan_devices import device_types, driver_input, require_physical, select


def summary(*types):
    return '\n'.join(f'GPU{i}:\n    deviceType = PHYSICAL_DEVICE_TYPE_{kind}\n'
                     for i, kind in enumerate(types))


class DeviceAdmission(unittest.TestCase):
    def test_integrated_and_discrete_devices_pass(self):
        self.assertEqual(require_physical(summary('INTEGRATED_GPU', 'DISCRETE_GPU')),
                         ['PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU', 'PHYSICAL_DEVICE_TYPE_DISCRETE_GPU'])

    def test_cpu_virtual_unknown_and_mixed_pools_fail(self):
        for kinds in (('CPU',), ('VIRTUAL_GPU',), ('OTHER',), ('FUTURE_GPU',),
                      ('DISCRETE_GPU', 'CPU'), ('INTEGRATED_GPU', 'VIRTUAL_GPU')):
            with self.subTest(kinds=kinds), self.assertRaises(ValueError):
                require_physical(summary(*kinds))

    def test_names_and_unrelated_text_cannot_qualify_a_device(self):
        for text in ('NVIDIA DISCRETE_GPU', 'deviceName = PHYSICAL_DEVICE_TYPE_DISCRETE_GPU',
                     'deviceType = PHYSICAL_DEVICE_TYPE_CPU extra', '',
                     summary('DISCRETE_GPU') + '\n deviceType = PHYSICAL_DEVICE_TYPE_CPU extra'):
            with self.subTest(text=text), self.assertRaises(ValueError):
                require_physical(text)
        self.assertEqual(device_types(summary('CPU')), ['PHYSICAL_DEVICE_TYPE_CPU'])

    def test_driver_selection_skips_software_and_has_no_fallback(self):
        with tempfile.TemporaryDirectory() as directory:
            cpu, gpu = [Path(directory) / name for name in ('a-software.json', 'b-hardware.json')]
            for path in (cpu, gpu):
                path.write_text('{}')
            query = lambda path: summary('CPU' if path == cpu else 'DISCRETE_GPU')
            self.assertEqual(select([cpu, gpu], query)[0], gpu)
            with self.assertRaises(ValueError):
                select([cpu], query)
            with self.assertRaises(ValueError):
                select([], query)

    def test_hardware_replay_requires_one_measurable_manifest(self):
        with tempfile.TemporaryDirectory() as directory:
            driver = Path(directory) / 'hardware.json'
            driver.write_text('{}')
            self.assertEqual(driver_input({'VK_DRIVER_FILES': str(driver)}), driver.resolve())
            for value in ('', str(driver) + os.pathsep + str(driver), directory):
                with self.subTest(value=value), self.assertRaises((ValueError, OSError)):
                    driver_input({'VK_DRIVER_FILES': value})

    def test_validation_diagnostics_on_stdout_prevent_evidence(self):
        harness = Harness.__new__(Harness)
        harness.env = {}
        result = SimpleNamespace(returncode=0, stdout=b'Validation Error: VUID-example', stderr=b'')
        with patch('run_portable.subprocess.run', return_value=result), self.assertRaises(AssertionError):
            harness.command(['mock-worker'])
