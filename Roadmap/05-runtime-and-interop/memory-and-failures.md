# Implement physical memory footprints and guard failures

## 1. Refine logical views to actual allocations

Keep logical resource identity/offset/extent distinct from allocation, binding, mapping and cache-management footprints. Each backend supplies the required alignment, binding granularity, coherence granularity, mapping rules and ownership-transfer requirements. The common contract expresses these as constraints; vendor SDK types remain in the adapter.

Check offset-plus-length and size multiplication without overflow before narrowing. Track aliased views and outstanding reads/writes across queues and host access. Record allocation-base offsets when views share an underlying allocation. Free only when no permitted asynchronous use remains, or return a defined deferred-release token/policy.

## 2. Enforce noncoherent atom ownership

For noncoherent Vulkan mappings, compute the physical cache footprint using the device's atom size and mapping/allocation bounds. A host write/flush to one byte can synchronize an entire aligned atom. The official [flush specification](https://docs.vulkan.org/refpages/latest/refpages/source/vkFlushMappedMemoryRanges.html) and [mapped-range constraints](https://docs.vulkan.org/refpages/latest/refpages/source/VkMappedMemoryRange.html) establish why disjoint logical views can still conflict.

Default policy: suballocations intended for independent concurrent host/device use receive atom-separated footprints. If padding cannot establish that separation, serialize the conflicting uses or use a justified coherent/staging strategy. Rounding a flush call is necessary but does not itself authorize access to neighboring bytes. Apply equivalent reasoning to invalidate and any supported imported-memory path.

Use checked `round_down(start, granule)` and `round_up(end, granule)` operations, clipping only according to valid allocation-end rules. Validate the computed range against the active mapping. Prove that the reserved permissions cover the expanded footprint before issuing the API call.

Test adjacent 64-byte views within a hypothetical 128-byte atom, padded views, nonzero suballocation bases, allocation tails and arithmetic overflow. A host write/flush concurrent with a device write in the adjacent atom-sharing view must be prevented or ordered. A coherent-only device cannot qualify the noncoherent path.

## 3. Separate assertions from guards

The pinned [Kuiper assertion interface](https://github.com/FStarLang/kuiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/src/lib/kuiper/Kuiper.Assert.fsti) requires `dassert`'s predicate beforehand but makes `dguard` establish it on successful return. The [implementation](https://github.com/FStarLang/kuiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/src/lib/kuiper/Kuiper.Assert.fst) contains an admitted guard model, while [runtime macros](https://github.com/FStarLang/kuiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/include/kuiper.h#L68-L92) distinguish release assertions from always-checked guards.

Define separate KIR operations:

- A proved assertion consumes established evidence and can be erased only under that relation.
- A runtime guard branches into a successful state satisfying the predicate or a specified exceptional state. It cannot log and continue into operations relying on a false predicate.
- An unchecked assertion is not proof evidence. Require checking, an explicit trusted precondition, or rejection under the selected policy.

Host guards may return a structured error under the new host contract; document the difference from legacy process abortion. Device guards require a supported collective-safe failure lowering. If one lane's early exit can break a later barrier or cooperative operation, reject that placement unless a checked transformation preserves participation and suppresses invalid effects. Do not assume a diagnostic buffer supplies these semantics.

After a device failure, mark affected outputs invalid/poisoned until the runtime establishes safe disposal or explicit reinitialization. Driver completion without a successful guard result does not establish the kernel postcondition. Include guard status visibility in the completion relation.

## 4. Required tests and evidence

Test valid/invalid caller obligations, guarded out-of-bounds accesses, true/false guards with diagnostic/release settings, guard placement before collectives, allocation failure, cancellation, timeout, device loss and teardown. Label simulated device-loss evidence separately from hardware observation. O6/O8/O9 cover memory and lifetime refinement; O1/O2/O5 cover preservation or elimination of guard/assertion operations.
