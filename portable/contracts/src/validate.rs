//! Conservative checker for the integer baseline. These checks are not a refinement proof.
use crate::*;
use std::collections::{BTreeMap, BTreeSet};

fn error(code: &str, message: impl Into<String>) -> Diagnostic {
    Diagnostic::new("checker", code, message)
}
pub fn name(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 128
        && s.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"_.-/+>=:".contains(&b))
}
fn local_size(size: [u32; 3]) -> Result<()> {
    if size[0] == 0 || size[0] > 256 || size[1] != 1 || size[2] != 1 {
        return Err(error(
            "local-size",
            "integer baseline requires a one-dimensional local size in 1..=256",
        ));
    }
    Ok(())
}

#[derive(Copy, Clone)]
struct Fact {
    ty: Scalar,
    global_x: bool,
}
type Env = BTreeMap<ValueId, Fact>;
struct Checker<'a> {
    resources: BTreeMap<u32, &'a Resource>,
    parameters: &'a [Scalar],
    declarations: BTreeSet<ValueId>,
    nodes: usize,
}
impl Checker<'_> {
    fn get(&self, env: &Env, id: ValueId) -> Result<Fact> {
        env.get(&id).copied().ok_or_else(|| {
            error(
                "unbound-value",
                format!("value {id} does not dominate its use"),
            )
        })
    }
    fn typed(&self, env: &Env, id: ValueId, ty: Scalar) -> Result<Fact> {
        let fact = self.get(env, id)?;
        if fact.ty != ty {
            return Err(error(
                "type-mismatch",
                format!("value {id} has the wrong scalar type"),
            ));
        }
        Ok(fact)
    }
    fn declare(&mut self, env: &mut Env, id: ValueId, fact: Fact) -> Result<()> {
        if !self.declarations.insert(id) {
            return Err(error(
                "duplicate-value",
                format!("SSA value {id} is declared twice"),
            ));
        }
        env.insert(id, fact);
        Ok(())
    }
    fn resource(&self, id: u32) -> Result<&Resource> {
        self.resources
            .get(&id)
            .copied()
            .ok_or_else(|| error("missing-resource", format!("resource {id} is not declared")))
    }
    fn region(&mut self, region: &Region, env: &mut Env, depth: usize) -> Result<Vec<Fact>> {
        if depth > MAX_DEPTH {
            return Err(error("region-depth", "KIR exceeds the region depth limit"));
        }
        for instruction in &region.instructions {
            self.nodes += 1;
            if self.nodes > MAX_NODES {
                return Err(error("node-limit", "KIR exceeds the instruction limit"));
            }
            match instruction {
                Instruction::Constant { result, bits } => {
                    if result.ty == Scalar::Bool && *bits > 1 {
                        return Err(error("boolean-bits", "Boolean constants must be 0 or 1"));
                    }
                    self.declare(
                        env,
                        result.id,
                        Fact {
                            ty: result.ty,
                            global_x: false,
                        },
                    )?;
                }
                Instruction::Builtin {
                    result,
                    builtin,
                    axis,
                } => {
                    if *axis > 2 {
                        return Err(error("builtin-axis", "builtin axis must be 0, 1 or 2"));
                    }
                    self.declare(
                        env,
                        *result,
                        Fact {
                            ty: Scalar::U32,
                            global_x: *builtin == Builtin::GlobalId && *axis == 0,
                        },
                    )?;
                }
                Instruction::Parameter { result, index } => {
                    let ty = self
                        .parameters
                        .get(*index as usize)
                        .copied()
                        .ok_or_else(|| {
                            error("parameter-index", "parameter index is out of range")
                        })?;
                    self.declare(
                        env,
                        *result,
                        Fact {
                            ty,
                            global_x: false,
                        },
                    )?;
                }
                Instruction::ResourceLength { result, resource } => {
                    self.resource(*resource)?;
                    self.declare(
                        env,
                        *result,
                        Fact {
                            ty: Scalar::U32,
                            global_x: false,
                        },
                    )?;
                }
                Instruction::Binary {
                    result,
                    op,
                    left,
                    right,
                } => {
                    let a = self.get(env, *left)?;
                    let b = self.get(env, *right)?;
                    let ty = match op {
                        Binary::LogicalAnd | Binary::LogicalOr => {
                            if a.ty != Scalar::Bool || b.ty != Scalar::Bool {
                                return Err(error(
                                    "type-mismatch",
                                    "logical operands must be Boolean",
                                ));
                            }
                            Scalar::Bool
                        }
                        Binary::Eq | Binary::Ne => {
                            if a.ty != b.ty {
                                return Err(error(
                                    "type-mismatch",
                                    "equality operand types differ",
                                ));
                            }
                            Scalar::Bool
                        }
                        Binary::Lt | Binary::Le | Binary::Gt | Binary::Ge => {
                            if a.ty == Scalar::Bool || a.ty != b.ty {
                                return Err(error(
                                    "type-mismatch",
                                    "ordered operands must have one integer type",
                                ));
                            }
                            Scalar::Bool
                        }
                        Binary::ShiftLeft | Binary::ShiftRight => {
                            if a.ty == Scalar::Bool || b.ty != Scalar::U32 {
                                return Err(error(
                                    "type-mismatch",
                                    "shift requires an integer and an unsigned count",
                                ));
                            }
                            a.ty
                        }
                        _ => {
                            if a.ty == Scalar::Bool || a.ty != b.ty {
                                return Err(error(
                                    "type-mismatch",
                                    "arithmetic operands must have one integer type",
                                ));
                            }
                            a.ty
                        }
                    };
                    self.declare(
                        env,
                        *result,
                        Fact {
                            ty,
                            global_x: false,
                        },
                    )?;
                }
                Instruction::Load {
                    result,
                    resource,
                    index,
                } => {
                    let r = self.resource(*resource)?;
                    let index = self.typed(env, *index, Scalar::U32)?;
                    match r.access {
                        Access::Read => (),
                        Access::ReadWrite if index.global_x => (),
                        Access::ReadWrite => {
                            return Err(error(
                                "read-race",
                                "read/write resource reads require the checked GlobalId.x index",
                            ));
                        }
                        _ => {
                            return Err(error(
                                "access-mode",
                                "ordinary loads require read or read/write access",
                            ));
                        }
                    }
                    self.declare(
                        env,
                        *result,
                        Fact {
                            ty: r.element,
                            global_x: false,
                        },
                    )?;
                }
                Instruction::Store {
                    resource,
                    index,
                    value,
                } => {
                    let r = self.resource(*resource)?;
                    if !matches!(r.access, Access::Write | Access::ReadWrite) {
                        return Err(error(
                            "access-mode",
                            "ordinary stores require write or read/write access",
                        ));
                    }
                    if !self.typed(env, *index, Scalar::U32)?.global_x {
                        return Err(error(
                            "write-race",
                            "ordinary writes require the checked GlobalId.x index",
                        ));
                    }
                    self.typed(env, *value, r.element)?;
                }
                Instruction::AtomicAdd {
                    result,
                    resource,
                    index,
                    value,
                } => {
                    let r = self.resource(*resource)?;
                    if r.access != Access::Atomic || r.element != Scalar::U32 {
                        return Err(error(
                            "access-mode",
                            "atomic add requires an atomic u32 resource",
                        ));
                    }
                    self.typed(env, *index, Scalar::U32)?;
                    self.typed(env, *value, Scalar::U32)?;
                    self.declare(
                        env,
                        *result,
                        Fact {
                            ty: Scalar::U32,
                            global_x: false,
                        },
                    )?;
                }
                Instruction::Guard { condition, code } => {
                    self.typed(env, *condition, Scalar::Bool)?;
                    if *code < 8 || !code.is_power_of_two() {
                        return Err(error(
                            "guard-code",
                            "explicit guard codes must be a single bit >= 8",
                        ));
                    }
                }
                Instruction::Select {
                    condition,
                    then_region,
                    else_region,
                    results,
                } => {
                    self.typed(env, *condition, Scalar::Bool)?;
                    let yes = self.region(then_region, &mut env.clone(), depth + 1)?;
                    let no = self.region(else_region, &mut env.clone(), depth + 1)?;
                    if yes.len() != results.len() || no.len() != results.len() {
                        return Err(error(
                            "region-arity",
                            "selection branches must return the declared result tuple",
                        ));
                    }
                    for ((decl, a), b) in results.iter().zip(yes).zip(no) {
                        if a.ty != decl.ty || b.ty != decl.ty {
                            return Err(error("type-mismatch", "selection result types differ"));
                        }
                        self.declare(
                            env,
                            decl.id,
                            Fact {
                                ty: decl.ty,
                                global_x: a.global_x && b.global_x,
                            },
                        )?;
                    }
                }
                Instruction::While {
                    carried,
                    condition,
                    body,
                    results,
                    iteration_limit,
                } => {
                    if *iteration_limit == 0 || *iteration_limit > MAX_WORDS as u32 {
                        return Err(error(
                            "iteration-limit",
                            "loop iteration limit must be in 1..=1048576",
                        ));
                    }
                    if carried.len() != results.len() {
                        return Err(error(
                            "region-arity",
                            "loop carried and result tuple arities differ",
                        ));
                    }
                    let mut loop_env = env.clone();
                    for param in carried {
                        self.typed(env, param.initial, param.value.ty)?;
                        self.declare(
                            &mut loop_env,
                            param.value.id,
                            Fact {
                                ty: param.value.ty,
                                global_x: false,
                            },
                        )?;
                    }
                    let test = self.region(condition, &mut loop_env.clone(), depth + 1)?;
                    if test.len() != 1 || test[0].ty != Scalar::Bool {
                        return Err(error(
                            "loop-condition",
                            "loop condition must return one Boolean",
                        ));
                    }
                    let outputs = self.region(body, &mut loop_env, depth + 1)?;
                    if outputs.len() != carried.len() {
                        return Err(error(
                            "region-arity",
                            "loop body must return its carried tuple",
                        ));
                    }
                    for ((param, decl), output) in carried.iter().zip(results).zip(outputs) {
                        if param.value.ty != decl.ty || output.ty != decl.ty {
                            return Err(error("type-mismatch", "loop carried/result types differ"));
                        }
                        self.declare(
                            env,
                            decl.id,
                            Fact {
                                ty: decl.ty,
                                global_x: false,
                            },
                        )?;
                    }
                }
            }
        }
        region.outputs.iter().map(|id| self.get(env, *id)).collect()
    }
}

pub fn package(package: &Package) -> Result<()> {
    if package.schema != KIR_SCHEMA || package.profile != INTEGER_PROFILE {
        return Err(error(
            "unsupported-contract",
            "unsupported KIR schema or profile",
        ));
    }
    if package.kernels.is_empty() || package.kernels.len() > 128 {
        return Err(error("kernel-count", "package requires 1..=128 kernels"));
    }
    let mut names = BTreeSet::new();
    for kernel in &package.kernels {
        if !name(&kernel.name) || !names.insert(&kernel.name) {
            return Err(error(
                "kernel-name",
                "kernel names must be valid and unique",
            ));
        }
        local_size(kernel.local_size)?;
        if kernel.resources.len() > MAX_RESOURCES || kernel.parameters.len() > 32 {
            return Err(error(
                "interface-limit",
                "kernel interface exceeds the baseline limit",
            ));
        }
        let mut resources = BTreeMap::new();
        for resource in &kernel.resources {
            if resources.insert(resource.id, resource).is_some() {
                return Err(error("duplicate-resource", "resource IDs must be unique"));
            }
            if resource.access == Access::Atomic && resource.element != Scalar::U32 {
                return Err(error(
                    "access-mode",
                    "atomic resources require u32 elements",
                ));
            }
        }
        let mut checker = Checker {
            resources,
            parameters: &kernel.parameters,
            declarations: BTreeSet::new(),
            nodes: 0,
        };
        let outputs = checker.region(&kernel.body, &mut Env::new(), 0)?;
        if !outputs.is_empty() {
            return Err(error(
                "kernel-outputs",
                "kernel body must return no SSA values",
            ));
        }
    }
    Ok(())
}
pub fn kernel<'a>(package_value: &'a Package, entry: &str) -> Result<&'a Kernel> {
    package(package_value)?;
    package_value
        .kernels
        .iter()
        .find(|k| k.name == entry)
        .ok_or_else(|| error("missing-entry", "requested entry is not in the package"))
}
pub fn for_kernel(kernel: &Kernel) -> Reflection {
    Reflection {
        entry: kernel.name.clone(),
        local_size: kernel.local_size,
        bindings: kernel
            .resources
            .iter()
            .enumerate()
            .map(|(binding, r)| Binding {
                resource: r.id,
                binding: binding as u32,
                element: r.element,
                access: r.access,
            })
            .collect(),
        parameter_types: kernel.parameters.clone(),
        parameter_binding: kernel.resources.len() as u32,
        guard_binding: kernel.resources.len() as u32 + 1,
    }
}
pub fn reflection(reflection: &Reflection) -> Result<()> {
    if !name(&reflection.entry) {
        return Err(error("entry-name", "invalid reflected entry name"));
    }
    local_size(reflection.local_size)?;
    if reflection.bindings.len() > MAX_RESOURCES || reflection.parameter_types.len() > 32 {
        return Err(error(
            "interface-limit",
            "reflection interface exceeds the baseline limit",
        ));
    }
    let mut ids = BTreeSet::new();
    for (index, binding) in reflection.bindings.iter().enumerate() {
        if !ids.insert(binding.resource) || binding.binding != index as u32 {
            return Err(error(
                "binding-layout",
                "resource IDs must be unique and bindings contiguous",
            ));
        }
        if binding.access == Access::Atomic && binding.element != Scalar::U32 {
            return Err(error(
                "access-mode",
                "atomic resources require u32 elements",
            ));
        }
    }
    if reflection.parameter_binding != reflection.bindings.len() as u32
        || reflection.guard_binding != reflection.parameter_binding + 1
    {
        return Err(error(
            "binding-layout",
            "parameter and guard bindings must follow the resources",
        ));
    }
    Ok(())
}
pub fn invocation(reflect: &Reflection, invocation: &Invocation) -> Result<()> {
    reflection(reflect)?;
    if invocation.entry != reflect.entry {
        return Err(error(
            "entry-mismatch",
            "invocation entry differs from reflection",
        ));
    }
    let [groups, y, z] = invocation.workgroups;
    if groups == 0
        || y != 1
        || z != 1
        || u64::from(groups) * u64::from(reflect.local_size[0]) > MAX_WORDS as u64
    {
        return Err(error(
            "dispatch-limit",
            "dispatch must be one-dimensional and within the baseline limit",
        ));
    }
    if invocation.parameters.len() != reflect.parameter_types.len()
        || invocation.buffers.len() != reflect.bindings.len()
    {
        return Err(error(
            "argument-count",
            "invocation argument counts differ from reflection",
        ));
    }
    for (word, ty) in invocation.parameters.iter().zip(&reflect.parameter_types) {
        if *ty == Scalar::Bool && *word > 1 {
            return Err(error("boolean-bits", "Boolean parameters must be 0 or 1"));
        }
    }
    let mut ids = BTreeSet::new();
    let mut total_words = 0usize;
    for buffer in &invocation.buffers {
        if !ids.insert(buffer.resource) {
            return Err(error("duplicate-resource", "invocation repeats a resource"));
        }
        let binding = reflect
            .bindings
            .iter()
            .find(|b| b.resource == buffer.resource)
            .ok_or_else(|| {
                error(
                    "missing-resource",
                    "invocation contains an undeclared resource",
                )
            })?;
        if buffer.words.is_empty() || buffer.words.len() > MAX_WORDS {
            return Err(error(
                "buffer-size",
                "resource allocation must contain 1..=1048576 initialized words",
            ));
        }
        if u64::from(buffer.offset) + u64::from(buffer.length) > buffer.words.len() as u64 {
            return Err(error(
                "view-bounds",
                "resource view exceeds its owned allocation",
            ));
        }
        total_words = total_words
            .checked_add(buffer.words.len())
            .ok_or_else(|| error("buffer-size", "resource allocation sum overflowed"))?;
        if total_words > MAX_WORDS {
            return Err(error(
                "buffer-size",
                "total resource allocations exceed the baseline limit",
            ));
        }
        if binding.element == Scalar::Bool && buffer.words.iter().any(|word| *word > 1) {
            return Err(error(
                "boolean-bits",
                "Boolean resource words must be 0 or 1",
            ));
        }
    }
    Ok(())
}
pub fn artifact(artifact: &Artifact) -> Result<()> {
    if artifact.schema != ARTIFACT_SCHEMA || !name(&artifact.format) || artifact.abi != WORD_ABI {
        return Err(error(
            "unsupported-contract",
            "unsupported artifact schema, format identity or ABI",
        ));
    }
    reflection(&artifact.reflection)?;
    if artifact.words.is_empty() || artifact.words.len() > MAX_WORDS {
        return Err(error(
            "artifact-size",
            "artifact word count exceeds the baseline limits",
        ));
    }
    if !canonical::is_digest(&artifact.parent_digest)
        || artifact.evidence.input_digest != artifact.parent_digest
        || artifact.evidence.output_digest != canonical::word_digest(&artifact.words)
    {
        return Err(error(
            "evidence-digest",
            "artifact evidence does not bind the exact input and output",
        ));
    }
    if artifact.evidence.policy != "kuiper.experimental-tested/1"
        || !name(&artifact.evidence.compiler)
    {
        return Err(error(
            "assurance-policy",
            "only the explicit experimental tested policy is currently implemented",
        ));
    }
    if artifact.requirements.is_empty()
        || artifact.requirements.iter().any(|r| !name(r))
        || artifact.requirements.windows(2).any(|p| p[0] >= p[1])
    {
        return Err(error(
            "requirements",
            "requirements must be nonempty, valid, strictly sorted and unique",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn invocation_rejects_invalid_views_and_boolean_words() {
        let r = Reflection {
            entry: "x".into(),
            local_size: [4, 1, 1],
            bindings: vec![Binding {
                resource: 0,
                binding: 0,
                element: Scalar::Bool,
                access: Access::Read,
            }],
            parameter_types: vec![],
            parameter_binding: 1,
            guard_binding: 2,
        };
        let mut i = Invocation {
            entry: "x".into(),
            workgroups: [1, 1, 1],
            parameters: vec![],
            buffers: vec![BufferArg {
                resource: 0,
                words: vec![0, 1],
                offset: 0,
                length: 2,
            }],
        };
        invocation(&r, &i).unwrap();
        i.buffers[0].offset = 1;
        assert_eq!(invocation(&r, &i).unwrap_err().code, "view-bounds");
        i.buffers[0].offset = 0;
        i.buffers[0].words[0] = 2;
        assert_eq!(invocation(&r, &i).unwrap_err().code, "boolean-bits");
    }
}
