use super::*;
use kuiper_contracts::{
    Access, Binary, Builtin, Instruction, Kernel, Region, Resource, Scalar, ValueDecl,
};
use spirt::{DeclDef, NodeKind, Value};

fn fixture(text: &str) -> Package {
    canonical::parse(text.as_bytes()).unwrap()
}
fn inspect_loops(package: &Package, expected: usize) {
    validate::package(package).unwrap();
    let lowered = lower::lower(&package.kernels[0]).unwrap();
    assert_eq!(lowered.loops, expected);
    let mut seen = 0;
    for export in lowered.module.exports.values() {
        let spirt::Exportee::Func(function) = export else {
            continue;
        };
        let DeclDef::Present(body) = &lowered.module.funcs[*function].def else {
            panic!("imported function")
        };
        let mut regions = vec![body.body];
        while let Some(region) = regions.pop() {
            let mut children = body.regions[region].children.iter();
            while let Some((node, rest)) = children.split_first(&body.nodes) {
                children = rest;
                match &body.nodes[node].kind {
                    NodeKind::Select { cases, .. } => regions.extend(cases),
                    NodeKind::Loop { body: region, .. } => {
                        seen += 1;
                        assert!(
                            body.nodes[node].outputs.is_empty(),
                            "Loop must not claim NodeOutput values"
                        );
                        assert!(
                            body.regions[*region]
                                .outputs
                                .iter()
                                .all(|value| matches!(value, Value::DataInstOutput(_))),
                            "loop exit values must be body-defined"
                        );
                        regions.push(*region);
                    }
                    _ => (),
                }
            }
        }
    }
    assert_eq!(seen, expected);
    compile(package, &package.kernels[0].name).unwrap();
}
#[test]
fn pretested_loop_has_an_outer_selection_and_no_loop_outputs() {
    inspect_loops(
        &fixture(include_str!("../tests/fixtures/pretested-loop.json")),
        1,
    );
}
#[test]
fn nested_loops_preserve_their_carried_tuples() {
    inspect_loops(
        &fixture(include_str!("../tests/fixtures/nested-loops.json")),
        2,
    );
}
#[test]
fn checked_memory_and_atomic_fixtures_validate() {
    for text in [
        include_str!("../tests/fixtures/checked-load.json"),
        include_str!("../tests/fixtures/guarded-vector-add.json"),
        include_str!("../tests/fixtures/atomic-add.json"),
    ] {
        let package = fixture(text);
        let artifact = compile(&package, &package.kernels[0].name).unwrap();
        assert_eq!(artifact.parent_digest, canonical::digest(&package).unwrap());
        assert_eq!(
            artifact.evidence.output_digest,
            canonical::word_digest(&artifact.words)
        );
        assert_eq!(artifact.words[0], 0x0723_0203);
        assert_eq!(artifact.words[1], 0x0001_0500);
        assert_eq!(artifact.evidence.policy, "kuiper.experimental-tested/1");
        assert_eq!(artifact.requirements, REQUIREMENTS.map(str::to_owned));
    }
}
fn arithmetic(op: Binary, ty: Scalar) -> Package {
    let output = if matches!(
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
        ty
    };
    let right_type = if matches!(op, Binary::ShiftLeft | Binary::ShiftRight) {
        Scalar::U32
    } else {
        ty
    };
    Package {
        schema: KIR_SCHEMA.into(),
        profile: INTEGER_PROFILE.into(),
        kernels: vec![Kernel {
            name: "arithmetic".into(),
            local_size: [4, 1, 1],
            resources: vec![Resource {
                id: 0,
                element: output,
                access: Access::Write,
            }],
            parameters: vec![],
            body: Region {
                instructions: vec![
                    Instruction::Builtin {
                        result: 1,
                        builtin: Builtin::GlobalId,
                        axis: 0,
                    },
                    Instruction::Constant {
                        result: ValueDecl { id: 2, ty },
                        bits: 1,
                    },
                    Instruction::Constant {
                        result: ValueDecl {
                            id: 3,
                            ty: right_type,
                        },
                        bits: 1,
                    },
                    Instruction::Binary {
                        result: 4,
                        op,
                        left: 2,
                        right: 3,
                    },
                    Instruction::Store {
                        resource: 0,
                        index: 1,
                        value: 4,
                    },
                ],
                outputs: vec![],
            },
        }],
    }
}
#[test]
fn every_supported_scalar_operation_lifts_to_valid_spirv() {
    for ty in [Scalar::U32, Scalar::I32] {
        for op in [
            Binary::Add,
            Binary::Sub,
            Binary::Mul,
            Binary::Div,
            Binary::Rem,
            Binary::BitAnd,
            Binary::BitOr,
            Binary::BitXor,
            Binary::ShiftLeft,
            Binary::ShiftRight,
            Binary::Eq,
            Binary::Ne,
            Binary::Lt,
            Binary::Le,
            Binary::Gt,
            Binary::Ge,
        ] {
            compile(&arithmetic(op, ty), "arithmetic").unwrap();
        }
    }
    for op in [
        Binary::Eq,
        Binary::Ne,
        Binary::LogicalAnd,
        Binary::LogicalOr,
    ] {
        compile(&arithmetic(op, Scalar::Bool), "arithmetic").unwrap();
    }
}
#[test]
fn unsupported_profile_and_unproved_writes_fail_closed() {
    let mut package = arithmetic(Binary::Add, Scalar::U32);
    package.profile = "unsupported.profile/1".into();
    assert_eq!(
        compile(&package, "arithmetic").unwrap_err().code,
        "unsupported-contract"
    );
    package.profile = INTEGER_PROFILE.into();
    let Instruction::Store { index, .. } = &mut package.kernels[0].body.instructions[4] else {
        unreachable!()
    };
    *index = 2;
    assert_eq!(
        compile(&package, "arithmetic").unwrap_err().code,
        "write-race"
    );
}
#[test]
fn validator_rejects_an_invalid_module() {
    assert_eq!(
        validate_words(&[0x0723_0203, 0x0001_0500, 0, 1, 0])
            .unwrap_err()
            .code,
        "invalid-spirv"
    );
}
#[test]
fn attached_spirt_diagnostics_cannot_be_admitted() {
    let mut lowered = lower::lower(&arithmetic(Binary::Add, Scalar::U32).kernels[0]).unwrap();
    let cx = lowered.module.cx();
    let function = lowered
        .module
        .exports
        .values()
        .find_map(|e| {
            if let spirt::Exportee::Func(f) = e {
                Some(*f)
            } else {
                None
            }
        })
        .unwrap();
    lowered.module.funcs[function].attrs.push_diag(
        &cx,
        spirt::Diag::err([spirt::DiagMsgPart::Plain(
            "unqualified transformation".into(),
        )]),
    );
    assert_eq!(
        inspect::clean(&lowered.module).unwrap_err().code,
        "unqualified-ir"
    );
}
#[test]
fn effectful_conditions_have_initial_and_posttest_sites() {
    let package = fixture(include_str!("../tests/fixtures/condition-effects.json"));
    let artifact = compile(&package, &package.kernels[0].name).unwrap();
    let atomic = spirt::spv::spec::Spec::get()
        .instructions
        .lookup("OpAtomicIAdd")
        .unwrap()
        .as_u16() as u32;
    let mut position = 5;
    let mut count = 0;
    while position < artifact.words.len() {
        let header = artifact.words[position];
        let length = (header >> 16) as usize;
        assert!(length > 0 && position + length <= artifact.words.len());
        if header & 0xffff == atomic {
            count += 1;
        }
        position += length;
    }
    // This checks static sites. Execution order is checked by the Vulkan runtime tests.
    assert_eq!(count, 2);
}
