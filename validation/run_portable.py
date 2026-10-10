#!/usr/bin/env python3
"""Exercise installed workers through the core and C ABI; record real results.

These tests establish behavior for the bounded integer profile. They are not
source verification, refinement proofs, or physical GPU qualification.
"""
import argparse
import copy
import hashlib
import json
import os
from pathlib import Path
import subprocess
import tempfile
import time

from vulkan_devices import device_types, driver_input, require_physical

ROOT = Path(__file__).resolve().parents[1]
MASK = (1 << 32) - 1


def encoded(value):
    return json.dumps(value, sort_keys=True, separators=(',', ':'), ensure_ascii=False).encode()


def digest(value):
    return hashlib.sha256(encoded(value)).hexdigest()


def fixture(name):
    directory = ROOT / 'backends/spirt/tests/fixtures'
    return json.loads((directory / (name + '.json')).read_text())


def region(instructions, outputs=()):
    return {'instructions': instructions, 'outputs': list(outputs)}


def const(identity, ty, bits):
    return {'kind': 'constant', 'result': {'id': identity, 'ty': ty}, 'bits': bits & MASK}


def package(instructions, resources, parameters=()):
    return {'schema': 'kuiper.kir/1', 'profile': 'kuiper.integer32/1', 'kernels': [{
        'name': 'test_kernel', 'local_size': [4, 1, 1], 'resources': resources,
        'parameters': list(parameters), 'body': region(instructions)}]}


def resource(identity, element='u32', access='write'):
    return {'id': identity, 'element': element, 'access': access}


def invocation(package, buffers, parameters=(), groups=1):
    return {'entry': package['kernels'][0]['name'], 'workgroups': [groups, 1, 1],
            'parameters': list(parameters), 'buffers': buffers}


def buffer(identity, words, offset=0, length=None):
    return {'resource': identity, 'words': words, 'offset': offset,
            'length': len(words) - offset if length is None else length}


def global_id(identity=1):
    return {'kind': 'builtin', 'result': identity, 'builtin': 'global_id', 'axis': 0}


def store(res, index, value):
    return {'kind': 'store', 'resource': res, 'index': index, 'value': value}


class Harness:
    def __init__(self, directory, release, require_gpu=False):
        self.directory = Path(directory)
        self.workers = self.directory / 'workers'
        self.cache = self.directory / 'cache'
        self.mode = 'release' if release else 'debug'
        self.core = ROOT / f'portable/core/target/{self.mode}/kuiper-core'
        self.runtime = ROOT / f'backends/vulkan/target/{self.mode}/kuiper-vulkan-worker'
        self.driver = self.directory / 'c-driver'
        self.cases = []
        self.devices = set()
        self.env = dict(os.environ, XDG_RUNTIME_DIR=str(self.directory),
                        VK_INSTANCE_LAYERS='VK_LAYER_KHRONOS_validation')
        self.env.pop('VK_LOADER_LAYERS_DISABLE', None)
        self.directory.chmod(0o700)
        device_info = self.command(['vulkaninfo', '--summary']).stdout.decode(errors='replace')
        if 'VK_LAYER_KHRONOS_validation' not in device_info:
            raise AssertionError('Qualification run requires the Khronos validation layer')
        self.device_info = device_info
        self.device_types = device_types(device_info)
        self.driver_input = None
        if require_gpu:
            require_physical(device_info)
            self.driver_input = driver_input(self.env)
        self.driver_input_digest = (hashlib.sha256(self.driver_input.read_bytes()).hexdigest()
                                    if self.driver_input else None)
        before = self.core_sources()
        immutable_hosts = [self.core, ROOT / f'bindings/c/target/{self.mode}/libkuiper_c.so']
        host_digests = {p: hashlib.sha256(p.read_bytes()).hexdigest() for p in immutable_hosts}
        for name in ('spirt', 'vulkan'):
            binary = ROOT / f'backends/{name}/target/{self.mode}/kuiper-{name}-worker'
            self.command(['python3', ROOT / 'scripts/install-portable-worker.py',
                          '--root', self.workers, binary])
        assert self.core_sources() == before, 'Installing workers changed core sources'
        assert all(hashlib.sha256(p.read_bytes()).hexdigest() == sha for p, sha in host_digests.items()), 'Installing workers changed host binaries'
        self.command(['cc', '-std=c11', '-Wall', '-Wextra', '-Werror',
                      '-I', ROOT / 'bindings/c/include', ROOT / 'bindings/c/test/driver.c',
                      '-L', ROOT / f'bindings/c/target/{self.mode}', '-lkuiper_c',
                      '-Wl,-rpath,' + str(ROOT / f'bindings/c/target/{self.mode}'), '-o', self.driver])
        self.host_paths = dict(zip(('core', 'c_binding', 'c_driver'), [*immutable_hosts, self.driver]))
        self.host_binaries = {name: hashlib.sha256(p.read_bytes()).hexdigest()
                              for name, p in self.host_paths.items()}
        self.manifests = self.cli('inspect', self.workers)
        assert len(self.manifests) == 2
        self.record('additive-worker-installation', 'installation', {'workers': self.manifests})

    def assert_host_inputs_unchanged(self):
        current = {name: hashlib.sha256(p.read_bytes()).hexdigest()
                   for name, p in self.host_paths.items()}
        if current != self.host_binaries:
            raise AssertionError('Host execution binaries changed during replay')

    @staticmethod
    def core_sources():
        return {str(p.relative_to(ROOT)): hashlib.sha256(p.read_bytes()).hexdigest()
                for top in ('portable/core', 'portable/contracts')
                for p in (ROOT / top).rglob('*')
                if p.is_file() and 'target' not in p.parts}

    def command(self, arguments, raw=None, accepted=(0,)):
        result = subprocess.run([str(x) for x in arguments], input=raw,
                                stdout=subprocess.PIPE, stderr=subprocess.PIPE, env=self.env)
        stderr = result.stderr.decode(errors='replace')
        messages = stderr + result.stdout.decode(errors='replace')
        if 'VUID-' in messages or 'Validation Error' in messages:
            raise AssertionError('Vulkan validation error: ' + messages[-8000:])
        if result.returncode not in accepted:
            raise AssertionError(f'{arguments[0]} exited {result.returncode}: {stderr[-8000:]}')
        return result

    def path(self, name, value):
        path = self.directory / name
        path.write_bytes(encoded(value))
        return path

    def cli(self, *arguments):
        return json.loads(self.command([self.core, *arguments]).stdout)

    def record(self, name, category, evidence):
        self.cases.append({'name': name, 'category': category, 'status': 'passed', **evidence})
        print(name, 'passed', flush=True)

    def success(self, name, package, invocation, expected, c=False):
        pp = self.path('package.json', package)
        ip = self.path('invocation.json', invocation)
        reference = self.cli('reference', pp, ip)
        assert reference['guard'] == 0, (name, reference)
        assert [b['words'] for b in reference['buffers']] == expected, (name, reference, expected)
        actual = self.cli('run', pp, ip, self.workers, '--allow-experimental')
        assert actual['guard'] == 0 and actual['buffers'] == reference['buffers'], (name, actual, reference)
        self.devices.add(actual['device'])
        if c:
            result = json.loads(self.command([self.driver, self.workers, pp, ip, 'kernel']).stdout)
            assert result['buffers'] == actual['buffers'] and result['guard'] == 0
        self.record(name, 'execution', {'package_digest': digest(package),
                    'invocation_digest': digest(invocation), 'output_digest': digest(actual['buffers']),
                    'literal_expected_digest': digest(expected), 'interfaces': ['core', 'c'] if c else ['core']})

    def guard(self, name, package, invocation, mask, c=False):
        pp = self.path('package.json', package)
        ip = self.path('invocation.json', invocation)
        reference = self.cli('reference', pp, ip)
        assert reference['guard'] == mask, (name, reference['guard'], mask)
        result = self.command([self.core, 'run', pp, ip, self.workers, '--allow-experimental'], accepted=(1,))
        error = json.loads(result.stderr.splitlines()[-1])
        assert error['code'] == 'guard-failed' and f'0x{mask:08x}' in error['message'], (name, error)
        assert not result.stdout.strip(), 'Failed GPU buffer version was published'
        if c:
            error = json.loads(self.command([self.driver, self.workers, pp, ip, 'kernel'], accepted=(3,)).stdout)
            assert error['code'] == 'guard-failed'
        self.record(name, 'guard-rejection', {'package_digest': digest(package),
                    'invocation_digest': digest(invocation), 'guard_mask': mask,
                    'interfaces': ['core', 'c'] if c else ['core']})

    def tamper(self, name, artifact, invocation):
        response = json.loads(self.command([self.runtime, 'worker'],
            encoded({'method': 'execute', 'artifact': artifact, 'invocation': invocation}) + b'\n').stdout)
        assert response['status'] == 'rejected', (name, response)
        self.record(name, 'artifact-rejection', {'diagnostic': response['diagnostic'],
                    'artifact_digest': digest(artifact)})

    def reject_source(self, name, package, invocation, code, check=False):
        pp = self.path('package.json', package)
        ip = self.path('invocation.json', invocation)
        arguments = [self.core, 'check', pp] if check else [
            self.core, 'run', pp, ip, self.workers, '--allow-experimental']
        result = self.command(arguments, accepted=(1,))
        error = json.loads(result.stderr.splitlines()[-1])
        assert error['code'] == code and not result.stdout.strip(), (name, error)
        self.record(name, 'source-rejection', {'diagnostic':error,'package_digest':digest(package)})


def execution_inputs():
    paths = sorted(p for top in ('portable/contracts', 'portable/core', 'backends',
                                'bindings', 'validation/fixtures')
                   for p in (ROOT / top).rglob('*') if p.is_file()
                   and p.suffix in ('.rs', '.toml', '.lock', '.h', '.c', '.json')
                   and not any(part in ('target', 'results', '__pycache__') for part in p.parts))
    paths += [ROOT / 'validation/run_portable.py', ROOT / 'validation/vulkan_devices.py',
              ROOT / 'scripts/install-portable-worker.py']
    return {str(p.relative_to(ROOT)): hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}


def arithmetic(h):
    # Literal expected values are separate from both the Rust evaluator and the backend.
    cases = [
        ('u32', 'add', MASK, 1, 0), ('u32', 'sub', 0, 1, MASK),
        ('u32', 'mul', 0x80000000, 2, 0), ('u32', 'div', 13, 3, 4),
        ('u32', 'rem', 13, 3, 1), ('u32', 'bit_and', 0xaa, 0x55, 0),
        ('u32', 'bit_or', 0xaa, 0x55, 0xff), ('u32', 'bit_xor', 0xaa, 0xff, 0x55),
        ('u32', 'shift_left', 1, 31, 0x80000000), ('u32', 'shift_right', MASK, 31, 1),
        ('u32', 'eq', MASK, MASK, 1), ('u32', 'ne', MASK, 0, 1),
        ('u32', 'lt', 0, MASK, 1), ('u32', 'le', MASK, MASK, 1),
        ('u32', 'gt', MASK, 0, 1), ('u32', 'ge', MASK, MASK, 1),
        ('i32', 'add', 0x7fffffff, 1, 0x80000000), ('i32', 'sub', 0x80000000, 1, 0x7fffffff),
        ('i32', 'mul', -7, 3, -21), ('i32', 'div', -7, 3, -2),
        ('i32', 'div', 7, -3, -2), ('i32', 'rem', -7, 3, -1),
        ('i32', 'rem', 7, -3, 1), ('i32', 'shift_right', 0x80000000, 31, MASK),
        ('i32', 'shift_left', -1, 1, -2), ('i32', 'bit_xor', -1, 0xff, 0xffffff00),
        ('i32', 'eq', -1, -1, 1), ('i32', 'ne', -1, 0, 1),
        ('i32', 'lt', -1, 0, 1), ('i32', 'le', -1, -1, 1),
        ('i32', 'gt', 0, -1, 1), ('i32', 'ge', -1, -1, 1),
        ('bool', 'logical_and', 1, 0, 0), ('bool', 'logical_or', 1, 0, 1),
        ('bool', 'eq', 0, 0, 1), ('bool', 'ne', 0, 1, 1),
    ]
    for n, (ty, op, a, b, expected) in enumerate(cases):
        comparison = op in ('eq', 'ne', 'lt', 'le', 'gt', 'ge', 'logical_and', 'logical_or')
        out_ty = 'bool' if comparison else ty
        count_ty = 'u32' if op in ('shift_left', 'shift_right') else ty
        p = package([global_id(), const(2, ty, a), const(3, count_ty, b),
            {'kind': 'binary', 'result': 4, 'op': op, 'left': 2, 'right': 3}, store(0, 1, 4)],
            [resource(0, out_ty)])
        i = invocation(p, [buffer(0, [0, 0, 0, 0, 0, 0], 1, 4)])
        h.success(f'arithmetic-{ty}-{op}-{n}', p, i, [[0] + [expected & MASK] * 4 + [0]], c=n in (0, 19, 33))
    for ty, op, a, b in [('u32', 'div', 10, 0), ('u32', 'rem', 10, 0),
                          ('i32', 'div', 0x80000000, MASK), ('i32', 'rem', 0x80000000, MASK),
                          ('u32', 'shift_left', 1, 32), ('u32', 'shift_right', 1, 32),
                          ('i32', 'shift_right', MASK, 32)]:
        count_ty = 'u32' if op in ('shift_left', 'shift_right') else ty
        p = package([global_id(), const(2, ty, a), const(3, count_ty, b),
                     {'kind': 'binary', 'result': 4, 'op': op, 'left': 2, 'right': 3}, store(0, 1, 4)],
                    [resource(0, ty)])
        h.guard(f'arithmetic-undefined-{ty}-{op}-{b}', p, invocation(p, [buffer(0, [9]*4)]), 2)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--output', required=True)
    parser.add_argument('--release', action='store_true')
    parser.add_argument('--require-physical-gpu', action='store_true')
    args = parser.parse_args()
    # Never leave a prior passing report behind after a failed candidate run.
    Path(args.output).unlink(missing_ok=True)
    started = time.time()
    inputs_before = execution_inputs()
    with tempfile.TemporaryDirectory(prefix='kuiper-integration-') as directory:
        h = Harness(directory, args.release, args.require_physical_gpu)
        p = fixture('guarded-vector-add')
        i = invocation(p, [buffer(0, [99, 1, 2, 3, 4, 88], 1, 4),
                           buffer(1, [77, 10, 20, 30, 40, 66], 1, 4),
                           buffer(2, [55, 0, 0, 0, 0, 44], 1, 4)])
        h.success('vector-add-interior-views', p, i,
                  [[99, 1, 2, 3, 4, 88], [77, 10, 20, 30, 40, 66], [55, 11, 22, 33, 44, 44]], c=True)
        ragged = copy.deepcopy(i)
        ragged['buffers'][0]['length'] = 3
        h.success('vector-add-ragged-dispatch', p, ragged,
                  [[99, 1, 2, 3, 4, 88], [77, 10, 20, 30, 40, 66], [55, 11, 22, 33, 0, 44]])
        loop = fixture('pretested-loop')
        for n in (0, 1, 3, 128):
            i = invocation(loop, [buffer(0, [99, 0, 0, 0, 0, 88], 1, 4)], [n])
            h.success(f'pretested-loop-{n}', loop, i, [[99] + [n]*4 + [88]], c=n == 0)
        h.guard('pretested-loop-exhaustion', loop,
                invocation(loop, [buffer(0, [9]*4)], [129]), 4, c=True)
        p = fixture('condition-effects')
        for n in (0, 1, 3):
            i = invocation(p, [buffer(0, [99, 0, 88], 1, 1), buffer(1, [77, 0, 0, 0, 0, 66], 1, 4)], [n])
            h.success(f'condition-evaluated-n-plus-one-{n}', p, i,
                      [[99, 4*(n+1), 88], [77] + [n]*4 + [66]])
        p = fixture('nested-loops')
        for outer, inner in ((0, 3), (3, 0), (2, 3)):
            i = invocation(p, [buffer(0, [99, 0, 0, 0, 0, 88], 1, 4)], [outer, inner])
            h.success(f'nested-loop-{outer}-{inner}', p, i, [[99] + [outer*inner]*4 + [88]])
        p = fixture('atomic-add')
        i = invocation(p, [buffer(0, [99, MASK-1, 88], 1, 1)])
        h.success('atomic-add-wrap-and-frame', p, i, [[99, 2, 88]])
        arithmetic(h)
        p = package([global_id(), const(2, 'bool', 0), {'kind': 'guard', 'condition': 2, 'code': 8},
                     const(3, 'u32', 10), store(0, 1, 3)], [resource(0)])
        h.guard('explicit-guard-no-published-output', p, invocation(p, [buffer(0, [0]*4)]), 8)
        p = package([global_id(), {'kind': 'load', 'result': 2, 'resource': 0, 'index': 1}, store(1, 1, 2)],
                    [resource(0, access='read'), resource(1)])
        h.guard('checked-load-oob', p, invocation(p, [buffer(0, [1,2,3]), buffer(1, [0]*4)]), 1)
        h.guard('checked-store-oob', p, invocation(p, [buffer(0, [1,2,3,4]), buffer(1, [0]*3)]), 1)
        p = package([global_id(), {'kind': 'parameter', 'result': 2, 'index': 0},
                     const(3, 'i32', -1), const(4, 'i32', 10),
                     {'kind': 'select', 'condition': 2, 'then_region': region([], [3]),
                      'else_region': region([], [4]), 'results': [{'id': 5, 'ty': 'i32'}]},
                     store(0, 1, 5)], [resource(0, 'i32')], ['bool'])
        for truth, expected in ((0,10), (1,MASK)):
            h.success(f'select-final-value-{truth}', p, invocation(p, [buffer(0, [0]*4)], [truth]), [[expected]*4])
        p = package([global_id(), {'kind': 'builtin', 'result': 2, 'builtin': 'local_id', 'axis': 0},
                     {'kind': 'builtin', 'result': 3, 'builtin': 'workgroup_id', 'axis': 0},
                     {'kind': 'builtin', 'result': 4, 'builtin': 'num_workgroups', 'axis': 0},
                     store(0,1,2), store(1,1,3), store(2,1,4)], [resource(0),resource(1),resource(2)])
        h.success('builtin-multiple-workgroups', p, invocation(p, [buffer(n,[0]*8) for n in range(3)], groups=2),
                  [[0,1,2,3]*2, [0]*4+[1]*4, [2]*8])
        # Compile once, then tamper after any compiler inspection. The runtime must recheck.
        pp = h.path('package.json', loop)
        artifact = h.cli('compile', pp, loop['kernels'][0]['name'], h.workers, h.cache)
        cached = list(h.cache.glob('*.json'))
        assert len(cached) == 1 and cached[0].name == digest(artifact) + '.json'
        h.record('content-addressed-artifact-cache', 'cache', {'artifact_digest':digest(artifact)})
        inv = invocation(loop,[buffer(0,[0]*4)],[1])
        altered = copy.deepcopy(artifact); altered['reflection']['local_size'] = [1,1,1]
        h.tamper('reflected-local-size-mismatch', altered, inv)
        altered = copy.deepcopy(artifact); altered['reflection']['entry'] = 'another_entry'
        h.tamper('reflected-entry-mismatch', altered, inv)
        altered = copy.deepcopy(artifact); altered['requirements'].append('vulkan.unknownFeature'); altered['requirements'].sort()
        h.tamper('unknown-required-feature', altered, inv)
        altered = copy.deepcopy(artifact); altered['evidence']['policy'] = 'refinement-verified/1'
        h.tamper('self-declared-refinement-policy', altered, inv)
        altered = copy.deepcopy(artifact); altered['words'][0] = 0
        altered['evidence']['output_digest'] = hashlib.sha256(b''.join(w.to_bytes(4,'little') for w in altered['words'])).hexdigest()
        h.tamper('rehash-does-not-admit-malformed-spirv', altered, inv)
        plan = {'schema':'kuiper.host-plan/1', 'parent_digest':digest(loop),
                'buffers':[{'name':'dependent','words':[99,0,0,0,0,88]},
                           {'name':'independent','words':[77,0,0,0,0,66]}], 'operations':[]}
        for identity, name, count, dependencies in ((1,'dependent',129,[]),(2,'dependent',3,[1]),(3,'independent',3,[])):
            plan['operations'].append({'id':identity,'entry':'pretested_loop','workgroups':[1,1,1],
              'parameters':[count],'bindings':[{'resource':0,'buffer':name,'offset':1,'length':4}],
              'dependencies':dependencies})
        pl = h.path('plan.json', plan)
        expected = h.cli('reference-plan', pp, pl)
        actual = h.cli('run-plan', pp, pl, h.workers, '--allow-experimental')
        assert [(s['id'],s['status']) for s in actual['steps']] == [(1,'failed'),(2,'failed'),(3,'succeeded')]
        assert actual['buffers'] == expected['buffers'] and actual['buffers'][0]['words'] is None
        c_actual = json.loads(h.command([h.driver,h.workers,pp,pl,'plan'], accepted=(4,)).stdout)
        assert c_actual['buffers'] == actual['buffers']
        h.record('host-dag-failure-poison-and-independent-publication', 'host-plan', {'interfaces':['core','c'], 'plan_digest':digest(plan)})
        plan['operations'][0]['parameters'] = [1]
        pl = h.path('plan.json', plan)
        actual = h.cli('run-plan', pp, pl, h.workers, '--allow-experimental')
        assert all(s['status'] == 'succeeded' for s in actual['steps'])
        assert actual['buffers'][0]['words'] == [99,3,3,3,3,88]
        assert json.loads(h.command([h.driver,h.workers,pp,pl,'plan']).stdout)['buffers'] == actual['buffers']
        h.record('host-dag-successful-sequenced-versions', 'host-plan', {'interfaces':['core','c'], 'plan_digest':digest(plan)})
        too_much = invocation(loop, [buffer(0,[0]*4)], [0], groups=262144)
        h.reject_source('aggregate-dispatch-work-rejected', loop, too_much, 'work-limit')
        nested = fixture('nested-loops')
        outer = next(x for x in nested['kernels'][0]['body']['instructions'] if x['kind']=='while')
        outer['iteration_limit'] = 1024
        next(x for x in outer['body']['instructions'] if x['kind']=='while')['iteration_limit'] = 1024
        h.reject_source('aggregate-nested-loop-work-rejected', nested,
                        invocation(nested,[buffer(0,[0]*4)],[0,0]), 'work-limit', check=True)
        bad = copy.deepcopy(inv); bad['buffers'][0]['offset'] = MASK
        h.reject_source('invalid-view-rejected-before-compilation', loop,bad,'view-bounds')
        if execution_inputs() != inputs_before:
            raise AssertionError('Measured execution inputs changed during replay')
        h.assert_host_inputs_unchanged()
        if h.driver_input and hashlib.sha256(h.driver_input.read_bytes()).hexdigest() != h.driver_input_digest:
            raise AssertionError('Selected hardware driver manifest changed during replay')
        report = {'schema':'kuiper.integer-integration/1', 'status':'passed',
            'profile':'kuiper.integer32/1', 'assurance':'kuiper.experimental-tested/1',
            'device_kind':('physical_gpu' if args.require_physical_gpu else
                           'cpu' if h.device_types and all(t == 'PHYSICAL_DEVICE_TYPE_CPU' for t in h.device_types)
                           else 'unclassified'),
            'device_types':h.device_types, 'hardware_only_driver_required':args.require_physical_gpu,
            'selected_driver_manifest':(str(h.driver_input) if h.driver_input else None),
            'selected_driver_manifest_sha256':h.driver_input_digest,
            'devices':sorted(h.devices), 'physical_gpu_qualification':False,
            'source_frontend_qualification':False, 'implementation_refinement':False,
            'vulkan_validation_layer':'VK_LAYER_KHRONOS_validation', 'validation_errors':0,
            'vulkaninfo_summary':h.device_info,
            'build_mode':h.mode,
            'elapsed_seconds':round(time.time()-started,2), 'case_count':len(h.cases), 'cases':h.cases,
            'sources':inputs_before,
            'workers':h.manifests, 'host_binaries':h.host_binaries,
            'validator':subprocess.check_output(['spirv-val','--version'],text=True),
            'validator_sha256':hashlib.sha256(Path('/usr/bin/spirv-val').read_bytes()).hexdigest()}
        output = Path(args.output); output.parent.mkdir(parents=True,exist_ok=True)
        output.write_text(json.dumps(report,indent=2)+'\n')
        print(f'{len(h.cases)} cases passed. Experimental integer profile only.',flush=True)


if __name__ == '__main__':
    main()
