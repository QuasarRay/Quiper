# Close dependency, ownership and runtime-host contracts

The roadmap correction is complete. Concrete backend implementation and qualification remain pending. The F* lemmas below concern the specified model; each implementation must establish the correspondence and closure cases.

## V2-01

Consumers remain unsubmitted until all semantic predecessors succeed; failure propagates through joins while accepted resources remain retained.

Implement [the milestone](../../../05-runtime-and-interop/01-enforce-submission-and-failure-semantics.md) against `Runtime.failed_dependency_blocks` in [the specification](../../../Specification/README.md). Preserve [the original finding](../01-runtime-and-ownership/v2-01-dependent-failure.md) as the reason for the change.

**Required implementation evidence:** Fail A before B uses its output as an index; repeat for joins, cross-queue graphs, partial acceptance and collective consumers.

## V2-02

Safe APIs transfer storage to a runtime registry or use a non-escaping supervisor scope; forgetting a handle cannot end retention.

Implement [the milestone](../../../05-runtime-and-interop/02-make-memory-and-bindings-safe.md) against `Host.pending_blocks_scope_exit; Runtime.forgotten_handle_retains` in [the specification](../../../Specification/README.md). Preserve [the original finding](../01-runtime-and-ownership/v2-02-forgotten-borrows.md) as the reason for the change.

**Required implementation evidence:** Forget futures, attempt scope escape, panic/cancel/timeout and lose replies without allowing caller reuse before quiescence.

## V2-03

A bounded acceptance window and contiguous retirement watermark distinguish new out-of-order IDs from permanently retired ones; queries never remint ownership.

Implement [the milestone](../../../05-runtime-and-interop/01-enforce-submission-and-failure-semantics.md) against `Runtime.retired_never_fresh; Runtime.redeemed_once` in [the specification](../../../Specification/README.md). Preserve [the original finding](../01-runtime-and-ownership/v2-03-operation-retirement.md) as the reason for the change.

**Required implementation evidence:** Test stalled holes, replay after retirement, live dependents, duplicate redemption, quotas and generation rollover.

## V2-06

One session owner serializes Vulkan queue, pool, descriptor, mapping and lifetime operations; callbacks use a specified reentrancy path.

Implement [the milestone](../../../05-runtime-and-interop/03-qualify-the-vulkan-service.md) against `Runtime.exclusive_owner` in [the specification](../../../Specification/README.md). Preserve [the original finding](../02-spirt-and-vulkan/v2-06-host-object-synchronization.md) as the reason for the change.

**Required implementation evidence:** Exercise concurrent record/submit/reset/free, callback reentry, shutdown races and error unwinding with host/API diagnostics.
