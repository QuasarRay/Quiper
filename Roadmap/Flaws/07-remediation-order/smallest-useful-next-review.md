# 4. Smallest useful next review

**Historical audit record.** This describes the audit snapshot, not the current correction status. See [current resolutions](../README/resolutions.md).

## 4. Smallest useful next review

The first correction PR should commit to F01/F02's scope and evidence policy, remove F03's safety-waiver ambiguity, and align F05/F14's milestones. Those changes make later acceptance decisions reviewable without touching compiler code.

The next work should produce executable feasibility evidence for F07–F11 and the operation distinctions in F21. Use those results to revise the v1 contract and effort ranges. Do not adopt every upstream branch or build a new general IR framework just because it is available.

After the contracts are settled, implement the runtime and independent-substitution tests before calling the boundary frozen. P8 must select evidence for the final release candidate and the same committed workload/evidence policies. A reduced release can be legitimate, but it must remain visibly distinct from completion of the full replacement objective.
