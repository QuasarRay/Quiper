# 7. Remediation order and closure rules

The findings should change the contracts and acceptance criteria before they become implementation assumptions. A new document saying an issue is understood does not close it. Closure requires the specific decision or evidence described by the finding.

## 1. Work order

| Order | Work | Findings | Required output | Stop condition |
|---|---|---|---|---|
| 1 | Fix release meaning and nonwaivable gates | F01–F03 | Mandatory replacement scope; versioned evidence policies; separate performance waivers | No migration-complete or stronger verification claim while scope/policy is unresolved |
| 2 | Resolve feasibility against actual dependencies | F05, F07–F11, F21 | Second-API decision; annotation/control-flow/QPtr probes; Pulse erasure trace; guard/assertion ledger | Do not freeze representations based on an unsupported or unexamined path |
| 3 | Complete language and host contracts | F12–F14 | Caller-obligation classes; host-service trust policy; interpreter/codegen scope | No safe or verified launch claim without its caller and import obligations |
| 4 | Complete plugin/runtime failure semantics | F16–F17, F19–F21 | Failure states; package-instance lifetimes; ambiguous-submission policy; physical memory footprints; guard failure rules | No candidate v1 freeze while normal failures can lose ownership or permit unsafe replay |
| 5 | Define reproducible acceptance machinery | F04, F06, F15, F18 | Candidate-bound gate records; deterministic performance evaluator; substitution matrix; CI trust admission | No promotion using stale evidence, coupled substitutes, or untrusted qualification artifacts |
| 6 | Repeat qualification on the actual release candidate | All applicable findings | Closure records plus P7/P8 evidence for the final tuple | A finding remains open where the affected release claim lacks evidence |

These steps can overlap where inputs are independent. Their output dependencies remain explicit. Proof and semantics work begins with the contract; it does not wait for P6's exit milestone.

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

## 3. Required closure record

Use one small record per finding, containing:

1. Finding ID, affected profile(s), owner, and decision.
2. Corrected specification/roadmap references and implementation commits where applicable.
3. Exact inputs, tools, contract/profile digests, package versions, and relevant hardware/driver identities.
4. Test/proof/reproduction evidence, including negative cases, failures, skipped scope, and assumptions.
5. What the evidence establishes and what remains excluded.
6. Reviewer and the gate/candidate to which the closure applies.

Statuses should distinguish open, investigating, corrected in specification, implemented, and qualified. An architectural wording correction can close a contradiction. It does not automatically qualify the eventual implementation. Conversely, F08 may close through evidence that the reported issue is inapplicable; a patch is not mandatory if the exclusion is established.

## 4. Smallest useful next review

The first correction PR should commit to F01/F02's scope and evidence policy, remove F03's safety-waiver ambiguity, and align F05/F14's milestones. Those changes make later acceptance decisions reviewable without touching compiler code.

The next work should produce executable feasibility evidence for F07–F11 and the operation distinctions in F21. Use those results to revise the v1 contract and effort ranges. Do not adopt every upstream branch or build a new general IR framework just because it is available.

After the contracts are settled, implement the runtime and independent-substitution tests before calling the boundary frozen. P8 must select evidence for the final release candidate and the same committed workload/evidence policies. A reduced release can be legitimate, but it must remain visibly distinct from completion of the full replacement objective.
