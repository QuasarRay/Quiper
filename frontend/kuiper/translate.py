"""Translate the closed U32/ref subset of live, checked Pulse capture into KIR.

This is an experimental adapter, not an implementation-refinement proof.
Source predicates and the per-lane lifting relation remain separate obligations.
"""
from dataclasses import dataclass


class Unsupported(ValueError):
    pass


@dataclass(frozen=True)
class Value:
    category: str
    number: int | None = None


UNIT = Value('unit')
PROOF = Value('proof')


class Jump(Exception):
    def __init__(self, label, value):
        self.label, self.value = label, value


def symbol(node):
    return node.get('name') if node.get('tag') == 'symbol' else None


def application(node):
    arguments = []
    while node.get('tag') == 'apply':
        arguments = node['arguments'] + arguments
        node = node['head']
    return symbol(node), arguments


def scalar_type(node):
    return {'FStar.UInt32.t': 'u32', 'Prims.bool': 'bool', 'Prims.unit': 'unit'}.get(symbol(node))


def ref_type(node):
    return (node.get('tag') == 'apply'
            and symbol(node['head']) == 'Kuiper.Ref.gpu_ref'
            and len(node['arguments']) == 1
            and scalar_type(node['arguments'][0]['value']) == 'u32')


def erased_type(node):
    return (node.get('tag') == 'apply' and symbol(node['head']) == 'FStar.Ghost.erased'
            and len(node['arguments']) == 1
            and scalar_type(node['arguments'][0]['value']) in ('u32', 'bool', 'unit'))


def reject_bypasses(node):
    if isinstance(node, dict):
        if node.get('tag') == 'unsupported' and 'effect' in node:
            raise Unsupported('unsupported source construct: ' + str(node.get('constructor')))
        if node.get('tag') in ('admit', 'unreachable', 'pragma'):
            raise Unsupported('capture contains ' + node['tag'] + ': ' + str(node.get('constructor', '')))
        if node.get('tag') == 'symbol' and node.get('name') in (
                'Prims.admit', 'Prims.assume', 'Prims._assume', 'Prims.magic', 'Prims.unsafe_coerce',
                'FStar.Pervasives.admit',
                'FStar.Pervasives.assume', 'FStar.Pervasives.unsafe_coerce',
                'Pulse.Lib.Core.admit', 'Pulse.Lib.Core.assume', 'Pulse.Lib.Core.assume_',
                'Pulse.Lib.Core.stt_admit', 'Pulse.Lib.Core.stt_atomic_admit',
                'Pulse.Lib.Core.stt_ghost_admit'):
            raise Unsupported('capture contains an explicit proof bypass')
        for child in node.values():
            reject_bypasses(child)
    elif isinstance(node, list):
        for child in node:
            reject_bypasses(child)


class Translator:
    def __init__(self, declarations):
        self.declarations = declarations
        self.instructions = []
        self.mapping = []
        self.next_value = 0
        self.resource_uses = {}
        self.call_stack = []
        self.global_id = None
        self.next_label = 0

    def emit(self, kind, origin, **fields):
        if len(self.mapping) >= 16384:
            raise Unsupported('instruction budget exceeded')
        self.instructions.append({'kind': kind, **fields})
        self.mapping.append({'instruction': len(self.mapping), 'function': self.call_stack[-1],
                             'source_range': origin.get('range'), 'source': origin.get('source')})

    def result(self):
        n = self.next_value
        self.next_value += 1
        return n

    def constant(self, bits, ty, origin):
        n = self.result()
        self.emit('constant', origin, result={'id': n, 'ty': ty}, bits=bits)
        return Value(ty, n)

    @staticmethod
    def expect(value, category):
        if value.category != category:
            raise Unsupported(f'expected {category}, received {value.category}')
        return value.number

    def pure(self, node, env, origin):
        tag = node['tag']
        if tag == 'bound':
            index = node['index']
            if not isinstance(index, int) or not 0 <= index < len(env):
                raise Unsupported('free or out-of-range bound variable')
            return env[index]
        if tag == 'unit':
            return UNIT
        if tag == 'bool':
            return self.constant(int(node['value']), 'bool', origin)
        if tag == 'apply':
            fn, args = application(node)
            if fn in ('FStar.UInt32.uint_to_t', 'FStar.UInt32.__uint_to_t'):
                if len(args) != 1 or args[0]['implicit'] or args[0]['value']['tag'] != 'integer':
                    raise Unsupported('nonliteral U32 constructor')
                bits = args[0]['value']['value']
                if not isinstance(bits, int) or isinstance(bits, bool) or not 0 <= bits <= 0xffffffff:
                    raise Unsupported('U32 literal is outside its representation')
                return self.constant(bits, 'u32', origin)
            explicit = [self.pure(a['value'], env, origin) for a in args if not a['implicit']]
            return self.call(fn, args, explicit, origin)
        raise Unsupported('unsupported executable pure expression: ' + tag
                          + ' within ' + str(origin.get('value', {}))[:1800])

    def call(self, fn, raw_args, args, origin):
        binary = {'FStar.UInt32.add_mod': 'add', 'FStar.UInt32.sub_mod': 'sub',
                  'FStar.UInt32.mul_mod': 'mul', 'FStar.UInt32.eq': 'eq',
                  'FStar.UInt32.lt': 'lt', 'FStar.UInt32.lte': 'le'}
        if fn in binary:
            if len(args) != 2 or any(a['implicit'] for a in raw_args):
                raise Unsupported('unexpected integer primitive signature')
            left, right = [self.expect(a, 'u32') for a in args]
            n = self.result()
            self.emit('binary', origin, result=n, op=binary[fn], left=left, right=right)
            return Value('bool' if binary[fn] in ('eq', 'lt', 'le') else 'u32', n)
        if fn in ('Kuiper.Ref.read', 'Kuiper.Ref.write'):
            if origin.get('effect') == 'atomic':
                raise Unsupported('ordinary reference accesses cannot realize an atomic source step')
            implicit = [a['value'] for a in raw_args if a['implicit']]
            if not implicit or scalar_type(implicit[0]) != 'u32':
                raise Unsupported('reference primitive is not instantiated at U32')
            if len(args) != (1 if fn.endswith('.read') else 2):
                raise Unsupported('unexpected reference primitive signature')
            resource = self.expect(args[0], 'resource')
            uses = self.resource_uses[resource]
            if fn.endswith('.read'):
                uses.add('read')
                n = self.result()
                self.emit('load', origin, result=n, resource=resource, index=self.global_id)
                return Value('u32', n)
            uses.add('write')
            self.emit('store', origin, resource=resource, index=self.global_id,
                      value=self.expect(args[1], 'u32'))
            return UNIT
        if fn in self.declarations:
            if any(a['implicit'] for a in raw_args):
                raise Unsupported('implicit helper arguments require specialization')
            return self.function(fn, args)
        raise Unsupported('unsupported primitive or unresolved helper: ' + str(fn)
                          + ' ' + str(origin.get('effect'))
                          + ' ' + str(origin.get('function', {}))[:1500])

    def stateful(self, node, env):
        tag = node['tag']
        if node['effect'] == 'ghost':
            return PROOF
        if tag in ('introduce_pure', 'introduce_exists', 'eliminate_exists', 'rewrite'):
            return PROOF
        if node['effect'] == 'divergent':
            raise Unsupported('source computation is outside the U32/ref profile')
        if tag == 'return':
            inferred_return = (not node['source'] and node['type'].get('tag') == 'unsupported'
                               and node['type'].get('constructor') == 'Tm_unknown')
            if scalar_type(node['type']) not in ('unit', 'u32', 'bool') and not inferred_return:
                raise Unsupported('unsupported return type')
            value = self.pure(node['value'], env, node)
            if value == PROOF and scalar_type(node['type']) == 'unit':
                value = UNIT
            if not inferred_return and value.category != scalar_type(node['type']):
                raise Unsupported('return type disagrees with executable value')
            return value
        if tag in ('bind', 'pure_bind'):
            value = (self.stateful(node['head'], env) if tag == 'bind'
                     else self.pure(node['head'], env, node))
            ty = scalar_type(node['binder']['type'])
            if value not in (UNIT, PROOF) and ty != value.category:
                raise Unsupported('binding type disagrees with executable value')
            return self.stateful(node['body'], [value, *env])
        if tag == 'stateful_apply':
            call = node['function']
            if call['tag'] == 'apply':
                fn, raw = application(call)
            else:
                fn, raw = symbol(call), []
            values = [self.pure(a['value'], env, node) for a in raw if not a['implicit']]
            values += [self.stateful(a, env) for a in node['arguments']]
            return self.call(fn, raw, values, node)
        if tag == 'label':
            label = self.next_label
            self.next_label += 1
            try:
                value = self.stateful(node['body'], [Value('label', label), *env])
            except Jump as jump:
                if jump.label != label:
                    raise
                value = jump.value
            if value == PROOF and scalar_type(node['result_type']) == 'unit':
                value = UNIT
            if scalar_type(node['result_type']) != value.category:
                raise Unsupported('label result type disagrees with the returned value')
            return value
        if tag == 'jump':
            label = self.expect(self.pure(node['label'], env, node), 'label')
            raise Jump(label, self.pure(node['argument'], env, node))
        raise Unsupported('unsupported executable stateful expression: ' + tag)

    def function(self, fn, args):
        if fn in self.call_stack or len(self.call_stack) >= 8:
            raise Unsupported('recursive or deeply nested helper closure')
        self.call_stack.append(fn)
        try:
            reject_bypasses(self.declarations[fn])
            body = self.declarations[fn]['body']
            env = []
            remaining = list(args)
            while body['tag'] == 'abstract':
                if body['implicit'] and erased_type(body['binder']['type']):
                    env.insert(0, PROOF)
                    body = body['body']
                    continue
                if body['implicit'] or not remaining:
                    raise Unsupported('helper arity/qualifier mismatch')
                value = remaining.pop(0)
                ty = body['binder']['type']
                if not (ref_type(ty) and value.category == 'resource') and scalar_type(ty) != value.category:
                    raise Unsupported('helper parameter type mismatch')
                env.insert(0, value)
                body = body['body']
            if remaining:
                raise Unsupported('overapplied helper')
            return self.stateful(body, env)
        finally:
            self.call_stack.pop()

    def kernel(self, name, local_size):
        record = self.declarations[name]
        reject_bypasses(record)
        body = record['body']
        resources, parameters, args = [], [], []
        while body['tag'] == 'abstract':
            if body['implicit'] and erased_type(body['binder']['type']):
                body = body['body']
                continue
            if body['implicit'] or body['computation'] not in (None, 'stateful', 'total'):
                raise Unsupported('entrypoint needs static/effect specialization')
            ty = body['binder']['type']
            if ref_type(ty):
                resource = len(resources)
                self.resource_uses[resource] = set()
                resources.append({'id': resource, 'element': 'u32', 'access': 'read_write'})
                args.append(Value('resource', resource))
            elif scalar_type(ty) == 'u32':
                n = self.result()
                self.call_stack = [name]
                self.emit('parameter', body, result=n, index=len(parameters))
                self.call_stack = []
                parameters.append('u32')
                args.append(Value('u32', n))
            else:
                raise Unsupported('unsupported kernel parameter type')
            body = body['body']
        if not resources or len(resources) > 16 or not 1 <= local_size <= 256:
            raise Unsupported('invalid resource/local-size profile')
        self.global_id = self.result()
        self.call_stack = [name]
        self.emit('builtin', record['body'], result=self.global_id, builtin='global_id', axis=0)
        self.call_stack = []
        self.expect(self.function(name, args), 'unit')
        for resource in resources:
            uses = self.resource_uses[resource['id']]
            resource['access'] = 'read_write' if uses == {'read', 'write'} else ('write' if 'write' in uses else 'read')
        return {'schema': 'kuiper.kir/1', 'profile': 'kuiper.integer32/1', 'kernels': [
            {'name': name, 'local_size': [local_size, 1, 1], 'resources': resources,
             'parameters': parameters, 'body': {'instructions': self.instructions, 'outputs': []}}]}
