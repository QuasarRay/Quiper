use kuiper_contracts::{canonical, validate, *};
use kuiper_core::{host::*, reference};

fn package() -> Package {
    canonical::parse(include_bytes!("../../../validation/fixtures/loop.json")).unwrap()
}
fn invocation(n: u32) -> Invocation {
    Invocation {
        entry: "pretested_loop".into(),
        workgroups: [1, 1, 1],
        parameters: vec![n],
        buffers: vec![BufferArg {
            resource: 0,
            words: vec![99, 0, 0, 0, 0, 88],
            offset: 1,
            length: 4,
        }],
    }
}
#[test]
fn zero_and_positive_loops_preserve_final_carried_values_and_view_frame() {
    for n in [0, 1, 3, 128] {
        let input = invocation(n);
        let result = reference::execute(&package(), &input).unwrap();
        assert_eq!(result.buffers[0].words, vec![99, n, n, n, n, 88]);
        reference::postcheck(
            &validate::for_kernel(&package().kernels[0]),
            &input,
            &result,
        )
        .unwrap();
    }
}
#[test]
fn exhausted_loop_is_a_failed_version() {
    let input = invocation(129);
    let result = reference::execute(&package(), &input).unwrap();
    assert_eq!(result.guard, 4);
    assert!(
        reference::postcheck(
            &validate::for_kernel(&package().kernels[0]),
            &input,
            &result
        )
        .is_err()
    );
}
fn plan(package: &Package) -> HostPlan {
    let operation = |id, name: &str, n, dependencies| HostOperation {
        id,
        entry: "pretested_loop".into(),
        workgroups: [1, 1, 1],
        parameters: vec![n],
        bindings: vec![HostBinding {
            resource: 0,
            buffer: name.into(),
            offset: 1,
            length: 4,
        }],
        dependencies,
    };
    HostPlan {
        schema: "kuiper.host-plan/1".into(),
        parent_digest: canonical::digest(package).unwrap(),
        buffers: vec![
            HostBuffer {
                name: "dependent".into(),
                words: vec![99, 0, 0, 0, 0, 88],
            },
            HostBuffer {
                name: "independent".into(),
                words: vec![77, 0, 0, 0, 0, 66],
            },
        ],
        operations: vec![
            operation(1, "dependent", 129, vec![]),
            operation(2, "dependent", 3, vec![1]),
            operation(3, "independent", 3, vec![]),
        ],
    }
}
#[test]
fn failed_parent_blocks_consumers_but_independent_branch_can_publish() {
    let package = package();
    let result = reference_plan(&package, &plan(&package)).unwrap();
    assert_eq!(
        result
            .steps
            .iter()
            .map(|s| s.status.as_str())
            .collect::<Vec<_>>(),
        vec!["failed", "failed", "succeeded"]
    );
    assert_eq!(result.buffers[0].words, None);
    assert_eq!(result.buffers[0].version, 2);
    assert_eq!(result.buffers[1].words, Some(vec![77, 3, 3, 3, 3, 66]));
}
#[test]
fn unsequenced_writers_and_cycles_are_rejected_before_execution() {
    let package = package();
    let mut plan = plan(&package);
    plan.operations[1].dependencies.clear();
    assert_eq!(
        validate_plan(&package, &plan).unwrap_err().code,
        "host-hazard"
    );
    plan.operations[1].dependencies = vec![1];
    plan.operations[0].dependencies = vec![2];
    assert_eq!(
        validate_plan(&package, &plan).unwrap_err().code,
        "host-cycle"
    );
}
