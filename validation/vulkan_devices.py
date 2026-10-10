#!/usr/bin/env python3
"""Select a measured hardware Vulkan ICD for experimental execution checks."""
import argparse
import os
from pathlib import Path
import re
import subprocess

PHYSICAL_TYPES = {'PHYSICAL_DEVICE_TYPE_DISCRETE_GPU', 'PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU'}


def device_types(summary):
    values = re.findall(r'^\s*deviceType\s*=\s*(.*?)\s*$', summary, re.MULTILINE)
    return [value if re.fullmatch(r'PHYSICAL_DEVICE_TYPE_[A-Z_]+', value)
            else 'UNCLASSIFIED_DEVICE_TYPE' for value in values]


def require_physical(summary):
    types = device_types(summary)
    if not types or any(kind not in PHYSICAL_TYPES for kind in types):
        raise ValueError('Hardware replay requires only integrated or discrete GPU devices; CPU, virtual, unknown and mixed pools are rejected')
    return types


def driver_input(environment):
    value = environment.get('VK_DRIVER_FILES', '')
    paths = value.split(os.pathsep)
    if len(paths) != 1 or not value:
        raise ValueError('Hardware replay requires exactly one explicit VK_DRIVER_FILES manifest')
    path = Path(paths[0]).resolve(strict=True)
    if not path.is_file() or path.suffix != '.json' or any(c in str(path) for c in '\r\n:'):
        raise ValueError('Invalid hardware driver manifest path')
    return path


def probe(path):
    env = dict(os.environ, VK_DRIVER_FILES=str(path),
               VK_INSTANCE_LAYERS='VK_LAYER_KHRONOS_validation')
    for name in ('VK_ICD_FILENAMES', 'VK_ADD_DRIVER_FILES', 'VK_LOADER_DRIVERS_SELECT',
                 'VK_LOADER_DRIVERS_DISABLE', 'VK_LOADER_LAYERS_DISABLE'):
        env.pop(name, None)
    result = subprocess.run(['vulkaninfo', '--summary'], stdout=subprocess.PIPE,
                            stderr=subprocess.PIPE, env=env, timeout=30, check=True)
    summary = result.stdout.decode(errors='replace')
    messages = summary + result.stderr.decode(errors='replace')
    if 'VUID-' in messages or 'Validation Error' in messages:
        raise ValueError('Vulkan validation diagnostics during hardware discovery')
    if 'VK_LAYER_KHRONOS_validation' not in summary:
        raise ValueError('Khronos validation layer is unavailable')
    require_physical(summary)
    return summary


def select(paths, query=probe):
    failures = []
    for path in sorted(set(Path(p).resolve(strict=True) for p in paths)):
        if not path.is_file() or path.suffix != '.json' or any(c in str(path) for c in '\r\n:'):
            continue
        try:
            summary = query(path)
            require_physical(summary)
            return path, summary
        except (OSError, ValueError, subprocess.SubprocessError) as error:
            failures.append(path.name + ': ' + str(error)[:300])
    raise ValueError('No hardware-only Vulkan driver passed admission. ' + '; '.join(failures))


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--environment-file', type=Path, required=True)
    parser.add_argument('--summary', type=Path, required=True)
    args = parser.parse_args()
    candidates = [p for directory in ('/etc/vulkan/icd.d', '/usr/share/vulkan/icd.d',
                                     '/usr/local/share/vulkan/icd.d')
                  for p in Path(directory).glob('*.json')]
    if len(candidates) > 64:
        raise ValueError('Driver manifest discovery exceeded its budget')
    path, summary = select(candidates)
    args.summary.write_text(summary)
    with args.environment_file.open('a') as environment:
        environment.write('VK_DRIVER_FILES=' + str(path) + '\n')
        for name in ('VK_ICD_FILENAMES', 'VK_ADD_DRIVER_FILES', 'VK_LOADER_DRIVERS_SELECT',
                     'VK_LOADER_DRIVERS_DISABLE', 'VK_LOADER_LAYERS_DISABLE'):
            environment.write(name + '=\n')
    print('Selected hardware driver: ' + str(path))
    print(summary)


if __name__ == '__main__':
    main()
