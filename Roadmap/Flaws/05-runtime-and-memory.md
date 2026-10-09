# 5. Submission, memory ownership, and exceptional behavior

These findings concern cases in which a normal success/failure API is insufficient. The runtime must know what was accepted, what may still execute, which bytes can be touched, and which logical postconditions remain available.

## F19

**IPC submission has no protocol for an unknown acceptance outcome.**

**Severity:** High. **Class:** protocol gap. **Owner:** runtime and verification owners. **Resolve by:** the P1 protocol/state machine; demonstrate in P4 and P8.

**Location:** [runtime IPC](https://github.com/QuasarRay/Quiper/blob/689c4528f227704df989f0f1e3eaabb8ce4b600a/Roadmap/03-backend-extension-contract.md#L40-L48), [runtime state and epochs](../05-runtime-and-interop.md#1-model-execution-before-binding-an-api), and O8.

The roadmap requires partial-submission reporting, exact completion tokens, and safe timeout behavior. It does not define how the client learns whether submission was accepted when the connection fails before the reply. Compiler request IDs do not solve this runtime problem.

Consider a kernel that increments a counter. The worker accepts and submits it, then the reply is lost. Retrying the request as new work can increment twice. Treating the error as rejection can return ownership while the original kernel still runs. Restarting the worker does not establish what happened to the old submission or imported resources.

**Required correction:** define an operation identity scoped to a session/runtime generation, an acceptance point, and an explicit unknown-outcome state. Within a live session, specify duplicate suppression and how a client queries accepted work. Across failure/restart, choose a supported policy: recover from trustworthy state, or fail the affected session and prohibit replay until resource disposition is established. Do not promise transparent exactly-once recovery without implementing the required mechanism.

| Observed result | Safe client interpretation |
|---|---|
| Rejected before acceptance | No submission occurred under the protocol; caller retains the specified resources |
| Accepted with token | Track the exact accepted operation and dependencies |
| Connection lost; acceptance unknown | Do not resubmit as new work or reclaim in-flight ownership automatically |
| Completion confirmed and visibility established | Recover only the successful postconditions covered by that result |
| Session/device failed | Follow the defined failure/poisoning and teardown relation; no successful kernel postcondition |

**Closure:** inject disconnects before acceptance, after acceptance, after driver submission, and after completion but before reply delivery. Use a non-idempotent operation and a dependent chain. Verify duplicate handling, outcome reporting, generation isolation, and buffer lifetime. A valid implementation may fail the session instead of recovering, provided that behavior is explicit and safe.

## F20

**Byte-range ownership omits noncoherent cache-atom overlap.**

**Severity:** High. **Class:** memory-contract gap. **Owner:** runtime, allocator, and verification owners. **Resolve by:** the P1 memory model; qualify in P4.

**Location:** [host visibility and subrange ownership](https://github.com/QuasarRay/Quiper/blob/689c4528f227704df989f0f1e3eaabb8ce4b600a/Roadmap/05-runtime-and-interop.md#L24-L47), [memory capabilities](../03-backend-extension-contract.md#5-capability-negotiation), and O6/O8/O9.

The roadmap requires noncoherent flush/invalidate operations and tracks views and byte ranges. It does not account for synchronization granularity being larger than the logical view.

Vulkan's [`vkFlushMappedMemoryRanges` specification](https://docs.vulkan.org/refpages/latest/refpages/source/vkFlushMappedMemoryRanges.html) makes the whole aligned `nonCoherentAtomSize` region count as accessed for synchronization when a write in that region is flushed. [`VkMappedMemoryRange`](https://docs.vulkan.org/refpages/latest/refpages/source/VkMappedMemoryRange.html) also constrains offset and size alignment. Rounding a driver call alone does not make the larger access safe.

Take a hypothetical 128-byte atom containing two disjoint 64-byte suballocations. The host writes and flushes A while the device writes B. The logical views do not overlap, but the flush's synchronization footprint includes B. A proof over logical byte disjointness alone misses the conflict.

**Required correction:** make the runtime's physical synchronization footprint part of allocation and ownership refinement. Either isolate independently accessed suballocations at the required granularity, serialize conflicting atom accesses, use suitable coherent/staging memory, or implement another justified policy. Include allocation-base offsets, mapping extents, end-of-allocation rules, imported memory where supported, and overflow in range rounding. Expose this through generic memory constraints or a runtime-internal refinement; do not hardcode a Vulkan field into every frontend.

**Closure:** test adjacent views sharing an atom, the same views separated by padding, nonzero allocation offsets, boundary rounding, and concurrent host/device access. Use an allocator/hazard model that can expose the conflict and real noncoherent memory where available. A driver with coherent memory alone cannot qualify this path. The safe case must pass; the conflicting case must be prevented or ordered before driver calls.

## F21

**Assertions and guards need different semantics, including failure at barriers.**

**Severity:** High. **Class:** confirmed source distinction and lowering-contract gap. **Owner:** frontend, concurrency, and verification owners. **Resolve by:** the primitive ledger and P1 operation semantics; enforce in P4/P6.

**Location:** [the device-assertion migration row](https://github.com/QuasarRay/Quiper/blob/689c4528f227704df989f0f1e3eaabb8ce4b600a/Roadmap/04-spirt-and-gpu-lowering.md#L26-L49), [source assumption inventory](../06-verification-and-trust.md#3-existing-assumptions-must-be-inventoried-first), and [failure outcomes](../05-runtime-and-interop.md#1-model-execution-before-binding-an-api).

**Evidence:** [`Kuiper.Assert.fsti`](https://github.com/QuasarRay/Quiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/src/lib/kuiper/Kuiper.Assert.fsti) gives `dassert` a precondition requiring its Boolean to hold. `dguard` has no such precondition and establishes the Boolean on return. [`Kuiper.Assert.fst`](https://github.com/QuasarRay/Quiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/src/lib/kuiper/Kuiper.Assert.fst) admits the latter implementation and discusses nontermination as a possible model. The [extractor](https://github.com/QuasarRay/Quiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/extraction/ExtractKuiper.fst#L594-L616) distinguishes the two. In the [current header](https://github.com/QuasarRay/Quiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/include/kuiper.h#L68-L92), `KPR_GUARD` aborts on failure while `KPR_ASSERT` is removed under `NDEBUG`. [GEMMCPU](https://github.com/QuasarRay/Quiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/src/lib/kernel/gemm/Kuiper.Kernel.GEMMCPU.fst#L159-L161) uses guards for divisibility conditions.

A diagnostic buffer alone is not an implementation of the guard's logical effect. Logging failure and continuing can expose later operations whose preconditions relied on the guard. Erasing a guard like a proved assertion has the same problem. An unchecked frontend also cannot turn an asserted proposition into an optimization assumption without the appropriate contract.

For any proposed device-side guard, terminating one invocation before other invocations reach a required workgroup barrier can violate participation requirements. A portable error path must preserve the applicable collective-control rules or reject that placement. This is a design counterexample, not a claim that the existing GEMMCPU guard was observed to deadlock a GPU.

**Required correction:** give proved assertions, runtime guards, host failures, and device failures separate operation meanings. Define whether failure aborts the session, returns an error, or is unsupported; specify the resulting resource state and suppression of successful postconditions. A checked transformation may eliminate an assertion whose precondition is established. A guard may be removed only with evidence that its successful condition already holds. For device failures, prove collective participation or restrict the supported placement; do not assume an arbitrary early return is safe.

**Closure:** classify all reachable guard/assertion uses and their proof closures. Test true/false conditions in release and diagnostic modes, guarded out-of-bounds accesses, and a proposed device guard before a collective barrier. Verify no forbidden access, fabricated postcondition, or unsupported divergent failure path is accepted. Resolve or explicitly exclude the existing admitted guard dependency under F02's evidence policy.
