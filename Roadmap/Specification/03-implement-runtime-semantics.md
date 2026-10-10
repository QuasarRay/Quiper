# S3. Implement resource and runtime transitions

**Deliverable:** a host-plan interpreter, resource manager and asynchronous protocol that refine the same resource/observation relations across bindings and GPU APIs.

## Connect host resources to runtime operations

`Host.host_step` describes allocation, reservation, quiescent resolution and free. `pending_prevents_free` blocks freeing a reserved resource. Generation changes distinguish new resources from stale handles. Each accepted runtime use increments/reserves the relevant host ownership exactly once; each reconciled, quiescent use resolves it exactly once. Prove this cross-module invariant for the concrete registry. Neither a counter nor a driver fence by itself proves the two registries agree.

`Host.copy_relation` specifies snapshot copy and a frame outside the destination. A backend copy with undefined overlapping behavior must prove disjointness or use a qualified temporary snapshot. Empty views preserve specified host effects even if no dispatch is required. Map logical views to actual allocation/binding/mapping/cache footprints and prove `Memory.covers`; `isolated_covers_do_not_overlap` supports independent atom-separated suballocations.

The host plan interprets allocate/view/copy/prepare/submit/wait/release, scalar/control operations and typed imports. Define its program counter, live resource environment, pending calls and explicit cleanup edges. Each edge includes rejection, accepted failure and unknown observation. A plan must not leak a successful postcondition through an error handler. Bound evaluation, allocation and request resources, and specify loop/service progress assumptions.

## Implement success-dependent submission

A request progresses through `Accepted`, `InFlight`, `CompletedUnchecked` and then `Succeeded` or `Failed`. `runnable` requires every semantic dependency to have succeeded. `publishable` requires visibility, a successful guard and established postcondition evidence. These fields are facts the implementation owes evidence for, not flags that a compiler may arbitrarily set.

Use host-gated submission initially. Never queue a consumer that relies on a producer's output while that producer's success is unresolved. Propagate failure through dependency joins; unrelated branches may continue. Retain storage for accepted work until quiescence. `safe_step` strengthens the operation transition with dependency and disposal conditions. Its preservation lemma establishes model retention across every allowed action; the runtime implementation must prove that its actions satisfy those premises.

The operation identity window `(retired, retired+window]` bounds replay records. `may_retire` requires terminal, reconciled entries and no unresolved dependent relying on the retired prefix. Status query is `Observe`, not `Redeem`. Preserve an atomic redemption record and owner identity in the implementation. Out-of-order holes produce backpressure; retired IDs cannot become new work after eviction. Worker death fails the initial session rather than authorizing blind replay.

## Make safe borrowing independent of destructors

Use runtime-owned buffers/copies by default. Safe scopes retain a supervisor registry until `scope_may_return` holds, including when user handles are forgotten or a closure panics. A dropped/forgotten handle is an observation change; it cannot decrement the registered device-use count. A timeout cannot return borrowed memory to the caller while DMA can still access it. `TokenOnly` cannot establish safe pending retention.

Bind safe caller checks to actual retained input bytes/versions with `valid_call`. An immutable descriptor does not prevent mutation of referenced index data. Establish source preconditions through static evidence, a protected dynamic check, caller evidence or an explicit trusted/unsafe route. Host imports have identified contracts and enforcement; a matching function signature cannot supply the proof.

## Map the abstract protocol to each API

For the initial Vulkan package, one owner thread serializes queue/pool/destruction calls. `permitted_host_call` is the model relation; `exclusive_owner` gives uniqueness of permitted callers. The implementation must establish actual thread ownership, callback reentrancy and shutdown ordering. Separate device completion, memory visibility, guard status, safe teardown and host-object synchronization.

Use [M17](../05-runtime-and-interop/01-enforce-submission-and-failure-semantics.md), [M18](../05-runtime-and-interop/02-make-memory-and-bindings-safe.md) and [M19](../05-runtime-and-interop/03-qualify-the-vulkan-service.md) for concrete tables and fault cases. Include lost replies, duplicate requests, forgotten futures, cross-queue joins, guard failure before collectives, device loss, noncoherent atoms and callback/shutdown races. Close O8/O9 only with the state relation and implementation correspondence; model proofs are not Rust lifetime or Vulkan driver proofs.
