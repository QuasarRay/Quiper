# V2-03: duplicate suppression has no bounded retirement protocol

**Severity:** Medium. **Status:** Roadmap corrected in v3; implementation pending. **Evidence class:** protocol-completeness gap. **Owner:** runtime protocol and operations owners. **Resolve by:** P1's operation lifecycle; qualify in P4 and P8.

V3 correction: [implementation and evidence record](../resolutions/01-close-runtime-contracts.md). The original audited evidence below is preserved against v2.

## 1. Affected instructions

The [submission procedure](https://github.com/QuasarRay/Quiper/blob/9dceaf274b46f295f7fc312fb3396d5729d7d97d/Roadmap/05-runtime-and-interop/submission-and-lifecycle.md) adds a live-session operation table and requires repeated IDs to reconcile with the original operation. The [service contract](https://github.com/QuasarRay/Quiper/blob/9dceaf274b46f295f7fc312fb3396d5729d7d97d/Roadmap/03-backend-extension-contract/implementation.md) requires resource budgets. The [production soak](https://github.com/QuasarRay/Quiper/blob/9dceaf274b46f295f7fc312fb3396d5729d7d97d/Roadmap/08-production-acceptance/ci.md) rejects unexplained growing resource leakage.

The [frozen submission procedure](https://github.com/QuasarRay/Quiper/blob/9dceaf274b46f295f7fc312fb3396d5729d7d97d/Roadmap/05-runtime-and-interop/submission-and-lifecycle.md) does not define when a completed record can be discarded, how old retries are rejected after discarding it, or whether repeating a completed reply is observation or another ownership transfer.

## 2. Failure case

Retaining every request, digest and result for an indefinitely live session makes storage grow with the number of operations. Evicting terminal entries by time or an ordinary bounded cache is not a sufficient repair:

1. Request 41 increments a counter and completes. Its reply is delayed or lost.
2. The worker evicts request 41's record to respect its memory budget.
3. An old retry for 41 arrives with the same digest.
4. If absence now means a new request, the counter increments again. If absence means unknown, the client needs a defined reconciliation/failure outcome.

Monotonic allocation of IDs prevents accidental reuse by an honest allocator. It does not, by itself, specify how the server distinguishes an unseen out-of-order request from a retired request. Replaying a completed response must also not create another exclusive ownership capability for the same resource.

These are protocol counterexamples derived from the roadmap's new table, not failures reproduced against a runtime. F19's unknown-acceptance problem is materially improved; the remaining issue is the lifetime of its deduplication evidence.

## 3. Required correction

Define terminal-record retirement and ownership redemption as explicit transitions. Choose a bounded design: acknowledged retirement plus rejection watermarks/tombstones, a bounded request window, or bounded sessions with safe rotation. Specify how it handles out-of-order delivery, multiple outstanding operations, lost acknowledgements and stale generations.

An ID below a retirement boundary must never become executable new work. A record may be retired only when the contract permits the remaining observation/retry behavior. A lost acknowledgement may retain state or cause a defined session outcome; it may not authorize replay. Keep unresolved acceptance and in-flight records protected from ordinary eviction.

Specify that duplicate completion observations refer to the same logical result. The binding/runtime must perform any linear ownership transfer at most once. Publish operation-table capacity, backpressure, rotation cost and failure behavior in the service budget contract.

## 4. Closure evidence

Run more completed operations than the configured retention capacity, then deliver old requests, replies and acknowledgements in adversarial order. Verify bounded retained state, one counter increment, no second ownership redemption and a stable error for retired IDs. Cover wraparound and generation rotation with pending work.

A permanently growing table and a cache that simply forgets completed IDs both fail this closure test. A documented bounded-session policy is acceptable if applications and the soak workload exercise its actual rotation behavior.
