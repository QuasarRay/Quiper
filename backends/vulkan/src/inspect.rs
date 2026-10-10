//! Independent SPIR-V interface and operation-catalogue admission.
//!
//! This module uses Khronos enumerants, not the compiler's SPIR-T structures.
//! It does not establish dominance of bounds guards, loop budgets or race freedom.
use kuiper_contracts::{Access, Artifact, Diagnostic, MAX_WORDS, Result};
use spirv::{
    AddressingModel, BuiltIn, Capability, Decoration, ExecutionMode, ExecutionModel, MemoryModel,
    Op, Scope, StorageClass,
};
use std::collections::{BTreeMap, BTreeSet};

fn fail(code: &str, message: impl Into<String>) -> Diagnostic {
    Diagnostic::new("vulkan", code, message)
}
#[derive(Clone, Debug, PartialEq, Eq)]
enum Ty {
    Void,
    Bool,
    Int(bool),
    Vector(u32, u32),
    RuntimeArray(u32),
    Struct(Vec<u32>),
    Pointer(u32, u32),
    Function(u32),
}
#[derive(Default)]
struct Decorations {
    block: bool,
    stride: Option<u32>,
    non_writable: bool,
    set: Option<u32>,
    binding: Option<u32>,
    builtin: Option<u32>,
}
#[derive(Clone, Copy)]
struct Inst<'a> {
    op: Op,
    args: &'a [u32],
}
fn arity(args: &[u32], n: usize) -> Result<()> {
    if args.len() != n {
        return Err(fail(
            "spv-operands",
            "instruction has an unsupported operand count",
        ));
    }
    Ok(())
}
fn string_words(words: &[u32]) -> Result<(String, usize)> {
    let mut bytes = vec![];
    for (index, word) in words.iter().enumerate() {
        let chunk = word.to_le_bytes();
        if let Some(end) = chunk.iter().position(|byte| *byte == 0) {
            if chunk[end..].iter().any(|byte| *byte != 0) {
                return Err(fail("spv-string", "literal string has nonzero padding"));
            }
            bytes.extend(&chunk[..end]);
            let string = String::from_utf8(bytes)
                .map_err(|_| fail("spv-string", "literal string is not UTF-8"))?;
            return Ok((string, index + 1));
        }
        bytes.extend(chunk);
    }
    Err(fail("spv-string", "literal string is not terminated"))
}
fn scalar(types: &BTreeMap<u32, Ty>, id: u32) -> bool {
    matches!(types.get(&id), Some(Ty::Bool | Ty::Int(_)))
}
fn unsigned(types: &BTreeMap<u32, Ty>, id: u32) -> bool {
    types.get(&id) == Some(&Ty::Int(false))
}
fn declare(id: u32, bound: u32, definitions: &mut BTreeSet<u32>) -> Result<()> {
    if id == 0 || id >= bound || !definitions.insert(id) {
        return Err(fail(
            "spv-id",
            "result ID is zero, out of range or duplicated",
        ));
    }
    Ok(())
}
fn put_once(slot: &mut Option<u32>, value: u32) -> Result<()> {
    if slot.replace(value).is_some() {
        return Err(fail("spv-decoration", "duplicate decoration"));
    }
    Ok(())
}
fn flag_once(slot: &mut bool) -> Result<()> {
    if *slot {
        return Err(fail("spv-decoration", "duplicate decoration"));
    }
    *slot = true;
    Ok(())
}

/// Reject unsupported executable/layout forms and match the final module to its ABI.
pub fn check(artifact: &Artifact) -> Result<()> {
    kuiper_contracts::validate::reflection(&artifact.reflection)?;
    let words = &artifact.words;
    if words.len() < 5
        || words.len() > MAX_WORDS
        || words[0] != 0x0723_0203
        || words[1] != 0x0001_0500
        || words[3] == 0
        || words[3] as usize > MAX_WORDS
        || words[4] != 0
    {
        return Err(fail(
            "spv-header",
            "expected bounded SPIR-V 1.5 with the standard word encoding",
        ));
    }
    let mut instructions = vec![];
    let mut position = 5;
    while position < words.len() {
        let header = words[position];
        let length = (header >> 16) as usize;
        if length == 0 || length > words.len() - position {
            return Err(fail(
                "spv-instruction",
                "truncated or zero-length instruction",
            ));
        }
        let op = Op::from_u32(header & 0xffff)
            .ok_or_else(|| fail("spv-opcode", "unknown SPIR-V opcode"))?;
        instructions.push(Inst {
            op,
            args: &words[position + 1..position + length],
        });
        position += length;
    }
    let mut types = BTreeMap::new();
    let mut value_types = BTreeMap::new();
    let mut definitions = BTreeSet::new();
    let mut constants = BTreeMap::new();
    let mut variables = BTreeMap::new();
    let mut decorations = BTreeMap::<u32, Decorations>::new();
    let mut offsets = BTreeMap::new();
    let mut capabilities = BTreeSet::new();
    let mut memory_model = None;
    let mut entry = None;
    let mut local = None;
    let mut functions = vec![];
    for inst in &instructions {
        let a = inst.args;
        match inst.op {
            Op::Capability => {
                arity(a, 1)?;
                if !capabilities.insert(a[0]) {
                    return Err(fail("spv-capability", "duplicate capability"));
                }
            }
            Op::MemoryModel => {
                arity(a, 2)?;
                if memory_model.replace((a[0], a[1])).is_some() {
                    return Err(fail("spv-memory-model", "duplicate memory model"));
                }
            }
            Op::EntryPoint => {
                if a.len() < 3 || entry.is_some() || a[0] != ExecutionModel::GLCompute as u32 {
                    return Err(fail("spv-entry", "expected one GLCompute entrypoint"));
                }
                let (name, count) = string_words(&a[2..])?;
                let interface = &a[2 + count..];
                let ids: BTreeSet<_> = interface.iter().copied().collect();
                if ids.len() != interface.len() {
                    return Err(fail("spv-entry", "entry interface repeats a global"));
                }
                entry = Some((a[1], name, ids));
            }
            Op::ExecutionMode => {
                arity(a, 5)?;
                if a[1] != ExecutionMode::LocalSize as u32
                    || local.replace((a[0], [a[2], a[3], a[4]])).is_some()
                {
                    return Err(fail(
                        "spv-local-size",
                        "expected one literal LocalSize mode",
                    ));
                }
            }
            Op::ModuleProcessed => {
                let (_, count) = string_words(a)?;
                if count != a.len() {
                    return Err(fail("spv-string", "trailing module-process operands"));
                }
            }
            Op::Decorate => {
                if a.len() < 2 {
                    return Err(fail("spv-decoration", "short decoration"));
                }
                let d = decorations.entry(a[0]).or_default();
                match Decoration::from_u32(a[1]) {
                    Some(Decoration::Block) => {
                        arity(a, 2)?;
                        flag_once(&mut d.block)?;
                    }
                    Some(Decoration::NonWritable) => {
                        arity(a, 2)?;
                        flag_once(&mut d.non_writable)?;
                    }
                    Some(Decoration::ArrayStride) => {
                        arity(a, 3)?;
                        put_once(&mut d.stride, a[2])?;
                    }
                    Some(Decoration::DescriptorSet) => {
                        arity(a, 3)?;
                        put_once(&mut d.set, a[2])?;
                    }
                    Some(Decoration::Binding) => {
                        arity(a, 3)?;
                        put_once(&mut d.binding, a[2])?;
                    }
                    Some(Decoration::BuiltIn) => {
                        arity(a, 3)?;
                        put_once(&mut d.builtin, a[2])?;
                    }
                    _ => {
                        return Err(fail(
                            "spv-decoration",
                            "decoration is outside the integer baseline",
                        ));
                    }
                }
            }
            Op::MemberDecorate => {
                arity(a, 4)?;
                if a[1] != 0
                    || a[2] != Decoration::Offset as u32
                    || a[3] != 0
                    || offsets.insert((a[0], a[1]), a[3]).is_some()
                {
                    return Err(fail(
                        "spv-layout",
                        "only a unique member-0 offset-0 decoration is supported",
                    ));
                }
            }
            Op::TypeVoid
            | Op::TypeBool
            | Op::TypeInt
            | Op::TypeVector
            | Op::TypeRuntimeArray
            | Op::TypeStruct
            | Op::TypePointer
            | Op::TypeFunction => {
                if a.is_empty() {
                    return Err(fail("spv-type", "missing type ID"));
                }
                declare(a[0], words[3], &mut definitions)?;
                let ty = match inst.op {
                    Op::TypeVoid => {
                        arity(a, 1)?;
                        Ty::Void
                    }
                    Op::TypeBool => {
                        arity(a, 1)?;
                        Ty::Bool
                    }
                    Op::TypeInt => {
                        arity(a, 3)?;
                        if a[1] != 32 || a[2] > 1 {
                            return Err(fail(
                                "spv-type",
                                "only 32-bit integer types are supported",
                            ));
                        }
                        Ty::Int(a[2] != 0)
                    }
                    Op::TypeVector => {
                        arity(a, 3)?;
                        if a[2] != 3 {
                            return Err(fail(
                                "spv-type",
                                "only three-component builtin vectors are supported",
                            ));
                        }
                        Ty::Vector(a[1], a[2])
                    }
                    Op::TypeRuntimeArray => {
                        arity(a, 2)?;
                        Ty::RuntimeArray(a[1])
                    }
                    Op::TypeStruct => {
                        arity(a, 2)?;
                        Ty::Struct(a[1..].to_vec())
                    }
                    Op::TypePointer => {
                        arity(a, 3)?;
                        if ![
                            StorageClass::StorageBuffer as u32,
                            StorageClass::Input as u32,
                        ]
                        .contains(&a[1])
                        {
                            return Err(fail("spv-storage", "unsupported pointer storage class"));
                        }
                        Ty::Pointer(a[1], a[2])
                    }
                    Op::TypeFunction => {
                        arity(a, 2)?;
                        Ty::Function(a[1])
                    }
                    _ => unreachable!(),
                };
                types.insert(a[0], ty);
            }
            Op::Constant | Op::ConstantTrue | Op::ConstantFalse => {
                arity(a, if inst.op == Op::Constant { 3 } else { 2 })?;
                declare(a[1], words[3], &mut definitions)?;
                value_types.insert(a[1], a[0]);
                constants.insert(
                    a[1],
                    (
                        a[0],
                        if inst.op == Op::Constant {
                            a[2]
                        } else {
                            u32::from(inst.op == Op::ConstantTrue)
                        },
                    ),
                );
            }
            Op::Variable => {
                arity(a, 3)?;
                declare(a[1], words[3], &mut definitions)?;
                value_types.insert(a[1], a[0]);
                variables.insert(a[1], (a[0], a[2]));
            }
            Op::Function => {
                arity(a, 4)?;
                declare(a[1], words[3], &mut definitions)?;
                if a[2] != 0 {
                    return Err(fail("spv-function", "unsupported function control"));
                }
                functions.push((a[1], a[0], a[3]));
            }
            Op::FunctionEnd | Op::Return => arity(a, 0)?,
            Op::Label => {
                arity(a, 1)?;
                declare(a[0], words[3], &mut definitions)?;
            }
            Op::Branch => arity(a, 1)?,
            Op::BranchConditional => arity(a, 3)?,
            Op::SelectionMerge => {
                arity(a, 2)?;
                if a[1] != 0 {
                    return Err(fail("spv-control", "unsupported selection control"));
                }
            }
            Op::LoopMerge => {
                arity(a, 3)?;
                if a[2] != 0 {
                    return Err(fail("spv-control", "unsupported loop control"));
                }
            }
            Op::Store => arity(a, 2)?,
            Op::Load
            | Op::AccessChain
            | Op::CompositeExtract
            | Op::CopyObject
            | Op::Bitcast
            | Op::Select
            | Op::Phi
            | Op::AtomicIAdd
            | Op::AtomicOr
            | Op::IAdd
            | Op::ISub
            | Op::IMul
            | Op::UDiv
            | Op::SDiv
            | Op::UMod
            | Op::SRem
            | Op::BitwiseAnd
            | Op::BitwiseOr
            | Op::BitwiseXor
            | Op::ShiftLeftLogical
            | Op::ShiftRightLogical
            | Op::ShiftRightArithmetic
            | Op::IEqual
            | Op::INotEqual
            | Op::ULessThan
            | Op::ULessThanEqual
            | Op::UGreaterThan
            | Op::UGreaterThanEqual
            | Op::SLessThan
            | Op::SLessThanEqual
            | Op::SGreaterThan
            | Op::SGreaterThanEqual
            | Op::LogicalEqual
            | Op::LogicalNotEqual
            | Op::LogicalAnd
            | Op::LogicalOr
            | Op::LogicalNot => {
                let length = match inst.op {
                    Op::Load | Op::CopyObject | Op::Bitcast | Op::LogicalNot => 3,
                    Op::AccessChain | Op::Select => 5,
                    Op::CompositeExtract => 4,
                    Op::AtomicIAdd | Op::AtomicOr => 6,
                    Op::Phi => {
                        if a.len() < 4 || a.len() % 2 != 0 {
                            return Err(fail("spv-phi", "unsupported phi operands"));
                        }
                        a.len()
                    }
                    _ => 4,
                };
                arity(a, length)?;
                declare(a[1], words[3], &mut definitions)?;
                value_types.insert(a[1], a[0]);
            }
            _ => {
                return Err(fail(
                    "spv-opcode",
                    format!("opcode {:?} is outside the integer catalogue", inst.op),
                ));
            }
        }
    }
    if capabilities
        != BTreeSet::from([
            Capability::Shader as u32,
            Capability::VulkanMemoryModel as u32,
            Capability::VulkanMemoryModelDeviceScope as u32,
        ])
    {
        return Err(fail(
            "spv-capability",
            "capability set differs from the admitted integer profile",
        ));
    }
    if memory_model != Some((AddressingModel::Logical as u32, MemoryModel::Vulkan as u32)) {
        return Err(fail(
            "spv-memory-model",
            "expected the Logical Vulkan memory model",
        ));
    }
    let (entry_id, name, interface) =
        entry.ok_or_else(|| fail("spv-entry", "missing entrypoint"))?;
    if name != artifact.reflection.entry
        || local != Some((entry_id, artifact.reflection.local_size))
    {
        return Err(fail(
            "spv-entry",
            "entry name or local size differs from reflection",
        ));
    }
    if functions.len() != 1
        || functions[0].0 != entry_id
        || types.get(&functions[0].1) != Some(&Ty::Void)
        || types.get(&functions[0].2) != Some(&Ty::Function(functions[0].1))
    {
        return Err(fail(
            "spv-function",
            "expected a single void entrypoint without parameters",
        ));
    }
    if interface != variables.keys().copied().collect() {
        return Err(fail(
            "spv-interface",
            "entry interface must contain exactly every global",
        ));
    }
    for (id, ty) in &types {
        match ty {
            Ty::Vector(component, 3) if unsigned(&types, *component) => (),
            Ty::RuntimeArray(element)
                if unsigned(&types, *element)
                    && decorations.get(id).and_then(|d| d.stride) == Some(4) => {}
            Ty::Struct(members)
                if members.len() == 1
                    && matches!(types.get(&members[0]), Some(Ty::RuntimeArray(_)))
                    && decorations.get(id).is_some_and(|d| d.block)
                    && offsets.get(&(*id, 0)) == Some(&0) => {}
            Ty::Pointer(_, target) if types.contains_key(target) => (),
            Ty::Function(ret) if types.get(ret) == Some(&Ty::Void) => (),
            Ty::Void | Ty::Bool | Ty::Int(_) => (),
            _ => {
                return Err(fail(
                    "spv-layout",
                    "type graph does not match the admitted scalar and word-buffer layout",
                ));
            }
        }
    }
    for (id, d) in &decorations {
        let allowed = match types.get(id) {
            Some(Ty::RuntimeArray(_)) => {
                d.stride == Some(4)
                    && !d.block
                    && !d.non_writable
                    && d.set.is_none()
                    && d.binding.is_none()
                    && d.builtin.is_none()
            }
            Some(Ty::Struct(_)) => {
                d.block
                    && d.stride.is_none()
                    && !d.non_writable
                    && d.set.is_none()
                    && d.binding.is_none()
                    && d.builtin.is_none()
            }
            _ => variables.contains_key(id) && !d.block && d.stride.is_none(),
        };
        if !allowed {
            return Err(fail(
                "spv-decoration",
                "decoration has an unsupported target or combination",
            ));
        }
    }
    if offsets
        .keys()
        .any(|(id, member)| *member != 0 || !matches!(types.get(id), Some(Ty::Struct(_))))
    {
        return Err(fail(
            "spv-layout",
            "member offset targets an undeclared block",
        ));
    }
    for (type_id, bits) in constants.values() {
        if !scalar(&types, *type_id) || (types.get(type_id) == Some(&Ty::Bool) && *bits > 1) {
            return Err(fail(
                "spv-constant",
                "unsupported constant type or Boolean encoding",
            ));
        }
    }
    let mut descriptor_variables = BTreeMap::new();
    let mut builtin_variables = BTreeMap::new();
    for (id, (pointer_type, storage)) in &variables {
        let Some(Ty::Pointer(pointer_storage, target)) = types.get(pointer_type) else {
            return Err(fail(
                "spv-variable",
                "global variable type is not a pointer",
            ));
        };
        if pointer_storage != storage {
            return Err(fail(
                "spv-storage",
                "global variable and pointer storage differ",
            ));
        }
        let d = decorations
            .get(id)
            .ok_or_else(|| fail("spv-variable", "global variable lacks required decorations"))?;
        if *storage == StorageClass::StorageBuffer as u32 {
            if !matches!(types.get(target), Some(Ty::Struct(_)))
                || d.set != Some(0)
                || d.builtin.is_some()
            {
                return Err(fail(
                    "spv-binding",
                    "storage buffer must have the admitted block in descriptor set 0",
                ));
            }
            let binding = d
                .binding
                .ok_or_else(|| fail("spv-binding", "storage buffer lacks its binding"))?;
            let read_only = if binding < artifact.reflection.parameter_binding {
                let reflected = artifact
                    .reflection
                    .bindings
                    .get(binding as usize)
                    .ok_or_else(|| {
                        fail("spv-binding", "resource binding is absent from reflection")
                    })?;
                reflected.access == Access::Read
            } else if binding == artifact.reflection.parameter_binding {
                true
            } else if binding == artifact.reflection.guard_binding {
                false
            } else {
                return Err(fail(
                    "spv-binding",
                    "storage binding exceeds the declared interface",
                ));
            };
            if d.non_writable != read_only || descriptor_variables.insert(binding, *id).is_some() {
                return Err(fail(
                    "spv-binding",
                    "resource permissions or uniqueness differ from reflection",
                ));
            }
        } else if *storage == StorageClass::Input as u32 {
            if !matches!(types.get(target),Some(Ty::Vector(component,3)) if unsigned(&types,*component))
                || d.binding.is_some()
                || d.set.is_some()
                || d.non_writable
            {
                return Err(fail(
                    "spv-builtin",
                    "builtin must be an undecorated descriptor-free U32 vector input",
                ));
            }
            let builtin = d
                .builtin
                .ok_or_else(|| fail("spv-builtin", "input variable lacks BuiltIn"))?;
            if ![
                BuiltIn::GlobalInvocationId as u32,
                BuiltIn::LocalInvocationId as u32,
                BuiltIn::WorkgroupId as u32,
                BuiltIn::NumWorkgroups as u32,
            ]
            .contains(&builtin)
                || builtin_variables.insert(builtin, *id).is_some()
            {
                return Err(fail("spv-builtin", "unsupported or duplicate builtin"));
            }
        } else {
            return Err(fail("spv-storage", "unsupported global storage class"));
        }
    }
    if descriptor_variables.len() != artifact.reflection.bindings.len() + 2
        || descriptor_variables
            .keys()
            .copied()
            .ne(0..artifact.reflection.guard_binding + 1)
        || builtin_variables.len() != 4
    {
        return Err(fail(
            "spv-interface",
            "descriptor or builtin interface is incomplete",
        ));
    }
    let descriptor_bindings: BTreeMap<_, _> = descriptor_variables
        .into_iter()
        .map(|(binding, id)| (id, binding))
        .collect();
    let mut pointers = BTreeMap::new();
    for inst in &instructions {
        let a = inst.args;
        if inst.op != Op::AccessChain {
            continue;
        }
        let binding = *descriptor_bindings.get(&a[2]).ok_or_else(|| {
            fail(
                "spv-pointer",
                "access chain must start at its declared descriptor",
            )
        })?;
        if !matches!(types.get(&a[0]),Some(Ty::Pointer(storage,target)) if *storage==StorageClass::StorageBuffer as u32 && unsigned(&types,*target))
            || !constants
                .get(&a[3])
                .is_some_and(|(ty, bits)| unsigned(&types, *ty) && *bits == 0)
            || !value_types
                .get(&a[4])
                .is_some_and(|ty| unsigned(&types, *ty))
        {
            return Err(fail(
                "spv-pointer",
                "access chain type or indices do not match the word ABI",
            ));
        }
        if binding == artifact.reflection.parameter_binding {
            let length = (artifact.reflection.bindings.len() * 2
                + artifact.reflection.parameter_types.len())
            .max(1) as u32;
            if !constants
                .get(&a[4])
                .is_some_and(|(ty, bits)| unsigned(&types, *ty) && *bits < length)
            {
                return Err(fail(
                    "spv-pointer",
                    "parameter access must use an admitted literal index",
                ));
            }
        }
        if binding == artifact.reflection.guard_binding
            && !constants
                .get(&a[4])
                .is_some_and(|(ty, bits)| unsigned(&types, *ty) && *bits == 0)
        {
            return Err(fail("spv-pointer", "guard access must address word zero"));
        }
        pointers.insert(a[1], binding);
    }
    for inst in &instructions {
        let a = inst.args;
        match inst.op {
            Op::Load => {
                if let Some(binding)=pointers.get(&a[2]) {
                    if !unsigned(&types,a[0]) || *binding==artifact.reflection.guard_binding || (*binding<artifact.reflection.parameter_binding && !matches!(artifact.reflection.bindings[*binding as usize].access,Access::Read|Access::ReadWrite)){return Err(fail("spv-access","ordinary load violates its resource access mode"));}
                } else if !builtin_variables.values().any(|id|*id==a[2]) || !matches!(types.get(&a[0]),Some(Ty::Vector(component,3)) if unsigned(&types,*component)) {
                    return Err(fail("spv-access","load must address a word-buffer chain or admitted builtin"));
                }
            }
            Op::Store => {
                let binding=*pointers.get(&a[0]).ok_or_else(||fail("spv-access","store pointer is not an admitted word-buffer chain"))?;
                if binding>=artifact.reflection.parameter_binding || !matches!(artifact.reflection.bindings[binding as usize].access,Access::Write|Access::ReadWrite) || !value_types.get(&a[1]).is_some_and(|ty|unsigned(&types,*ty)){return Err(fail("spv-access","ordinary store violates its resource access mode or word type"));}
            }
            Op::AtomicIAdd|Op::AtomicOr => {
                let binding=*pointers.get(&a[2]).ok_or_else(||fail("spv-atomic","atomic pointer is not an admitted word-buffer chain"))?;
                let scope=constants.get(&a[3]);let semantics=constants.get(&a[4]);
                if !unsigned(&types,a[0]) || !scope.is_some_and(|(ty,bits)|unsigned(&types,*ty)&&*bits==Scope::Device as u32) || !semantics.is_some_and(|(ty,bits)|unsigned(&types,*ty)&&*bits==0) || !value_types.get(&a[5]).is_some_and(|ty|unsigned(&types,*ty)){return Err(fail("spv-atomic","atomic operation must use U32, Device scope and Relaxed semantics"));}
                if inst.op==Op::AtomicIAdd {
                    if binding>=artifact.reflection.parameter_binding || artifact.reflection.bindings[binding as usize].access!=Access::Atomic{return Err(fail("spv-atomic","atomic add requires its declared atomic resource"));}
                } else if binding!=artifact.reflection.guard_binding || !constants.get(&a[5]).is_some_and(|(ty,bits)|unsigned(&types,*ty)&&bits.is_power_of_two()){return Err(fail("spv-atomic","atomic OR is reserved for single-bit guard failure recording"));}
            }
            Op::CompositeExtract => {if !unsigned(&types,a[0]) || a[3]>2 || !value_types.get(&a[2]).is_some_and(|ty|matches!(types.get(ty),Some(Ty::Vector(component,3)) if unsigned(&types,*component))){return Err(fail("spv-value","unsupported builtin extraction"));}},
            Op::CopyObject|Op::Bitcast|Op::Select|Op::Phi|Op::IAdd|Op::ISub|Op::IMul|Op::UDiv|Op::SDiv|Op::UMod|Op::SRem|Op::BitwiseAnd|Op::BitwiseOr|Op::BitwiseXor|Op::ShiftLeftLogical|Op::ShiftRightLogical|Op::ShiftRightArithmetic|Op::IEqual|Op::INotEqual|Op::ULessThan|Op::ULessThanEqual|Op::UGreaterThan|Op::UGreaterThanEqual|Op::SLessThan|Op::SLessThanEqual|Op::SGreaterThan|Op::SGreaterThanEqual|Op::LogicalEqual|Op::LogicalNotEqual|Op::LogicalAnd|Op::LogicalOr|Op::LogicalNot => {
                if !scalar(&types,a[0]) {return Err(fail("spv-value","pointer or aggregate scalar operations are outside the integer catalogue"));}
            }
            _=>(),
        }
    }
    Ok(())
}
