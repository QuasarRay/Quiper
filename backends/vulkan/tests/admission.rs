use kuiper_contracts::{Artifact, canonical};
use spirv::{Decoration, Op};

fn fixture() -> Artifact {
    canonical::parse(include_bytes!("fixtures/guarded-vector-add.json")).unwrap()
}
fn rebind(a: &mut Artifact) {
    a.evidence.output_digest = canonical::word_digest(&a.words);
}
fn mutate(a: &mut Artifact, op: Op, change: impl FnOnce(&mut [u32])) {
    let mut position = 5;
    let mut change = Some(change);
    while position < a.words.len() {
        let header = a.words[position];
        let count = (header >> 16) as usize;
        if header & 0xffff == op as u32 {
            change.take().unwrap()(&mut a.words[position + 1..position + count]);
            rebind(a);
            return;
        }
        position += count;
    }
    panic!("fixture lacks {op:?}")
}
#[test]
fn independently_admits_all_emitted_interfaces() {
    for bytes in [
        include_bytes!("fixtures/guarded-vector-add.json").as_slice(),
        include_bytes!("fixtures/checked-load.json"),
        include_bytes!("fixtures/pretested-loop.json"),
        include_bytes!("fixtures/nested-loops.json"),
        include_bytes!("fixtures/atomic-add.json"),
        include_bytes!("fixtures/condition-effects.json"),
        include_bytes!("fixtures/binary_div.json"),
        include_bytes!("fixtures/binary_rem.json"),
        include_bytes!("fixtures/binary_shift_right.json"),
        include_bytes!("fixtures/binary_add.json"),
        include_bytes!("fixtures/binary_ne.json"),
    ] {
        let artifact = canonical::parse(bytes).unwrap();
        kuiper_vulkan::inspect::check(&artifact).unwrap();
        kuiper_vulkan::validate_words(&artifact.words).unwrap();
    }
}

#[test]
fn named_foreign_formats_do_not_admit_this_runtime() {
    let mut artifact = fixture();
    artifact.format = "example.binary/1".into();
    kuiper_contracts::validate::artifact(&artifact).unwrap();
    let invocation = kuiper_contracts::Invocation {
        entry: artifact.reflection.entry.clone(),
        workgroups: [1, 1, 1],
        parameters: vec![],
        buffers: vec![],
    };
    assert_eq!(
        kuiper_vulkan::execute(&artifact, &invocation)
            .unwrap_err()
            .code,
        "unsupported-contract"
    );
}
#[test]
fn rejects_an_unknown_required_feature_and_missing_robustness_before_dispatch() {
    for requirements in [
        vec![
            "vulkan.api>=1.2",
            "vulkan.vulkanMemoryModel",
            "vulkan.vulkanMemoryModelDeviceScope",
        ],
        vec![
            "vulkan.api>=1.2",
            "vulkan.robustBufferAccess",
            "vulkan.unknown",
            "vulkan.vulkanMemoryModel",
            "vulkan.vulkanMemoryModelDeviceScope",
        ],
    ] {
        let mut artifact = fixture();
        artifact.requirements = requirements.into_iter().map(str::to_owned).collect();
        let invocation = kuiper_contracts::Invocation {
            entry: artifact.reflection.entry.clone(),
            workgroups: [1, 1, 1],
            parameters: vec![],
            buffers: vec![],
        };
        assert_eq!(
            kuiper_vulkan::execute(&artifact, &invocation)
                .unwrap_err()
                .code,
            "requirements"
        );
    }
}
#[test]
fn self_hashing_does_not_admit_unsupported_opcodes() {
    for op in [
        Op::TypeFloat,
        Op::ControlBarrier,
        Op::FunctionCall,
        Op::ConvertUToF,
    ] {
        let mut artifact = fixture();
        let position = (5..artifact.words.len())
            .find(|p| {
                artifact.words[*p] & 0xffff == Op::TypeInt as u32 && artifact.words[*p] >> 16 == 4
            })
            .unwrap();
        artifact.words[position] = (4 << 16) | op as u32;
        rebind(&mut artifact);
        assert!(kuiper_vulkan::inspect::check(&artifact).is_err());
    }
}
#[test]
fn rejects_reflection_and_local_size_disagreement() {
    let mut artifact = fixture();
    artifact.reflection.local_size[0] = 8;
    assert_eq!(
        kuiper_vulkan::inspect::check(&artifact).unwrap_err().code,
        "spv-entry"
    );
    artifact.reflection.local_size[0] = 4;
    artifact.reflection.parameter_binding += 1;
    assert!(kuiper_vulkan::inspect::check(&artifact).is_err());
}
#[test]
fn rejects_changed_storage_layout_and_missing_read_only_permission() {
    let mut artifact = fixture();
    let mut position = 5;
    let mut stride = false;
    while position < artifact.words.len() {
        let header = artifact.words[position];
        let count = (header >> 16) as usize;
        if header & 0xffff == Op::Decorate as u32
            && count == 4
            && artifact.words[position + 2] == Decoration::ArrayStride as u32
        {
            artifact.words[position + 3] = 8;
            stride = true;
            break;
        }
        position += count;
    }
    assert!(stride);
    rebind(&mut artifact);
    assert!(kuiper_vulkan::inspect::check(&artifact).is_err());
    let mut artifact = fixture();
    let mut position = 5;
    let mut removed = false;
    while position < artifact.words.len() {
        let header = artifact.words[position];
        let count = (header >> 16) as usize;
        if header & 0xffff == Op::Decorate as u32
            && count == 3
            && artifact.words[position + 2] == Decoration::NonWritable as u32
        {
            artifact.words.drain(position..position + count);
            removed = true;
            break;
        }
        position += count;
    }
    assert!(removed);
    rebind(&mut artifact);
    assert_eq!(
        kuiper_vulkan::inspect::check(&artifact).unwrap_err().code,
        "spv-binding"
    );
}
#[test]
fn rejects_atomics_with_unadmitted_scope() {
    let mut artifact = fixture();
    let mut scope = None;
    mutate(&mut artifact, Op::AtomicOr, |args| scope = Some(args[3]));
    let mut position = 5;
    let mut changed = false;
    while position < artifact.words.len() {
        let header = artifact.words[position];
        let count = (header >> 16) as usize;
        if header & 0xffff == Op::Constant as u32
            && count == 4
            && artifact.words[position + 2] == scope.unwrap()
        {
            artifact.words[position + 3] = 2;
            changed = true;
            break;
        }
        position += count;
    }
    assert!(changed);
    rebind(&mut artifact);
    assert!(kuiper_vulkan::inspect::check(&artifact).is_err());
}
#[test]
fn rejects_invalid_instruction_lengths_without_panicking() {
    let mut artifact = fixture();
    artifact.words[5] = 0;
    rebind(&mut artifact);
    assert_eq!(
        kuiper_vulkan::inspect::check(&artifact).unwrap_err().code,
        "spv-instruction"
    );
}
