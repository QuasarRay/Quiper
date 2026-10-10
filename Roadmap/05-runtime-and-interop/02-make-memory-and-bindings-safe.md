# Make physical memory and asynchronous bindings safe

**Milestone M18.** An allocator and binding ownership argument covering noncoherent footprints, guards and forgotten completion handles.

## Required inputs and specification

Start from [M17](01-enforce-submission-and-failure-semantics.md). Use the exit-gate dependencies in [the implementation plan](../milestones.json); proof and investigation work may begin earlier. Read [the declarative specification](../Specification/README.md) before choosing representation details. This milestone implements `Foundation.subview`, `Memory.covers`, `Host.scope_may_return`, `Runtime.safe_retention`.

## Refine logical views to actual allocations

Keep logical resource identity/offset/extent distinct from allocation, binding, mapping and cache-management footprints. Each backend supplies the required alignment, binding granularity, coherence granularity, mapping rules and ownership-transfer requirements. The common contract expresses these as constraints; vendor SDK types remain in the adapter.

Check offset-plus-length and size multiplication without overflow before narrowing. Track aliased views and outstanding reads/writes across queues and host access. Record allocation-base offsets when views share an underlying allocation. Free only when no permitted asynchronous use remains, or return a defined deferred-release token/policy.

## Enforce noncoherent atom ownership

For noncoherent Vulkan mappings, compute the physical cache footprint using the device's atom size and mapping/allocation bounds. A host write/flush to one byte can synchronize an entire aligned atom. The official [flush specification](https://docs.vulkan.org/refpages/latest/refpages/source/vkFlushMappedMemoryRanges.html) and [mapped-range constraints](https://docs.vulkan.org/refpages/latest/refpages/source/VkMappedMemoryRange.html) establish why disjoint logical views can still conflict.

Default policy: suballocations intended for independent concurrent host/device use receive atom-separated footprints. If padding cannot establish that separation, serialize the conflicting uses or use a justified coherent/staging strategy. Rounding a flush call is necessary but does not itself authorize access to neighboring bytes. Apply equivalent reasoning to invalidate and any supported imported-memory path.

Use checked `round_down(start, granule)` and `round_up(end, granule)` operations, clipping only according to valid allocation-end rules. Validate the computed range against the active mapping. Prove that the reserved permissions cover the expanded footprint before issuing the API call.

Test adjacent 64-byte views within a hypothetical 128-byte atom, padded views, nonzero suballocation bases, allocation tails and arithmetic overflow. A host write/flush concurrent with a device write in the adjacent atom-sharing view must be prevented or ordered. A coherent-only device cannot qualify the noncoherent path.

## Separate assertions from guards

The pinned [Kuiper assertion interface](https://github.com/FStarLang/kuiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/src/lib/kuiper/Kuiper.Assert.fsti) requires `dassert`'s predicate beforehand but makes `dguard` establish it on successful return. The [implementation](https://github.com/FStarLang/kuiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/src/lib/kuiper/Kuiper.Assert.fst) contains an admitted guard model, while [runtime macros](https://github.com/FStarLang/kuiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/include/kuiper.h#L68-L92) distinguish release assertions from always-checked guards.

Define separate KIR operations:

- A proved assertion consumes established evidence and can be erased only under that relation.
- A runtime guard branches into a successful state satisfying the predicate or a specified exceptional state. It cannot log and continue into operations relying on a false predicate.
- An unchecked assertion is not proof evidence. Require checking, an explicit trusted precondition, or rejection under the selected policy.

Host guards may return a structured error under the new host contract; document the difference from legacy process abortion. Device guards require a supported collective-safe failure lowering. If one lane's early exit can break a later barrier or cooperative operation, reject that placement unless a checked transformation preserves participation and suppresses invalid effects. Do not assume a diagnostic buffer supplies these semantics.

After a device failure, mark affected outputs invalid/poisoned until the runtime establishes safe disposal or explicit reinitialization. Driver completion without a successful guard result does not establish the kernel postcondition. Include guard status visibility in the completion relation.

## Required tests and evidence

Test valid/invalid caller obligations, guarded out-of-bounds accesses, true/false guards with diagnostic/release settings, guard placement before collectives, allocation failure, cancellation, timeout, device loss and teardown. Label simulated device-loss evidence separately from hardware observation. O6/O8/O9 cover memory and lifetime refinement; O1/O2/O5 cover preservation or elimination of guard/assertion operations.

## Resource safety

- Record allocation extent, alignment, memory kind, device, generation, and outstanding uses.
- Check slice arithmetic and byte-size multiplication for overflow before calling the driver.
- Keep pinned/borrowed host buffers alive until all asynchronous uses finish; document whether the runtime copies or borrows host input.
- Define aliasing permissions across views and across bindings. A safe Rust wrapper cannot infer uniqueness merely from an opaque C handle.
- Validate launch arguments against reflection and the artifact ABI, including offsets, lengths, strides, scalar encodings, and specialization values.
- Defer freeing resources still in use or reject the operation predictably; specify the policy and its memory costs.
- Define callbacks, thread affinity, thread safety, reentrancy, process teardown, and runtime shutdown. Never call application code while holding an undocumented global runtime lock.
- Preserve explicit device selection. An allocation must not silently move to a fallback device with different capability or visibility assumptions.

The baseline uses logical buffers. Unified virtual addressing, sparse allocations, device-address pointers, peer copies, external memory, and RDMA are separate capability profiles; their coherence and lifetime guarantees require their own evidence.
## Host ABI and bindings

The C-compatible ABI exposes opaque handles, fixed-width values, explicit lengths, versioned descriptors, and status/error retrieval. Specify who allocates/frees every returned object and the lifetime of error strings. Avoid C `long`, platform-dependent enums, implicit struct padding, and unspecified allocator sharing.

Generate bindings from the same contract wherever practical. C and Rust bindings must load identical device packages and exercise the same host plans. The Rust binding should expose safe ownership wrappers over a small reviewed unsafe boundary. Other languages can use IPC, C FFI, or a native implementation of the protocol.

Include asynchronous APIs with explicit completion objects, cancellation semantics, and error propagation. Cancellation can prevent future submission or stop waiting; it must not falsely promise arbitrary already-submitted GPU work has stopped. Provide bounded batching to control IPC overhead.

The host-plan interpreter supplies a portable implementation. Optional native host code generation is another plugin role. It must preserve the same operation order, effects, and failure cleanup. A generated C header is one binding, not the canonical program representation.

## Make asynchronous ownership sound when handles are forgotten

Implement V2-02 with owned buffers or retained copies in the initial safe API. Submission transfers the backing allocation into the runtime's session registry before the device can use it. A completion handle observes that entry; dropping or forgetting it cannot remove the registry's ownership. Returning a result transfers ownership exactly once after quiescence and visibility. Bound unreclaimed storage with quotas and backpressure.

A safe borrowed API requires a non-escaping scope. Its supervisor owns every outstanding registration and drains all submitted uses before returning to the caller, including when individual futures are forgotten or the closure unwinds. Handles cannot escape the scope's lifetime. A timeout may fail the wait but cannot return from the scope while DMA still borrows caller memory. If quiescence cannot be established, the safe scope cannot resume the caller with access to that storage; document the blocking or process-failure policy. Do not implement a scope using only each handle's destructor.

`Pin<&mut [T]>`, a lifetime parameter and `Drop::wait` are insufficient: safe code may call `mem::forget`. See the official [forget contract](https://doc.rust-lang.org/std/mem/fn.forget.html), [leaking discussion](https://doc.rust-lang.org/nomicon/leaking.html) and [scoped thread contract](https://doc.rust-lang.org/std/thread/fn.scope.html). A lower-level caller-managed borrowing route must be explicitly unsafe with lifetime, aliasing and quiescence obligations; it is not the safe default.

Implement `Runtime.safe_retention` and preserve it on handle observation/forgetting. Test forgetting a future followed by attempted buffer mutation/free, dropping a parent future, panic in a scope, cancellation, timeout, lost IPC reply, session failure and a forgotten final handle at shutdown. Compile-fail tests must cover attempted scope escape. A memory-safety claim requires the implementation's ownership argument, not only these tests.

## Evidence required to close this milestone

Close **G-CONCURRENCY, O6, O9** only with the implementation artifacts, positive and negative cases, and source-to-result identities required above. Link the implementation relation to the named F* symbols and the policy's applicable O1–O10 obligations. The specification's proved lemmas are reusable model facts; they do not discharge this implementation correspondence. Record unresolved cases as blockers or explicitly outside the claim. No backend gate is marked passed by this roadmap revision.
