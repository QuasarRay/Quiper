# 2. Map corrections into existing work packages

**Historical audit record.** This describes the audit snapshot, not the current correction status. See [current resolutions](../README/resolutions.md).

## 2. Map corrections into existing work packages

| Existing package/gate | Required addition |
|---|---|
| W01 / G-BASELINE | Distinguish mandatory replacement rows from deferred scope; enumerate guards/assertions and reachable assumptions; add concrete upstream restriction rows |
| W02 / performance ratification | Lock measurement formulas, uncertainty handling, tail/absolute limits, and baseline identities |
| W03 / G-CONTRACT | Specify caller obligations, host import contracts, failure/unknown outcomes, physical access footprints, and evidence-policy requirements |
| W04 / discovery and protocols | Add runtime request acceptance/recovery, package-instance lifetime, and trusted CI admission boundaries |
| W05/W06 / G-EXTRACT | Trace the pinned Pulse erasure boundary and demonstrate correspondence after transformations |
| W07/W08/W09 / compiler validation | Resolve reuse decisions, annotation support, loop-report applicability, and QPtr eligibility; preserve independent oracle responsibilities |
| W10–W14 / G-CONCURRENCY | Test ambiguous submission, noncoherent adjacent views, failure at collective operations, callback obligations, and foreign-caller preconditions |
| W16–W18 / G-COVERAGE and G-TRUST | Qualify exact numeric/annotation paths and bind each obligation to the required evidence policy |
| W19/W20 / G-ADD-* | Add compiler-only and runtime-only substitutions where contracts are compatible; record the complete protected artifact closure |
| W21/W22 / G-PRODUCTION | Enforce trusted hardware jobs, final-candidate evidence identity, nonwaivable safety failures, active-object rollback, and scope-specific claims |

A named role owns each finding in its detailed document. Assign a person when implementation work is scheduled. This audit does not invent staff commitments or delivery dates.
