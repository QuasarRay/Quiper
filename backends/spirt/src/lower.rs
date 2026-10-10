use crate::spirv_values::*;
use kuiper_contracts::{
    Access, Binary, Builtin, Diagnostic, Instruction, Kernel, Region as KirRegion, Result, Scalar,
    ValueId,
};
use smallvec::{SmallVec, smallvec};
use spirt::{
    AddrSpace, Attr, AttrSet, AttrSetDef, ConstDef, ConstKind, Context, DataInstDef, DataInstKind,
    DeclDef, ExportKey, Exportee, FuncDecl, FuncDefBody, GlobalVar, GlobalVarDecl,
    GlobalVarDefBody, Module, ModuleDebugInfo, ModuleDialect, NodeDef, NodeKind, NodeOutputDecl,
    Region, RegionDef, RegionInputDecl, SelectionKind, Type, TypeDef, TypeKind, TypeOrConst, Value,
    spv,
};
use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

#[derive(Copy, Clone)]
struct TypedValue {
    value: Value,
    scalar: Scalar,
}
type Env = BTreeMap<ValueId, TypedValue>;

fn instruction(name: &str, operands: &[(&str, u32)]) -> spv::Inst {
    let spec = spv::spec::Spec::get();
    spv::Inst {
        opcode: spec
            .instructions
            .lookup(name)
            .expect("instruction in pinned grammar"),
        imms: operands
            .iter()
            .map(|(kind, value)| {
                spv::Imm::Short(
                    spec.operand_kinds
                        .lookup(kind)
                        .expect("operand in pinned grammar"),
                    *value,
                )
            })
            .collect(),
    }
}
fn attributes(cx: &Context, annotations: impl IntoIterator<Item = spv::Inst>) -> AttrSet {
    cx.intern(AttrSetDef {
        attrs: annotations.into_iter().map(Attr::SpvAnnotation).collect(),
    })
}
fn scalar_type(cx: &Context, scalar: Scalar) -> Type {
    let spv_inst = match scalar {
        Scalar::Bool => instruction("OpTypeBool", &[]),
        Scalar::U32 => instruction(
            "OpTypeInt",
            &[("LiteralInteger", 32), ("LiteralInteger", 0)],
        ),
        Scalar::I32 => instruction(
            "OpTypeInt",
            &[("LiteralInteger", 32), ("LiteralInteger", 1)],
        ),
    };
    cx.intern(TypeDef {
        attrs: AttrSet::default(),
        kind: TypeKind::SpvInst {
            spv_inst,
            type_and_const_inputs: SmallVec::new(),
        },
    })
}
fn literal(cx: &Context, scalar: Scalar, bits: u32) -> Value {
    let spv_inst = match scalar {
        Scalar::Bool => instruction(
            if bits == 0 {
                "OpConstantFalse"
            } else {
                "OpConstantTrue"
            },
            &[],
        ),
        Scalar::U32 | Scalar::I32 => {
            instruction("OpConstant", &[("LiteralContextDependentNumber", bits)])
        }
    };
    Value::Const(cx.intern(ConstDef {
        attrs: AttrSet::default(),
        ty: scalar_type(cx, scalar),
        kind: ConstKind::SpvInst {
            spv_inst_and_const_inputs: Rc::new((spv_inst, SmallVec::new())),
        },
    }))
}
fn pointer_type(cx: &Context, storage: u32, pointee: Type) -> Type {
    cx.intern(TypeDef {
        attrs: AttrSet::default(),
        kind: TypeKind::SpvInst {
            spv_inst: instruction("OpTypePointer", &[("StorageClass", storage)]),
            type_and_const_inputs: smallvec![TypeOrConst::Type(pointee)],
        },
    })
}
fn global_pointer(cx: &Context, module: &Module, variable: GlobalVar) -> Value {
    Value::Const(cx.intern(ConstDef {
        attrs: AttrSet::default(),
        ty: module.global_vars[variable].type_of_ptr_to,
        kind: ConstKind::PtrToGlobalVar(variable),
    }))
}
fn word_buffer(module: &mut Module, binding: u32, read_only: bool) -> GlobalVar {
    let cx = module.cx();
    let array = cx.intern(TypeDef {
        attrs: attributes(
            &cx,
            [instruction(
                "OpDecorate",
                &[("Decoration", ARRAY_STRIDE), ("LiteralInteger", 4)],
            )],
        ),
        kind: TypeKind::SpvInst {
            spv_inst: instruction("OpTypeRuntimeArray", &[]),
            type_and_const_inputs: smallvec![TypeOrConst::Type(scalar_type(&cx, Scalar::U32))],
        },
    });
    let structure = cx.intern(TypeDef {
        attrs: attributes(
            &cx,
            [
                instruction("OpDecorate", &[("Decoration", BLOCK)]),
                instruction(
                    "OpMemberDecorate",
                    &[
                        ("LiteralInteger", 0),
                        ("Decoration", OFFSET),
                        ("LiteralInteger", 0),
                    ],
                ),
            ],
        ),
        kind: TypeKind::SpvInst {
            spv_inst: instruction("OpTypeStruct", &[]),
            type_and_const_inputs: smallvec![TypeOrConst::Type(array)],
        },
    });
    let mut annotations = vec![
        instruction(
            "OpDecorate",
            &[("Decoration", DESCRIPTOR_SET), ("LiteralInteger", 0)],
        ),
        instruction(
            "OpDecorate",
            &[("Decoration", BINDING), ("LiteralInteger", binding)],
        ),
    ];
    if read_only {
        annotations.push(instruction("OpDecorate", &[("Decoration", NON_WRITABLE)]));
    }
    module.global_vars.define(
        &cx,
        GlobalVarDecl {
            attrs: attributes(&cx, annotations),
            type_of_ptr_to: pointer_type(&cx, STORAGE_BUFFER, structure),
            shape: None,
            addr_space: AddrSpace::SpvStorageClass(STORAGE_BUFFER),
            def: DeclDef::Present(GlobalVarDefBody { initializer: None }),
        },
    )
}
fn builtin_global(module: &mut Module, builtin: u32) -> GlobalVar {
    let cx = module.cx();
    let vector = cx.intern(TypeDef {
        attrs: AttrSet::default(),
        kind: TypeKind::SpvInst {
            spv_inst: instruction("OpTypeVector", &[("LiteralInteger", 3)]),
            type_and_const_inputs: smallvec![TypeOrConst::Type(scalar_type(&cx, Scalar::U32))],
        },
    });
    module.global_vars.define(
        &cx,
        GlobalVarDecl {
            attrs: attributes(
                &cx,
                [instruction(
                    "OpDecorate",
                    &[("Decoration", BUILTIN), ("BuiltIn", builtin)],
                )],
            ),
            type_of_ptr_to: pointer_type(&cx, STORAGE_INPUT, vector),
            shape: None,
            addr_space: AddrSpace::SpvStorageClass(STORAGE_INPUT),
            def: DeclDef::Present(GlobalVarDefBody { initializer: None }),
        },
    )
}

pub(crate) struct Lowered {
    pub module: Module,
    #[cfg_attr(not(test), allow(dead_code))]
    pub loops: usize,
}
struct Builder {
    module: Module,
    func: FuncDefBody,
    resources: BTreeMap<u32, (GlobalVar, usize, Scalar)>,
    parameters: GlobalVar,
    guard: GlobalVar,
    builtins: [GlobalVar; 4],
    parameter_types: Vec<Scalar>,
    loops: usize,
}
impl Builder {
    fn cx(&self) -> Rc<Context> {
        self.module.cx()
    }
    fn region(&mut self) -> Region {
        self.func.regions.define(&self.cx(), RegionDef::default())
    }
    fn append_node(&mut self, region: Region, node: NodeDef) -> spirt::Node {
        let node = self.func.nodes.define(&self.cx(), node.into());
        self.func.regions[region]
            .children
            .insert_last(node, &mut self.func.nodes);
        node
    }
    fn emit(
        &mut self,
        region: Region,
        inst: spv::Inst,
        ty: Option<Type>,
        inputs: impl IntoIterator<Item = Value>,
    ) -> Option<Value> {
        let data = self.func.data_insts.define(
            &self.cx(),
            DataInstDef {
                attrs: AttrSet::default(),
                kind: DataInstKind::SpvInst(inst),
                inputs: inputs.into_iter().collect(),
                output_type: ty,
            }
            .into(),
        );
        let mut insts = spirt::EntityList::default();
        insts.insert_last(data, &mut self.func.data_insts);
        self.append_node(
            region,
            NodeDef {
                kind: NodeKind::Block { insts },
                outputs: SmallVec::new(),
            },
        );
        ty.map(|_| Value::DataInstOutput(data))
    }
    fn op(
        &mut self,
        region: Region,
        name: &str,
        scalar: Scalar,
        inputs: impl IntoIterator<Item = Value>,
    ) -> Value {
        self.emit(
            region,
            instruction(name, &[]),
            Some(scalar_type(&self.cx(), scalar)),
            inputs,
        )
        .unwrap()
    }
    fn c(&self, scalar: Scalar, bits: u32) -> Value {
        literal(&self.cx(), scalar, bits)
    }
    fn get(env: &Env, id: ValueId) -> Result<TypedValue> {
        env.get(&id).copied().ok_or_else(|| {
            Diagnostic::new(
                "spirt",
                "unbound-value",
                format!("value {id} does not dominate its use"),
            )
        })
    }
    fn word_pointer(&mut self, region: Region, variable: GlobalVar, index: Value) -> Value {
        self.emit(
            region,
            instruction("OpAccessChain", &[]),
            Some(pointer_type(
                &self.cx(),
                STORAGE_BUFFER,
                scalar_type(&self.cx(), Scalar::U32),
            )),
            [
                global_pointer(&self.cx(), &self.module, variable),
                self.c(Scalar::U32, 0),
                index,
            ],
        )
        .unwrap()
    }
    fn raw_load(&mut self, region: Region, variable: GlobalVar, index: Value) -> Value {
        let pointer = self.word_pointer(region, variable, index);
        self.op(region, "OpLoad", Scalar::U32, [pointer])
    }
    fn word_to_scalar(&mut self, region: Region, word: Value, scalar: Scalar) -> Value {
        match scalar {
            Scalar::U32 => word,
            Scalar::I32 => self.op(region, "OpBitcast", Scalar::I32, [word]),
            Scalar::Bool => self.op(
                region,
                "OpINotEqual",
                Scalar::Bool,
                [word, self.c(Scalar::U32, 0)],
            ),
        }
    }
    fn scalar_to_word(&mut self, region: Region, value: TypedValue) -> Value {
        match value.scalar {
            Scalar::U32 => value.value,
            Scalar::I32 => self.op(region, "OpBitcast", Scalar::U32, [value.value]),
            Scalar::Bool => self.op(
                region,
                "OpSelect",
                Scalar::U32,
                [value.value, self.c(Scalar::U32, 1), self.c(Scalar::U32, 0)],
            ),
        }
    }
    fn select(
        &mut self,
        region: Region,
        condition: Value,
        yes: Region,
        no: Region,
        types: &[Scalar],
    ) -> Vec<Value> {
        let outputs = types
            .iter()
            .map(|ty| NodeOutputDecl {
                attrs: AttrSet::default(),
                ty: scalar_type(&self.cx(), *ty),
            })
            .collect();
        let node = self.append_node(
            region,
            NodeDef {
                kind: NodeKind::Select {
                    kind: SelectionKind::BoolCond,
                    scrutinee: condition,
                    cases: smallvec![yes, no],
                },
                outputs,
            },
        );
        types
            .iter()
            .enumerate()
            .map(|(index, _)| Value::NodeOutput {
                node,
                output_idx: index as u32,
            })
            .collect()
    }
    fn record_guard(&mut self, region: Region, code: u32) {
        let pointer = self.word_pointer(region, self.guard, self.c(Scalar::U32, 0));
        self.op(
            region,
            "OpAtomicOr",
            Scalar::U32,
            [
                pointer,
                self.c(Scalar::U32, SCOPE_DEVICE),
                self.c(Scalar::U32, SEMANTICS_RELAXED),
                self.c(Scalar::U32, code),
            ],
        );
    }
    fn guard_unless(&mut self, region: Region, condition: Value, code: u32) {
        let yes = self.region();
        let no = self.region();
        self.record_guard(no, code);
        self.select(region, condition, yes, no, &[]);
    }
    fn checked_index(
        &mut self,
        region: Region,
        resource: u32,
        index: Value,
    ) -> Result<(GlobalVar, Scalar, Value, Value)> {
        let (variable, position, scalar) =
            self.resources.get(&resource).copied().ok_or_else(|| {
                Diagnostic::new("spirt", "missing-resource", "resource was not declared")
            })?;
        let offset = self.raw_load(
            region,
            self.parameters,
            self.c(Scalar::U32, position as u32 * 2),
        );
        let length = self.raw_load(
            region,
            self.parameters,
            self.c(Scalar::U32, position as u32 * 2 + 1),
        );
        let valid = self.op(region, "OpULessThan", Scalar::Bool, [index, length]);
        Ok((variable, scalar, offset, valid))
    }
    fn load(&mut self, region: Region, resource: u32, index: Value) -> Result<TypedValue> {
        let (variable, scalar, offset, valid) = self.checked_index(region, resource, index)?;
        let yes = self.region();
        let no = self.region();
        // Invocation checking establishes offset + length <= the owned allocation.
        // Pointer formation and access happen only after the logical index guard.
        let physical = self.op(yes, "OpIAdd", Scalar::U32, [offset, index]);
        let word = self.raw_load(yes, variable, physical);
        let value = self.word_to_scalar(yes, word, scalar);
        self.func.regions[yes].outputs = smallvec![value];
        self.record_guard(no, 1);
        self.func.regions[no].outputs = smallvec![self.c(scalar, 0)];
        Ok(TypedValue {
            value: self.select(region, valid, yes, no, &[scalar])[0],
            scalar,
        })
    }
    fn store(
        &mut self,
        region: Region,
        resource: u32,
        index: Value,
        value: TypedValue,
    ) -> Result<()> {
        let (variable, _, offset, valid) = self.checked_index(region, resource, index)?;
        let yes = self.region();
        let no = self.region();
        let physical = self.op(yes, "OpIAdd", Scalar::U32, [offset, index]);
        let pointer = self.word_pointer(yes, variable, physical);
        let word = self.scalar_to_word(yes, value);
        self.emit(yes, instruction("OpStore", &[]), None, [pointer, word]);
        self.record_guard(no, 1);
        self.select(region, valid, yes, no, &[]);
        Ok(())
    }
    fn atomic_add(
        &mut self,
        region: Region,
        resource: u32,
        index: Value,
        value: Value,
    ) -> Result<TypedValue> {
        let (variable, _, offset, valid) = self.checked_index(region, resource, index)?;
        let yes = self.region();
        let no = self.region();
        let physical = self.op(yes, "OpIAdd", Scalar::U32, [offset, index]);
        let pointer = self.word_pointer(yes, variable, physical);
        let old = self.op(
            yes,
            "OpAtomicIAdd",
            Scalar::U32,
            [
                pointer,
                self.c(Scalar::U32, SCOPE_DEVICE),
                self.c(Scalar::U32, SEMANTICS_RELAXED),
                value,
            ],
        );
        self.func.regions[yes].outputs = smallvec![old];
        self.record_guard(no, 1);
        self.func.regions[no].outputs = smallvec![self.c(Scalar::U32, 0)];
        Ok(TypedValue {
            value: self.select(region, valid, yes, no, &[Scalar::U32])[0],
            scalar: Scalar::U32,
        })
    }
    fn binary(
        &mut self,
        region: Region,
        op: Binary,
        left: TypedValue,
        right: TypedValue,
    ) -> TypedValue {
        let signed = left.scalar == Scalar::I32;
        let scalar = if matches!(
            op,
            Binary::Eq
                | Binary::Ne
                | Binary::Lt
                | Binary::Le
                | Binary::Gt
                | Binary::Ge
                | Binary::LogicalAnd
                | Binary::LogicalOr
        ) {
            Scalar::Bool
        } else {
            left.scalar
        };
        let name = match op {
            Binary::Add => "OpIAdd",
            Binary::Sub => "OpISub",
            Binary::Mul => "OpIMul",
            Binary::Div => {
                if signed {
                    "OpSDiv"
                } else {
                    "OpUDiv"
                }
            }
            Binary::Rem => {
                if signed {
                    "OpSRem"
                } else {
                    "OpUMod"
                }
            }
            Binary::BitAnd => "OpBitwiseAnd",
            Binary::BitOr => "OpBitwiseOr",
            Binary::BitXor => "OpBitwiseXor",
            Binary::ShiftLeft => "OpShiftLeftLogical",
            Binary::ShiftRight => {
                if signed {
                    "OpShiftRightArithmetic"
                } else {
                    "OpShiftRightLogical"
                }
            }
            Binary::Eq => {
                if left.scalar == Scalar::Bool {
                    "OpLogicalEqual"
                } else {
                    "OpIEqual"
                }
            }
            Binary::Ne => {
                if left.scalar == Scalar::Bool {
                    "OpLogicalNotEqual"
                } else {
                    "OpINotEqual"
                }
            }
            Binary::Lt => {
                if signed {
                    "OpSLessThan"
                } else {
                    "OpULessThan"
                }
            }
            Binary::Le => {
                if signed {
                    "OpSLessThanEqual"
                } else {
                    "OpULessThanEqual"
                }
            }
            Binary::Gt => {
                if signed {
                    "OpSGreaterThan"
                } else {
                    "OpUGreaterThan"
                }
            }
            Binary::Ge => {
                if signed {
                    "OpSGreaterThanEqual"
                } else {
                    "OpUGreaterThanEqual"
                }
            }
            Binary::LogicalAnd => "OpLogicalAnd",
            Binary::LogicalOr => "OpLogicalOr",
        };
        let valid = match op {
            Binary::Div | Binary::Rem => {
                let nonzero = self.op(
                    region,
                    "OpINotEqual",
                    Scalar::Bool,
                    [right.value, self.c(right.scalar, 0)],
                );
                if signed {
                    let minimum = self.op(
                        region,
                        "OpIEqual",
                        Scalar::Bool,
                        [left.value, self.c(Scalar::I32, 0x8000_0000)],
                    );
                    let minus_one = self.op(
                        region,
                        "OpIEqual",
                        Scalar::Bool,
                        [right.value, self.c(Scalar::I32, u32::MAX)],
                    );
                    let overflow =
                        self.op(region, "OpLogicalAnd", Scalar::Bool, [minimum, minus_one]);
                    let safe = self.op(region, "OpLogicalNot", Scalar::Bool, [overflow]);
                    Some(self.op(region, "OpLogicalAnd", Scalar::Bool, [nonzero, safe]))
                } else {
                    Some(nonzero)
                }
            }
            Binary::ShiftLeft | Binary::ShiftRight => Some(self.op(
                region,
                "OpULessThan",
                Scalar::Bool,
                [right.value, self.c(right.scalar, 32)],
            )),
            _ => None,
        };
        let value = if let Some(valid) = valid {
            let yes = self.region();
            let no = self.region();
            let result = self.op(yes, name, scalar, [left.value, right.value]);
            self.func.regions[yes].outputs = smallvec![result];
            self.record_guard(no, 2);
            self.func.regions[no].outputs = smallvec![self.c(scalar, 0)];
            self.select(region, valid, yes, no, &[scalar])[0]
        } else {
            self.op(region, name, scalar, [left.value, right.value])
        };
        TypedValue { value, scalar }
    }
    fn lower_region(
        &mut self,
        target: Region,
        source: &KirRegion,
        env: &mut Env,
    ) -> Result<Vec<TypedValue>> {
        for inst in &source.instructions {
            let result = match inst {
                Instruction::Constant { result, bits } => Some((
                    result.id,
                    TypedValue {
                        value: self.c(result.ty, *bits),
                        scalar: result.ty,
                    },
                )),
                Instruction::Builtin {
                    result,
                    builtin,
                    axis,
                } => {
                    let variable = self.builtins[match builtin {
                        Builtin::GlobalId => 0,
                        Builtin::LocalId => 1,
                        Builtin::WorkgroupId => 2,
                        Builtin::NumWorkgroups => 3,
                    }];
                    let vector = self.cx().intern(TypeDef {
                        attrs: AttrSet::default(),
                        kind: TypeKind::SpvInst {
                            spv_inst: instruction("OpTypeVector", &[("LiteralInteger", 3)]),
                            type_and_const_inputs: smallvec![TypeOrConst::Type(scalar_type(
                                &self.cx(),
                                Scalar::U32
                            ))],
                        },
                    });
                    let value = self
                        .emit(
                            target,
                            instruction("OpLoad", &[]),
                            Some(vector),
                            [global_pointer(&self.cx(), &self.module, variable)],
                        )
                        .unwrap();
                    let value = self
                        .emit(
                            target,
                            instruction("OpCompositeExtract", &[("LiteralInteger", *axis)]),
                            Some(scalar_type(&self.cx(), Scalar::U32)),
                            [value],
                        )
                        .unwrap();
                    Some((
                        *result,
                        TypedValue {
                            value,
                            scalar: Scalar::U32,
                        },
                    ))
                }
                Instruction::Parameter { result, index } => {
                    let scalar = self.parameter_types[*index as usize];
                    let word = self.raw_load(
                        target,
                        self.parameters,
                        self.c(Scalar::U32, self.resources.len() as u32 * 2 + *index),
                    );
                    let value = self.word_to_scalar(target, word, scalar);
                    Some((*result, TypedValue { value, scalar }))
                }
                Instruction::ResourceLength { result, resource } => {
                    let (_, position, _) =
                        self.resources.get(resource).copied().ok_or_else(|| {
                            Diagnostic::new(
                                "spirt",
                                "missing-resource",
                                "resource was not declared",
                            )
                        })?;
                    let value = self.raw_load(
                        target,
                        self.parameters,
                        self.c(Scalar::U32, position as u32 * 2 + 1),
                    );
                    Some((
                        *result,
                        TypedValue {
                            value,
                            scalar: Scalar::U32,
                        },
                    ))
                }
                Instruction::Binary {
                    result,
                    op,
                    left,
                    right,
                } => Some((
                    *result,
                    self.binary(target, *op, Self::get(env, *left)?, Self::get(env, *right)?),
                )),
                Instruction::Load {
                    result,
                    resource,
                    index,
                } => Some((
                    *result,
                    self.load(target, *resource, Self::get(env, *index)?.value)?,
                )),
                Instruction::Store {
                    resource,
                    index,
                    value,
                } => {
                    self.store(
                        target,
                        *resource,
                        Self::get(env, *index)?.value,
                        Self::get(env, *value)?,
                    )?;
                    None
                }
                Instruction::AtomicAdd {
                    result,
                    resource,
                    index,
                    value,
                } => Some((
                    *result,
                    self.atomic_add(
                        target,
                        *resource,
                        Self::get(env, *index)?.value,
                        Self::get(env, *value)?.value,
                    )?,
                )),
                Instruction::Guard { condition, code } => {
                    self.guard_unless(target, Self::get(env, *condition)?.value, *code);
                    None
                }
                Instruction::Select {
                    condition,
                    then_region,
                    else_region,
                    results,
                } => {
                    let yes = self.region();
                    let no = self.region();
                    let yes_values = self.lower_region(yes, then_region, &mut env.clone())?;
                    let no_values = self.lower_region(no, else_region, &mut env.clone())?;
                    self.func.regions[yes].outputs = yes_values.iter().map(|v| v.value).collect();
                    self.func.regions[no].outputs = no_values.iter().map(|v| v.value).collect();
                    let types: Vec<_> = results.iter().map(|r| r.ty).collect();
                    let values =
                        self.select(target, Self::get(env, *condition)?.value, yes, no, &types);
                    for (decl, value) in results.iter().zip(values) {
                        env.insert(
                            decl.id,
                            TypedValue {
                                value,
                                scalar: decl.ty,
                            },
                        );
                    }
                    None
                }
                Instruction::While {
                    carried,
                    condition,
                    body,
                    results,
                    iteration_limit,
                } => {
                    let mut initial_env = env.clone();
                    let mut initial_values = SmallVec::<[Value; 2]>::new();
                    for param in carried {
                        let value = Self::get(env, param.initial)?;
                        initial_values.push(value.value);
                        initial_env.insert(param.value.id, value);
                    }
                    // KIR is pretested, while SPIR-T Loop is tail-controlled. Test once
                    // outside the loop, including all condition-region effects.
                    let initial_test =
                        self.lower_region(target, condition, &mut initial_env)?[0].value;
                    let yes = self.region();
                    let no = self.region();
                    let loop_body = self.region();
                    for (index, param) in carried.iter().enumerate() {
                        let ty = scalar_type(&self.cx(), param.value.ty);
                        self.func.regions[loop_body].inputs.push(RegionInputDecl {
                            attrs: AttrSet::default(),
                            ty,
                        });
                        initial_env.insert(
                            param.value.id,
                            TypedValue {
                                value: Value::RegionInput {
                                    region: loop_body,
                                    input_idx: index as u32,
                                },
                                scalar: param.value.ty,
                            },
                        );
                    }
                    let counter_type = scalar_type(&self.cx(), Scalar::U32);
                    self.func.regions[loop_body].inputs.push(RegionInputDecl {
                        attrs: AttrSet::default(),
                        ty: counter_type,
                    });
                    let current_counter = Value::RegionInput {
                        region: loop_body,
                        input_idx: carried.len() as u32,
                    };
                    let updated = self.lower_region(loop_body, body, &mut initial_env.clone())?;
                    let mut finals = SmallVec::<[Value; 2]>::new();
                    for (param, value) in carried.iter().zip(updated) {
                        // Loop has no NodeOutput. Copies define final values in the
                        // body, dominating the positive-iteration branch's exit.
                        let value =
                            self.op(loop_body, "OpCopyObject", param.value.ty, [value.value]);
                        finals.push(value);
                        initial_env.insert(
                            param.value.id,
                            TypedValue {
                                value,
                                scalar: param.value.ty,
                            },
                        );
                    }
                    let final_test =
                        self.lower_region(loop_body, condition, &mut initial_env)?[0].value;
                    let counter = self.op(
                        loop_body,
                        "OpIAdd",
                        Scalar::U32,
                        [current_counter, self.c(Scalar::U32, 1)],
                    );
                    let budget_left = self.op(
                        loop_body,
                        "OpULessThan",
                        Scalar::Bool,
                        [counter, self.c(Scalar::U32, *iteration_limit)],
                    );
                    let stopped = self.op(loop_body, "OpLogicalNot", Scalar::Bool, [final_test]);
                    let not_exhausted = self.op(
                        loop_body,
                        "OpLogicalOr",
                        Scalar::Bool,
                        [stopped, budget_left],
                    );
                    self.guard_unless(loop_body, not_exhausted, 4);
                    let repeat = self.op(
                        loop_body,
                        "OpLogicalAnd",
                        Scalar::Bool,
                        [final_test, budget_left],
                    );
                    self.func.regions[loop_body].outputs = finals.clone();
                    self.func.regions[loop_body].outputs.push(counter);
                    let mut loop_initial = initial_values.clone();
                    loop_initial.push(self.c(Scalar::U32, 0));
                    let loop_node = self.append_node(
                        yes,
                        NodeDef {
                            kind: NodeKind::Loop {
                                initial_inputs: loop_initial,
                                body: loop_body,
                                repeat_condition: repeat,
                            },
                            outputs: SmallVec::new(),
                        },
                    );
                    debug_assert!(self.func.nodes[loop_node].outputs.is_empty());
                    self.loops += 1;
                    self.func.regions[yes].outputs = finals;
                    self.func.regions[no].outputs = initial_values;
                    let types: Vec<_> = results.iter().map(|r| r.ty).collect();
                    let values = self.select(target, initial_test, yes, no, &types);
                    for (decl, value) in results.iter().zip(values) {
                        env.insert(
                            decl.id,
                            TypedValue {
                                value,
                                scalar: decl.ty,
                            },
                        );
                    }
                    None
                }
            };
            if let Some((id, value)) = result {
                env.insert(id, value);
            }
        }
        source
            .outputs
            .iter()
            .map(|id| Self::get(env, *id))
            .collect()
    }
}

pub(crate) fn lower(kernel: &Kernel) -> Result<Lowered> {
    let cx = Rc::new(Context::new());
    let mut module = Module::new(
        cx.clone(),
        ModuleDialect::Spv(spv::Dialect {
            version_major: 1,
            version_minor: 5,
            capabilities: BTreeSet::from([
                SHADER,
                VULKAN_MEMORY_MODEL,
                VULKAN_MEMORY_MODEL_DEVICE_SCOPE,
            ]),
            extensions: BTreeSet::new(),
            addressing_model: ADDRESSING_LOGICAL,
            memory_model: MEMORY_MODEL_VULKAN,
        }),
        ModuleDebugInfo::Spv(spv::ModuleDebugInfo {
            original_generator_magic: None,
            source_languages: BTreeMap::new(),
            source_extensions: vec![],
            module_processes: vec!["Kuiper direct KIR construction; no QPtr or importer".into()],
        }),
    );
    let mut resources = BTreeMap::new();
    let mut interfaces = SmallVec::<[GlobalVar; 4]>::new();
    for (position, resource) in kernel.resources.iter().enumerate() {
        let variable = word_buffer(
            &mut module,
            position as u32,
            resource.access == Access::Read,
        );
        resources.insert(resource.id, (variable, position, resource.element));
        interfaces.push(variable);
    }
    let parameters = word_buffer(&mut module, kernel.resources.len() as u32, true);
    let guard = word_buffer(&mut module, kernel.resources.len() as u32 + 1, false);
    interfaces.extend([parameters, guard]);
    let builtins = [
        GLOBAL_INVOCATION_ID,
        LOCAL_INVOCATION_ID,
        WORKGROUP_ID,
        NUM_WORKGROUPS,
    ]
    .map(|builtin| builtin_global(&mut module, builtin));
    interfaces.extend(builtins);
    let mut regions = spirt::EntityDefs::default();
    let body = regions.define(&cx, RegionDef::default());
    let mut builder = Builder {
        module,
        func: FuncDefBody {
            regions,
            nodes: Default::default(),
            data_insts: Default::default(),
            body,
            unstructured_cfg: None,
        },
        resources,
        parameters,
        guard,
        builtins,
        parameter_types: kernel.parameters.clone(),
        loops: 0,
    };
    builder.lower_region(body, &kernel.body, &mut Env::new())?;
    let void = cx.intern(TypeDef {
        attrs: AttrSet::default(),
        kind: TypeKind::SpvInst {
            spv_inst: instruction("OpTypeVoid", &[]),
            type_and_const_inputs: SmallVec::new(),
        },
    });
    let function = builder.module.funcs.define(
        &cx,
        FuncDecl {
            attrs: attributes(
                &cx,
                [instruction(
                    "OpExecutionMode",
                    &[
                        ("ExecutionMode", LOCAL_SIZE),
                        ("LiteralInteger", kernel.local_size[0]),
                        ("LiteralInteger", kernel.local_size[1]),
                        ("LiteralInteger", kernel.local_size[2]),
                    ],
                )],
            ),
            ret_type: void,
            params: SmallVec::new(),
            def: DeclDef::Present(builder.func),
        },
    );
    let spec = spv::spec::Spec::get();
    let mut imms = smallvec![spv::Imm::Short(
        spec.operand_kinds.lookup("ExecutionModel").unwrap(),
        GL_COMPUTE
    )];
    imms.extend(spv::encode_literal_string(&kernel.name));
    builder.module.exports.insert(
        ExportKey::SpvEntryPoint {
            imms,
            interface_global_vars: interfaces,
        },
        Exportee::Func(function),
    );
    Ok(Lowered {
        module: builder.module,
        loops: builder.loops,
    })
}
