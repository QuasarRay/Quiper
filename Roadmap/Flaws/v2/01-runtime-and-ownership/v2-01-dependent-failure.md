# V2-01: failed producers do not have a specified execution barrier for dependent kernels

**Severity:** High. **Status:** Open. **Evidence class:** contract composition gap; no GPU reproduction. **Owner:** runtime, host-plan and verification owners. **Resolve by:** P1's dependency semantics; enforce before P4 admits guarded asynchronous chains.

## 1. Affected instructions

The [epoch procedure](../../../05-runtime-and-interop/state-and-epochs.md) permits dependent launches without returning ownership to the host. The [Vulkan completion procedure](../../../05-runtime-and-interop/vulkan-adapter.md) checks device guard status after fence/event completion. The [guard procedure](../../../05-runtime-and-interop/memory-and-failures.md) poisons failed outputs and suppresses a successful postcondition.

Those rules cover the failing operation and host observation. They do not specify how a consumer already submitted to the GPU is prevented from executing when its producer fails. The [submission state machine](../../../05-runtime-and-interop/submission-and-lifecycle.md) also lacks a success-dependent execution transition for such consumers. See the [frozen v2 text](https://github.com/QuasarRay/Quiper/blob/9dceaf274b46f295f7fc312fb3396d5729d7d97d/Roadmap/05-runtime-and-interop/vulkan-adapter.md).

## 2. Evidence and failure case

The pinned [Kuiper launch contract](https://github.com/FStarLang/kuiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/src/lib/kuiper/Kuiper.Kernel.Base.fsti#L18-L38) passes a pledged producer postcondition directly into the next launch's precondition. [Async.Chain](https://github.com/FStarLang/kuiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/src/examples/Kuiper.Example.Async.Chain.fst) exercises this ownership pattern. The new recoverable failure relation must explain what replaces that pledge when success is absent.

Consider a proposed two-dispatch graph:

1. A checks an input and, on success, produces an in-range index. Its supported failure path records a false guard result and leaves its output invalid.
2. B consumes A's promised postcondition and uses that index to access a small buffer. A and B are already queued with the required execution and memory dependencies.
3. A fails. The queue can still execute B before the host reads A's status.
4. Reporting A's failure and poisoning its output after the fence does not undo B's access.

This is a design counterexample, not an observed Kuiper execution. Vulkan [synchronization](https://docs.vulkan.org/spec/latest/chapters/synchronization.html) supplies ordering and visibility; it does not interpret Kuiper's application-defined guard result as a condition for the next kernel's success. A correctly ordered read can still read a semantically invalid output.

## 3. Required correction

Make dependency success part of the host-plan and runtime contract. Distinguish completion-for-lifetime purposes from success sufficient to establish a consumer's precondition. Propagate failure through every affected dependency, including joins, cross-queue edges and partially accepted graphs.

Select an implementable first policy. A host scheduler can withhold dependent dispatch until the producer's successful status is visible. A qualified device-side protocol can condition consumer execution on that status, with the necessary memory ordering and collective-participation argument. Alternatively, reject recoverable device guards in pre-submitted chains until one of these policies exists. Merely adding a status-buffer read after the whole graph finishes is insufficient.

Do not return resources while any already accepted operation can still access them. Failed consumers must receive a defined failure/cleanup outcome without receiving the producer's successful proposition. Bind this relation to O6, O8 and O9.

## 4. Closure evidence

Test A-fails/B-would-access-out-of-bounds, A-succeeds/B-runs, two producers joining at B, an unrelated branch that may continue, and failure on another queue. Include consumers with collectives. Instrument the consumer to show whether it executed; checking only the final returned error is insufficient.

Require a checked dependency/failure relation and tests showing that invalid consumers never perform the prohibited effect. Preserve the successful asynchronous chain's ownership behavior. This extends F21 and F19; their v2 corrections remain useful but do not close this composition case.
