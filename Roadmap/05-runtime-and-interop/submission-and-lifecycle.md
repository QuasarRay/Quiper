# Implement submission outcomes and package lifetimes

## 1. Specify acceptance before implementing transport retries

Runtime identity is `(admitted package instance, device/context, session generation)`. Operation identity adds a monotonically allocated request/submission ID. Reject handles from another instance or generation. Never retry a submission as new work merely because an IPC reply was lost.

Within a live session, keep an operation table with request digest, reserved resource permissions, acceptance state, driver submission/event identity, and completion/failure state. Repeating the same ID and digest returns/query-reconciles the existing operation. The same ID with different contents is a protocol error. Define the acceptance point before returning an accepted token; reserve resources before work can race with a caller's release.

| State | Permitted transition | Resource meaning |
|---|---|---|
| Received/unaccepted | Validate → rejected or accepted | Caller retains ownership until specified transfer; no driver submission before acceptance bookkeeping |
| Rejected | Terminal with rejection reason | No work accepted; specified caller ownership retained |
| Accepted | Submit → in-flight or reported accepted failure | Runtime retains reserved resources and owes an outcome |
| In-flight | Completion, session/device failure, or unknown client observation | No premature host reuse or successful postcondition |
| Completed | Visibility operations → successful result | Recover the specified ownership and postcondition exactly once |
| Failed/poisoned | Safe destruction/reinitialization under the failure relation | No successful kernel postcondition; contents may be invalid |
| Client observation unknown | Query live session or fail session under policy | No blind replay, no inferred rejection or completion |

The first IPC profile need not recover across worker process death. It may fail the session, quarantine affected resources until backend teardown establishes safe disposition, then require a fresh generation. Advertise that limitation. Persistent exactly-once recovery is a later capability requiring its own durable state/reconciliation design. Never infer GPU completion from worker death alone, especially for imported resources.

## 2. Preserve epochs and visibility

Use unique monotonically ordered submission tokens within each queue generation. Dependencies reference exact tokens, not equal integers from unrelated queues. Queue order must be refined to the target's execution and memory dependencies. Return a normal completion witness only after backend completion and required host visibility steps.

Partial graph acceptance returns the accepted prefix/set and its obligations. If the result is unknown, retain the complete potentially affected resource set until reconciliation/failure policy resolves it. Cancellation can reject unaccepted work or stop waiting; it does not promise to stop arbitrary submitted GPU work.

Inject failures before acceptance, after acceptance, after driver submission, and after completion but before reply delivery. Use a counter-increment kernel and dependent launches to expose duplicate execution and false ownership recovery. Test event/counter wraparound as a generation change rather than silently reusing identities.

## 3. Pin active objects to package instances

All resources, pipelines, callbacks, events, dispatch tables and destructors retain the exact admitted package instance. Disable means reject new sessions; it does not unload active code. An upgrade installs a new instance alongside the old one. Existing handles continue to route only to their original instance.

Use reference-counted/otherwise proved ownership of the provider lifetime. Drain or fail old sessions under the defined policy, destroy resources/callback registrations in dependency order, then unload. Keep private libraries and worker executables available until the last permitted owner releases them. Refuse live replacement if that policy cannot be met.

Rollback selects the earlier qualified package for new sessions, invalidates only incompatible cache/evidence identities, and preserves or safely closes existing sessions. Test disable/upgrade/rollback during a kernel, mapping, event wait and callback, including failure to drain. A raw library unload while function pointers remain live is prohibited.

## 4. Define the failure boundary of each transport

IPC compiler/runtime services can isolate process aborts, subject to resource/session failure rules. In-process runtimes expose a documented application-fatal boundary for aborting panics, native crashes or violated provider invariants. Catch recoverable language exceptions before crossing the ABI; do not promise universal recovery through `catch_unwind`. The official [Rust documentation](https://doc.rust-lang.org/std/panic/fn.catch_unwind.html) distinguishes unwinding from aborting panics.

For every operation document resources created, consumed, retained, released or poisoned on each outcome. Test returned errors and caught unwinds, and run abort cases in a controlled subprocess. No error path may mint a successful source postcondition.
