//! Independent KIR interpreter; no compiler or GPU dependency.
use crate::error;
use kuiper_contracts::{validate, *};
use std::collections::BTreeMap;

#[derive(Clone, Copy)]
struct Value {
    ty: Scalar,
    bits: u32,
}
type Env = BTreeMap<ValueId, Value>;
struct Context<'a> {
    reflection: &'a Reflection,
    invocation: &'a Invocation,
    buffers: Vec<BufferArg>,
    global: u32,
    guard: u32,
    fuel: u64,
}
fn get(env: &Env, id: ValueId) -> Result<Value> {
    env.get(&id)
        .copied()
        .ok_or_else(|| error("undefined-value", format!("value {id}")))
}
fn binary(op: Binary, a: Value, b: Value, guard: &mut u32) -> Value {
    use Binary::*;
    let x = a.bits;
    let y = b.bits;
    let signed = a.ty == Scalar::I32;
    let boolean = matches!(op, Eq | Ne | Lt | Le | Gt | Ge | LogicalAnd | LogicalOr);
    let bits = match op {
        Add => x.wrapping_add(y),
        Sub => x.wrapping_sub(y),
        Mul => x.wrapping_mul(y),
        Div | Rem => {
            if y == 0 || (signed && x == 0x80000000 && y == u32::MAX) {
                *guard |= 2;
                0
            } else if signed {
                if op == Div {
                    ((x as i32) / (y as i32)) as u32
                } else {
                    ((x as i32) % (y as i32)) as u32
                }
            } else if op == Div {
                x / y
            } else {
                x % y
            }
        }
        ShiftLeft | ShiftRight => {
            if y >= 32 {
                *guard |= 2;
                0
            } else if op == ShiftLeft {
                x << y
            } else if signed {
                ((x as i32) >> y) as u32
            } else {
                x >> y
            }
        }
        BitAnd => x & y,
        BitOr => x | y,
        BitXor => x ^ y,
        Eq => u32::from(x == y),
        Ne => u32::from(x != y),
        Lt => u32::from(if signed {
            (x as i32) < (y as i32)
        } else {
            x < y
        }),
        Le => u32::from(if signed {
            (x as i32) <= (y as i32)
        } else {
            x <= y
        }),
        Gt => u32::from(if signed {
            (x as i32) > (y as i32)
        } else {
            x > y
        }),
        Ge => u32::from(if signed {
            (x as i32) >= (y as i32)
        } else {
            x >= y
        }),
        LogicalAnd => u32::from(x != 0 && y != 0),
        LogicalOr => u32::from(x != 0 || y != 0),
    };
    Value {
        ty: if boolean { Scalar::Bool } else { a.ty },
        bits,
    }
}
impl Context<'_> {
    fn buffer(&self, resource: u32) -> Result<usize> {
        self.buffers
            .iter()
            .position(|b| b.resource == resource)
            .ok_or_else(|| error("missing-resource", "resource missing from interpreter"))
    }
    fn address(&mut self, resource: u32, index: u32) -> Result<Option<(usize, usize)>> {
        let b = self.buffer(resource)?;
        let buffer = &self.buffers[b];
        if index >= buffer.length {
            self.guard |= 1;
            return Ok(None);
        }
        let address = u64::from(buffer.offset) + u64::from(index);
        if address >= buffer.words.len() as u64 {
            return Err(error("internal-view", "validated view is inconsistent"));
        }
        Ok(Some((b, address as usize)))
    }
    fn region(&mut self, region: &Region, env: &mut Env) -> Result<Vec<Value>> {
        for instruction in &region.instructions {
            if self.fuel == 0 {
                return Err(error("reference-budget", "interpreter fuel exhausted"));
            }
            self.fuel -= 1;
            use Instruction::*;
            match instruction {
                Constant { result, bits } => {
                    env.insert(
                        result.id,
                        Value {
                            ty: result.ty,
                            bits: *bits,
                        },
                    );
                }
                Builtin {
                    result,
                    builtin,
                    axis,
                } => {
                    let bits = if *axis != 0 {
                        if *builtin == kuiper_contracts::Builtin::NumWorkgroups {
                            1
                        } else {
                            0
                        }
                    } else {
                        match builtin {
                            kuiper_contracts::Builtin::GlobalId => self.global,
                            kuiper_contracts::Builtin::LocalId => {
                                self.global % self.reflection.local_size[0]
                            }
                            kuiper_contracts::Builtin::WorkgroupId => {
                                self.global / self.reflection.local_size[0]
                            }
                            kuiper_contracts::Builtin::NumWorkgroups => {
                                self.invocation.workgroups[0]
                            }
                        }
                    };
                    env.insert(
                        *result,
                        Value {
                            ty: Scalar::U32,
                            bits,
                        },
                    );
                }
                Parameter { result, index } => {
                    env.insert(
                        *result,
                        Value {
                            ty: self.reflection.parameter_types[*index as usize],
                            bits: self.invocation.parameters[*index as usize],
                        },
                    );
                }
                ResourceLength { result, resource } => {
                    let bits = self.buffers[self.buffer(*resource)?].length;
                    env.insert(
                        *result,
                        Value {
                            ty: Scalar::U32,
                            bits,
                        },
                    );
                }
                Binary {
                    result,
                    op,
                    left,
                    right,
                } => {
                    let value = binary(*op, get(env, *left)?, get(env, *right)?, &mut self.guard);
                    env.insert(*result, value);
                }
                Load {
                    result,
                    resource,
                    index,
                } => {
                    let ty = self
                        .reflection
                        .bindings
                        .iter()
                        .find(|b| b.resource == *resource)
                        .ok_or_else(|| error("missing-binding", "resource has no binding"))?
                        .element;
                    let bits = match self.address(*resource, get(env, *index)?.bits)? {
                        Some((b, i)) => self.buffers[b].words[i],
                        None => 0,
                    };
                    env.insert(*result, Value { ty, bits });
                }
                Store {
                    resource,
                    index,
                    value,
                } => {
                    if let Some((b, i)) = self.address(*resource, get(env, *index)?.bits)? {
                        self.buffers[b].words[i] = get(env, *value)?.bits;
                    }
                }
                AtomicAdd {
                    result,
                    resource,
                    index,
                    value,
                } => {
                    let bits = match self.address(*resource, get(env, *index)?.bits)? {
                        Some((b, i)) => {
                            let old = self.buffers[b].words[i];
                            self.buffers[b].words[i] = old.wrapping_add(get(env, *value)?.bits);
                            old
                        }
                        None => 0,
                    };
                    env.insert(
                        *result,
                        Value {
                            ty: Scalar::U32,
                            bits,
                        },
                    );
                }
                Guard { condition, code } => {
                    if get(env, *condition)?.bits == 0 {
                        self.guard |= *code;
                    }
                }
                Select {
                    condition,
                    then_region,
                    else_region,
                    results,
                } => {
                    let branch = if get(env, *condition)?.bits != 0 {
                        then_region
                    } else {
                        else_region
                    };
                    let values = self.region(branch, &mut env.clone())?;
                    for (decl, value) in results.iter().zip(values) {
                        env.insert(decl.id, value);
                    }
                }
                While {
                    carried,
                    condition,
                    body,
                    results,
                    iteration_limit,
                } => {
                    let mut values = carried
                        .iter()
                        .map(|p| get(env, p.initial))
                        .collect::<Result<Vec<_>>>()?;
                    let mut count = 0;
                    loop {
                        let mut scope = env.clone();
                        for (parameter, value) in carried.iter().zip(&values) {
                            scope.insert(parameter.value.id, *value);
                        }
                        let test = self.region(condition, &mut scope.clone())?;
                        if test[0].bits == 0 {
                            break;
                        }
                        if count >= *iteration_limit {
                            self.guard |= 4;
                            break;
                        }
                        values = self.region(body, &mut scope)?;
                        count += 1;
                    }
                    for (decl, value) in results.iter().zip(values) {
                        env.insert(decl.id, value);
                    }
                }
            }
        }
        region.outputs.iter().map(|id| get(env, *id)).collect()
    }
}

pub fn execute(package: &Package, invocation: &Invocation) -> Result<Execution> {
    validate::package(package)?;
    let kernel = package
        .kernels
        .iter()
        .find(|k| k.name == invocation.entry)
        .ok_or_else(|| error("missing-entry", "entrypoint is not in package"))?;
    let reflection = validate::for_kernel(kernel);
    validate::invocation(&reflection, invocation)?;
    let mut context = Context {
        reflection: &reflection,
        invocation,
        buffers: invocation.buffers.clone(),
        global: 0,
        guard: 0,
        fuel: 100_000_000,
    };
    let invocations = u64::from(reflection.local_size[0]) * u64::from(invocation.workgroups[0]);
    for global in 0..invocations {
        context.global = global as u32;
        context.region(&kernel.body, &mut Env::new())?;
    }
    Ok(Execution {
        buffers: context.buffers,
        guard: context.guard,
        device: "kuiper.reference/1".into(),
    })
}

pub fn postcheck(reflection: &Reflection, input: &Invocation, output: &Execution) -> Result<()> {
    if output.guard != 0 {
        return Err(error(
            "device-guard",
            format!("guard bits {}", output.guard),
        ));
    }
    let final_invocation = Invocation {
        buffers: output.buffers.clone(),
        ..input.clone()
    };
    validate::invocation(reflection, &final_invocation)?;
    for binding in &reflection.bindings {
        let before = input
            .buffers
            .iter()
            .find(|b| b.resource == binding.resource)
            .ok_or_else(|| error("shape", "input resource missing"))?;
        let after = output
            .buffers
            .iter()
            .find(|b| b.resource == binding.resource)
            .ok_or_else(|| error("shape", "output resource missing"))?;
        if before.offset != after.offset
            || before.length != after.length
            || before.words.len() != after.words.len()
        {
            return Err(error("shape", "worker changed allocation or view extent"));
        }
        let begin = before.offset as usize;
        let end = begin + before.length as usize;
        for (index, (a, b)) in before.words.iter().zip(&after.words).enumerate() {
            if a != b && (binding.access == Access::Read || index < begin || index >= end) {
                return Err(error(
                    "frame",
                    "worker changed a cell without write permission",
                ));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn value(bits: u32, ty: Scalar) -> Value {
        Value { bits, ty }
    }
    #[test]
    fn signed_division_and_remainder_truncate_to_zero() {
        let mut guard = 0;
        assert_eq!(
            binary(
                Binary::Div,
                value((-7i32) as u32, Scalar::I32),
                value(3, Scalar::I32),
                &mut guard
            )
            .bits,
            (-2i32) as u32
        );
        assert_eq!(
            binary(
                Binary::Rem,
                value((-7i32) as u32, Scalar::I32),
                value(3, Scalar::I32),
                &mut guard
            )
            .bits,
            u32::MAX
        );
        assert_eq!(guard, 0);
    }
    #[test]
    fn target_undefined_arithmetic_is_guarded() {
        for (op, x, y, ty) in [
            (Binary::Div, 3, 0, Scalar::U32),
            (Binary::Rem, 0x80000000, u32::MAX, Scalar::I32),
            (Binary::ShiftLeft, 1, 32, Scalar::U32),
        ] {
            let mut guard = 0;
            assert_eq!(binary(op, value(x, ty), value(y, ty), &mut guard).bits, 0);
            assert_eq!(guard, 2);
        }
    }
}
