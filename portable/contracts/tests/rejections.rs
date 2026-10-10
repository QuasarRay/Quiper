use kuiper_contracts::{canonical, validate, *};
fn package() -> Package {
    canonical::parse(include_bytes!("../../../validation/fixtures/loop.json")).unwrap()
}
#[test]
fn syntactic_value_identity_cannot_forge_global_ownership() {
    let mut package = package();
    package.kernels[0].body.instructions[0] = Instruction::Constant {
        result: ValueDecl {
            id: 1,
            ty: Scalar::U32,
        },
        bits: 0,
    };
    assert_eq!(validate::package(&package).unwrap_err().code, "write-race");
}
#[test]
fn loop_body_types_and_dominance_are_checked() {
    let mut package = package();
    let loop_ = package.kernels[0]
        .body
        .instructions
        .iter_mut()
        .find(|i| matches!(i, Instruction::While { .. }))
        .unwrap();
    if let Instruction::While {
        condition, body, ..
    } = loop_
    {
        body.outputs = condition.outputs.clone();
    }
    assert_eq!(
        validate::package(&package).unwrap_err().code,
        "unbound-value"
    );
}
#[test]
fn readwrite_memory_requires_per_invocation_index_authority() {
    let mut package = package();
    package.kernels[0].resources[0].access = Access::ReadWrite;
    package.kernels[0].body.instructions.insert(
        2,
        Instruction::Load {
            result: 555,
            resource: 0,
            index: 2,
        },
    );
    assert_eq!(validate::package(&package).unwrap_err().code, "read-race");
}
#[test]
fn backend_format_is_open_but_assurance_policy_is_not_self_admitted() {
    let package = package();
    let parent = canonical::digest(&package).unwrap();
    let words = vec![1];
    let mut artifact = Artifact {
        schema: ARTIFACT_SCHEMA.into(),
        format: "new.backend.format/1".into(),
        abi: WORD_ABI.into(),
        parent_digest: parent.clone(),
        words: words.clone(),
        reflection: validate::for_kernel(&package.kernels[0]),
        requirements: vec!["new.backend.feature/1".into()],
        evidence: Evidence {
            policy: "kuiper.experimental-tested/1".into(),
            input_digest: parent,
            output_digest: canonical::word_digest(&words),
            compiler: "new.backend/1".into(),
            dependencies: vec![],
            checks: vec![],
        },
    };
    validate::artifact(&artifact).unwrap();
    artifact.evidence.policy = "kuiper.refinement-verified/1".into();
    assert_eq!(
        validate::artifact(&artifact).unwrap_err().code,
        "assurance-policy"
    );
}
