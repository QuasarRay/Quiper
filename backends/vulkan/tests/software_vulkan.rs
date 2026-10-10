//! These integration cases require a Vulkan 1.2 device and SPIRV-Tools.
//! They are run explicitly with llvmpipe; no physical-GPU qualification is claimed.
use kuiper_contracts::{Artifact, BufferArg, Invocation, canonical};
fn fixture(bytes: &[u8]) -> Artifact {
    canonical::parse(bytes).unwrap()
}
fn buffer(resource: u32, words: Vec<u32>, offset: u32, length: u32) -> BufferArg {
    BufferArg {
        resource,
        words,
        offset,
        length,
    }
}
fn invocation(a: &Artifact, parameters: Vec<u32>, buffers: Vec<BufferArg>) -> Invocation {
    Invocation {
        entry: a.reflection.entry.clone(),
        workgroups: [1, 1, 1],
        parameters,
        buffers,
    }
}

#[test]
#[ignore = "requires explicit software Vulkan qualification run"]
fn vector_add_preserves_owned_views_tails_and_argument_order() {
    let a = fixture(include_bytes!("fixtures/guarded-vector-add.json"));
    let mut i = invocation(
        &a,
        vec![],
        vec![
            buffer(2, vec![777; 9], 2, 5),
            buffer(1, vec![999, 10, 20, 30, 40, 50, 888], 1, 5),
            buffer(0, vec![111, 1, 2, 3, 4, 5, 222], 1, 5),
        ],
    );
    i.workgroups = [2, 1, 1];
    let e = kuiper_vulkan::execute(&a, &i).unwrap();
    assert_eq!(e.guard, 0);
    assert_eq!(
        e.buffers[0].words,
        vec![777, 777, 11, 22, 33, 44, 55, 777, 777]
    );
    assert_eq!(e.buffers[1], i.buffers[1]);
    assert_eq!(e.buffers[2], i.buffers[2]);
    assert_eq!(
        e.buffers.iter().map(|b| b.resource).collect::<Vec<_>>(),
        vec![2, 1, 0]
    );
    assert!(!e.device.is_empty());
}
#[test]
#[ignore = "requires explicit software Vulkan qualification run"]
fn empty_views_keep_their_backing_words() {
    let a = fixture(include_bytes!("fixtures/guarded-vector-add.json"));
    let i = invocation(
        &a,
        vec![],
        vec![
            buffer(0, vec![7], 0, 0),
            buffer(1, vec![8], 0, 0),
            buffer(2, vec![9], 0, 0),
        ],
    );
    assert_eq!(kuiper_vulkan::execute(&a, &i).unwrap().buffers, i.buffers);
}
#[test]
#[ignore = "requires explicit software Vulkan qualification run"]
fn checked_access_failure_does_not_publish_partial_buffers() {
    let a = fixture(include_bytes!("fixtures/checked-load.json"));
    for lengths in [(2, 4), (4, 2)] {
        let i = invocation(
            &a,
            vec![],
            vec![
                buffer(0, vec![1, 2, 3, 4], 0, lengths.0),
                buffer(1, vec![99; 4], 0, lengths.1),
            ],
        );
        let original = i.clone();
        assert_eq!(
            kuiper_vulkan::execute(&a, &i).unwrap_err().code,
            "guard-failed"
        );
        assert_eq!(i, original);
    }
}
#[test]
#[ignore = "requires explicit software Vulkan qualification run"]
fn pretested_loops_cover_zero_positive_and_exhausted_budgets() {
    let a = fixture(include_bytes!("fixtures/pretested-loop.json"));
    for n in [0, 1, 3, 128] {
        let i = invocation(&a, vec![n], vec![buffer(0, vec![77, 0, 0, 0, 0, 88], 1, 4)]);
        assert_eq!(
            kuiper_vulkan::execute(&a, &i).unwrap().buffers[0].words,
            vec![77, n, n, n, n, 88]
        );
    }
    let i = invocation(&a, vec![129], vec![buffer(0, vec![0; 4], 0, 4)]);
    let error = kuiper_vulkan::execute(&a, &i).unwrap_err();
    assert_eq!(error.code, "guard-failed");
    assert!(error.message.contains("00000004"));
}
#[test]
#[ignore = "requires explicit software Vulkan qualification run"]
fn nested_loops_return_the_updated_accumulator() {
    let a = fixture(include_bytes!("fixtures/nested-loops.json"));
    for (n, m) in [(0, 3), (3, 0), (1, 1), (2, 3)] {
        let i = invocation(&a, vec![n, m], vec![buffer(0, vec![0; 4], 0, 4)]);
        assert_eq!(
            kuiper_vulkan::execute(&a, &i).unwrap().buffers[0].words,
            vec![n * m; 4]
        );
    }
}
#[test]
#[ignore = "requires explicit software Vulkan qualification run"]
fn condition_effects_execute_once_at_each_source_test() {
    let a = fixture(include_bytes!("fixtures/condition-effects.json"));
    for n in [0, 1, 3, 128] {
        let i = invocation(
            &a,
            vec![n],
            vec![buffer(0, vec![7, 0, 9], 1, 1), buffer(1, vec![0; 4], 0, 4)],
        );
        let e = kuiper_vulkan::execute(&a, &i).unwrap();
        assert_eq!(e.buffers[0].words, vec![7, 4 * (n + 1), 9]);
        assert_eq!(e.buffers[1].words, vec![n; 4]);
    }
}
#[test]
#[ignore = "requires explicit software Vulkan qualification run"]
fn atomic_add_respects_its_owned_offset_and_empty_view_guard() {
    let a = fixture(include_bytes!("fixtures/atomic-add.json"));
    let mut i = invocation(&a, vec![], vec![buffer(0, vec![91, 10, 99], 1, 1)]);
    i.workgroups = [4, 1, 1];
    assert_eq!(
        kuiper_vulkan::execute(&a, &i).unwrap().buffers[0].words,
        vec![91, 26, 99]
    );
    i.buffers[0].length = 0;
    assert_eq!(
        kuiper_vulkan::execute(&a, &i).unwrap_err().code,
        "guard-failed"
    );
}

#[test]
#[ignore = "requires explicit software Vulkan qualification run"]
fn arithmetic_and_boolean_edges_preserve_the_integer_contract() {
    let cases = [
        (
            include_bytes!("fixtures/binary_div.json").as_slice(),
            vec![(-7i32) as u32, 3],
            (-2i32) as u32,
        ),
        (
            include_bytes!("fixtures/binary_rem.json"),
            vec![(-7i32) as u32, 3],
            (-1i32) as u32,
        ),
        (
            include_bytes!("fixtures/binary_shift_right.json"),
            vec![(-7i32) as u32, 1],
            (-4i32) as u32,
        ),
        (
            include_bytes!("fixtures/binary_add.json"),
            vec![u32::MAX, 1],
            0,
        ),
        (include_bytes!("fixtures/binary_ne.json"), vec![0, 1], 1),
        (include_bytes!("fixtures/binary_ne.json"), vec![1, 1], 0),
    ];
    for (bytes, parameters, expected) in cases {
        let a = fixture(bytes);
        let i = invocation(&a, parameters, vec![buffer(0, vec![0; 4], 0, 4)]);
        assert_eq!(
            kuiper_vulkan::execute(&a, &i).unwrap().buffers[0].words,
            vec![expected; 4]
        );
    }
    for bytes in [
        include_bytes!("fixtures/binary_div.json").as_slice(),
        include_bytes!("fixtures/binary_rem.json"),
    ] {
        let a = fixture(bytes);
        for parameters in [vec![1, 0], vec![i32::MIN as u32, (-1i32) as u32]] {
            let i = invocation(&a, parameters, vec![buffer(0, vec![0; 4], 0, 4)]);
            let error = kuiper_vulkan::execute(&a, &i).unwrap_err();
            assert_eq!(error.code, "guard-failed");
            assert!(error.message.contains("00000002"));
        }
    }
    let a = fixture(include_bytes!("fixtures/binary_shift_right.json"));
    let i = invocation(&a, vec![1, 32], vec![buffer(0, vec![0; 4], 0, 4)]);
    assert_eq!(
        kuiper_vulkan::execute(&a, &i).unwrap_err().code,
        "guard-failed"
    );
}
