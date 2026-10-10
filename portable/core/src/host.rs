//! Checked DAG execution over owned buffer versions.
use crate::{
    error, reference,
    session::{Handle, Phase, Session},
    worker::Route,
};
use kuiper_contracts::{canonical, validate, *};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HostBuffer {
    pub name: String,
    pub words: Vec<u32>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HostBinding {
    pub resource: u32,
    pub buffer: String,
    pub offset: u32,
    pub length: u32,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HostOperation {
    pub id: u32,
    pub entry: String,
    pub workgroups: [u32; 3],
    pub parameters: Vec<u32>,
    pub bindings: Vec<HostBinding>,
    pub dependencies: Vec<u32>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HostPlan {
    pub schema: String,
    pub parent_digest: String,
    pub buffers: Vec<HostBuffer>,
    pub operations: Vec<HostOperation>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HostStep {
    pub id: u32,
    pub status: String,
    pub diagnostic: Option<Diagnostic>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HostOutput {
    pub name: String,
    pub version: u32,
    pub words: Option<Vec<u32>>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HostResult {
    pub steps: Vec<HostStep>,
    pub buffers: Vec<HostOutput>,
}

fn invocation(
    operation: &HostOperation,
    buffers: &BTreeMap<String, Vec<u32>>,
) -> Result<Invocation> {
    Ok(Invocation {
        entry: operation.entry.clone(),
        workgroups: operation.workgroups,
        parameters: operation.parameters.clone(),
        buffers: operation
            .bindings
            .iter()
            .map(|b| {
                let words = buffers
                    .get(&b.buffer)
                    .ok_or_else(|| error("host-buffer", "binding refers to an absent buffer"))?
                    .clone();
                Ok(BufferArg {
                    resource: b.resource,
                    words,
                    offset: b.offset,
                    length: b.length,
                })
            })
            .collect::<Result<_>>()?,
    })
}
fn precedes(operations: &[HostOperation], first: u32, last: u32, seen: &mut BTreeSet<u32>) -> bool {
    if first == last {
        return true;
    }
    if !seen.insert(last) {
        return false;
    }
    operations
        .iter()
        .find(|op| op.id == last)
        .is_some_and(|op| {
            op.dependencies
                .iter()
                .any(|d| precedes(operations, first, *d, seen))
        })
}
fn kernel<'a>(package: &'a Package, operation: &HostOperation) -> Result<&'a Kernel> {
    package
        .kernels
        .iter()
        .find(|k| k.name == operation.entry)
        .ok_or_else(|| error("host-entry", "plan entry is absent"))
}
fn writes(package: &Package, operation: &HostOperation) -> Result<BTreeSet<String>> {
    let kernel = kernel(package, operation)?;
    Ok(operation
        .bindings
        .iter()
        .filter(|b| {
            kernel
                .resources
                .iter()
                .any(|r| r.id == b.resource && r.access != Access::Read)
        })
        .map(|b| b.buffer.clone())
        .collect())
}
pub fn validate_plan(package: &Package, plan: &HostPlan) -> Result<Vec<usize>> {
    validate::package(package)?;
    if plan.schema != "kuiper.host-plan/1" || plan.parent_digest != canonical::digest(package)? {
        return Err(error("host-parent", "plan is not bound to this package"));
    }
    if plan.operations.is_empty()
        || plan.operations.len() > 64
        || plan.buffers.is_empty()
        || plan.buffers.len() > 64
    {
        return Err(error(
            "host-limit",
            "host plan exceeds the bounded operation/allocation profile",
        ));
    }
    let mut total = 0usize;
    let mut buffers = BTreeMap::new();
    for buffer in &plan.buffers {
        if buffer.name.is_empty()
            || buffer.name.len() > 128
            || buffers
                .insert(buffer.name.clone(), buffer.words.clone())
                .is_some()
        {
            return Err(error(
                "host-buffer",
                "allocation identities must be unique and bounded",
            ));
        }
        total = total
            .checked_add(buffer.words.len())
            .ok_or_else(|| error("host-limit", "allocation size overflow"))?;
        if total > MAX_WORDS {
            return Err(error("host-limit", "aggregate allocation limit exceeded"));
        }
    }
    let mut ids = BTreeSet::new();
    for operation in &plan.operations {
        if operation.id == 0 || !ids.insert(operation.id) {
            return Err(error(
                "host-id",
                "operation identities must be unique and nonzero",
            ));
        }
        if operation
            .bindings
            .iter()
            .map(|b| &b.buffer)
            .collect::<BTreeSet<_>>()
            .len()
            != operation.bindings.len()
        {
            return Err(error(
                "host-alias",
                "this profile requires distinct owned allocations for each binding",
            ));
        }
        validate::invocation(
            &validate::for_kernel(kernel(package, operation)?),
            &invocation(operation, &buffers)?,
        )?;
        validate::work_for_dispatch(kernel(package, operation)?, operation.workgroups)?;
    }
    for operation in &plan.operations {
        if operation
            .dependencies
            .iter()
            .any(|d| *d == operation.id || !ids.contains(d))
            || operation.dependencies.iter().collect::<BTreeSet<_>>().len()
                != operation.dependencies.len()
        {
            return Err(error(
                "host-dependency",
                "dependencies must be unique existing operations",
            ));
        }
    }
    let mut order = Vec::new();
    let mut done = BTreeSet::new();
    while order.len() < plan.operations.len() {
        let next = plan
            .operations
            .iter()
            .enumerate()
            .filter(|(_, op)| {
                !done.contains(&op.id) && op.dependencies.iter().all(|d| done.contains(d))
            })
            .min_by_key(|(_, op)| op.id);
        let Some((index, operation)) = next else {
            return Err(error("host-cycle", "host dependency graph is cyclic"));
        };
        order.push(index);
        done.insert(operation.id);
    }
    for (i, a) in plan.operations.iter().enumerate() {
        let a_writes = writes(package, a)?;
        for b in &plan.operations[i + 1..] {
            let b_writes = writes(package, b)?;
            let conflict = a.bindings.iter().any(|x| {
                b.bindings.iter().any(|y| {
                    x.buffer == y.buffer
                        && (a_writes.contains(&x.buffer) || b_writes.contains(&y.buffer))
                })
            });
            if conflict
                && !precedes(&plan.operations, a.id, b.id, &mut BTreeSet::new())
                && !precedes(&plan.operations, b.id, a.id, &mut BTreeSet::new())
            {
                return Err(error(
                    "host-hazard",
                    "shared writes need an explicit dependency path",
                ));
            }
        }
    }
    Ok(order)
}

fn execute_with<F>(package: &Package, plan: &HostPlan, mut executor: F) -> Result<HostResult>
where
    F: FnMut(&Invocation) -> Result<Execution>,
{
    let order = validate_plan(package, plan)?;
    let mut buffers: BTreeMap<_, _> = plan
        .buffers
        .iter()
        .map(|b| (b.name.clone(), b.words.clone()))
        .collect();
    let mut versions: BTreeMap<_, u32> = plan.buffers.iter().map(|b| (b.name.clone(), 0)).collect();
    let mut poisoned = BTreeSet::new();
    let mut handles = BTreeMap::<u32, Handle>::new();
    let mut session = Session::new()?;
    let mut steps = Vec::new();
    for index in order {
        let operation = &plan.operations[index];
        let input = invocation(operation, &buffers)?;
        let dependencies = operation
            .dependencies
            .iter()
            .map(|d| handles[d])
            .collect::<Vec<_>>();
        let handle = session.accept(canonical::encode(&input)?, &dependencies)?;
        handles.insert(operation.id, handle);
        let changed = writes(package, operation)?;
        for buffer in &changed {
            *versions
                .get_mut(buffer)
                .ok_or_else(|| error("host-buffer", "write allocation missing"))? += 1;
        }
        let result = if dependencies
            .iter()
            .any(|d| session.phase(*d).ok() != Some(Phase::Succeeded))
        {
            Err(error(
                "failed-dependency",
                "a dependent input version was not published successfully",
            ))
        } else {
            session.begin(handle)?;
            match executor(&input) {
                Ok(output) => {
                    session.complete(handle)?;
                    reference::postcheck(
                        &validate::for_kernel(kernel(package, operation)?),
                        &input,
                        &output,
                    )
                    .map(|_| output)
                }
                Err(error) => Err(error),
            }
        };
        match result {
            Ok(output) => {
                session.publish(handle)?;
                for binding in &operation.bindings {
                    if changed.contains(&binding.buffer) {
                        let result = output
                            .buffers
                            .iter()
                            .find(|b| b.resource == binding.resource)
                            .ok_or_else(|| {
                                error("host-result", "checked output resource missing")
                            })?;
                        buffers.insert(binding.buffer.clone(), result.words.clone());
                        poisoned.remove(&binding.buffer);
                    }
                }
                steps.push(HostStep {
                    id: operation.id,
                    status: "succeeded".into(),
                    diagnostic: None,
                });
            }
            Err(diagnostic) => {
                session.fail(handle)?;
                poisoned.extend(changed);
                steps.push(HostStep {
                    id: operation.id,
                    status: "failed".into(),
                    diagnostic: Some(diagnostic),
                });
            }
        }
        // Executors are blocking: no outstanding device access survives return.
        session.redeem(handle, true)?;
    }
    session.retire(session.last_id())?;
    Ok(HostResult {
        steps,
        buffers: buffers
            .into_iter()
            .map(|(name, words)| HostOutput {
                version: versions[&name],
                words: if poisoned.contains(&name) {
                    None
                } else {
                    Some(words)
                },
                name,
            })
            .collect(),
    })
}
pub fn reference_plan(package: &Package, plan: &HostPlan) -> Result<HostResult> {
    execute_with(package, plan, |invocation| {
        reference::execute(package, invocation)
    })
}
pub fn run_plan(package: &Package, plan: &HostPlan, route: &Route) -> Result<HostResult> {
    // Compile and validate every entry before accepting any device operation.
    validate_plan(package, plan)?;
    let mut artifacts = BTreeMap::new();
    for operation in &plan.operations {
        if !artifacts.contains_key(&operation.entry) {
            artifacts.insert(
                operation.entry.clone(),
                route.compile(package, &operation.entry)?,
            );
        }
    }
    execute_with(package, plan, |invocation| {
        route.execute(&artifacts[&invocation.entry], invocation)
    })
}
