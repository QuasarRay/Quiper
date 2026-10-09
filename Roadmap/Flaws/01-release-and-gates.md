# 1. Release scope and acceptance gates

## F01

**A scoped release can complete while the CUDA replacement objective remains open.**

**Severity:** High. **Class:** scope/gate gap. **Owner:** release and architecture owners. **Resolve by:** G-BASELINE, before committing implementation scope.

**Location:** [README, required result and completion](https://github.com/QuasarRay/Quiper/blob/689c4528f227704df989f0f1e3eaabb8ce4b600a/Roadmap/README.md#L9-L26), [P5/P8](https://github.com/QuasarRay/Quiper/blob/689c4528f227704df989f0f1e3eaabb8ce4b600a/Roadmap/07-implementation-phases.md#L71-L117), and [production scope](../08-production-acceptance.md#1-qualify-explicit-release-profiles).

The roadmap deliberately permits a portable-core release, requires a disposition for every legacy feature, and forbids calling that release full CUDA parity. Those are sound distinctions. The missing commitment is which existing user-visible behaviors must actually work before the requested replacement is considered complete.

G-COVERAGE can pass with difficult rows deferred. G-CUDA-FREE can pass for the new default path while applications still select the legacy package for required behavior. The README then allows the project to be complete for its declared scope. That is a valid intermediate release, but it does not close the broader migration objective. An exhaustive list of deferred features does not resolve this difference.

**Required correction:** separate portable release acceptance from migration completion. Establish a versioned baseline of current entrypoints, host behaviors, supported numerical relations, and customer-relevant features. Mark each as mandatory for replacement, an explicitly accepted retirement, or a later optional feature. Record the rationale and accountable owner. Do not let an implementation team silently turn an unmet mandatory row into an optional one to pass G-COVERAGE.

**Closure:** add a migration-completion gate over that baseline. For every mandatory row, require a qualified CUDA-independent implementation or an explicitly accepted change to the requirement. Include a negative gate fixture in which every advanced row is deferred: the portable release may pass, but migration completion must remain open. Publish both states in the release report.

## F02

**The verified profile has no committed minimum evidence policy.**

**Severity:** High. **Class:** assurance-policy gap. **Owner:** verification owner. **Resolve by:** G-CONTRACT; enforce at G-TRUST.

**Location:** [document 06, obligation policy](https://github.com/QuasarRay/Quiper/blob/689c4528f227704df989f0f1e3eaabb8ce4b600a/Roadmap/06-verification-and-trust.md#L15-L28), [trust record and release report](../06-verification-and-trust.md#7-trusted-computing-base-record), and `milestones.json`'s G-TRUST criterion.

O1–O10 may be discharged or reduced to a named trusted boundary permitted by the profile. The profile's allowed reductions are not fixed. G-TRUST requires complete dispositions and no silent discharge, but does not state which obligations must receive a proof or sound validation rather than an explicit assumption.

A release could therefore name extraction, every compiler pass, the ABI, and the runtime as trusted, provide a complete report, and pass the generic gate. That report would be honest about its assumptions. It would still offer a substantially different assurance level from a release that checks the extraction and runtime relations. The label cannot carry the distinction by itself.

**Required correction:** define versioned evidence policies with a minimum outcome for each obligation: proof, accepted sound validator, or a specifically enumerated trusted boundary. Separate source verification from an end-to-end refinement claim. Give each policy a digest and prevent a package from expanding the allowed trust base through its own manifest. A new exception changes the policy and its advertised assurance.

**Closure:** check a profile/obligation matrix into the specification. Test that replacing a required O1 proof or validator with an arbitrary trusted-boundary declaration fails promotion. Test a legitimately permitted driver assumption separately. The release report must identify exactly which evidence policy passed; it must not merely say G-TRUST passed.

This does not require proving a GPU driver or all machine code. It requires committing to the boundary of the promised result.

## F03

**The final checklist permits exceptions to failure and cleanup tests.**

**Severity:** High. **Class:** contradictory requirements. **Owner:** release owner. **Resolve by:** the next roadmap revision, before any release checklist is used.

**Location:** [document 08, lines 86 and 100–108](https://github.com/QuasarRay/Quiper/blob/689c4528f227704df989f0f1e3eaabb8ce4b600a/Roadmap/08-production-acceptance.md#L86-L108).

The soak requirement excludes unexplained correctness failures, hangs, and growing leakage. The final checklist then permits scoped exceptions to performance budgets and soak/failure/cleanup tests in the same item. The wording extends the performance-waiver mechanism to safety and ownership failures.

For example, a backend that releases an in-flight buffer after a timeout could receive a scoped cleanup exception and still satisfy the checklist literally. Naming an owner or expiry does not make the resulting use-after-free acceptable for the claimed production profile.

**Required correction:** separate performance waivers from correctness and safety gates. Failures involving invalid memory access, false completion, incorrect results, lost ownership, or unsafe cleanup must block the affected qualified profile. A missing test environment can reduce the advertised support scope; it cannot count as passing evidence for that scope. A test that is genuinely inapplicable needs a checked applicability reason, not a waiver of a failing applicable test.

**Closure:** exercise the release evaluator with one waived performance regression, one correctness failure, one cleanup failure, and one unavailable-device result. Only the first may pass under the documented waiver policy. The latter cases must block the affected production claim or remove it explicitly before qualification.

## F04

**Gate results have no specified identity or invalidation record.**

**Severity:** Medium. **Class:** evidence-enforcement gap. **Owner:** tooling and release owners. **Resolve by:** P1's evidence schema; enforce through P7/P8.

**Location:** [P4–P8](https://github.com/QuasarRay/Quiper/blob/689c4528f227704df989f0f1e3eaabb8ce4b600a/Roadmap/07-implementation-phases.md#L59-L117), [milestones.json](../milestones.json), and [evidence integrity](../06-verification-and-trust.md#6-evidence-integrity).

The prose correctly requires hashes, evidence invalidation, and a repeated freeze/test after a required core fix. The machine-readable milestone model records gate IDs, descriptions, and status only. It does not specify the gate-result object that makes those rules enforceable.

P7 can overlap P5/P6. A no-edit exercise may pass on candidate A, then numerical, ABI, or proof changes produce candidate B. A manually retained green G-ADD status is stale even if A's report remains perfectly valid. The existing prose prohibits treating that as sufficient, but the release process has no defined comparison that rejects it.

**Required correction:** keep the planning file distinct from an append-only evidence record. A result needs the gate definition version, core and contract digests, plugin/runtime/binding versions, corpus and harness digests, profile, environment, outcome, skips, evidence location, and applicable waivers. Specify each gate's invalidation dependencies. P8 must resolve results for the exact candidate tuple and refuse inherited statuses with mismatched inputs.

**Closure:** pass a no-edit exercise, change a covered contract/core input, and show that promotion becomes blocked until the required exercise is repeated. Changing an unrelated document should not force a GPU rerun. Retain the original result as evidence for A rather than overwriting its identity.

## F05

**Second-runtime selection has two incompatible deadlines.**

**Severity:** Medium. **Class:** contradictory requirements. **Owner:** architecture owner. **Resolve by:** P0.

**Location:** [document 03, G-ADD-RUNTIME](https://github.com/QuasarRay/Quiper/blob/689c4528f227704df989f0f1e3eaabb8ce4b600a/Roadmap/03-backend-extension-contract.md#L79-L85), [P0 hardware requirements](../07-implementation-phases.md#p0-inventory-the-current-semantics-and-establish-the-baseline), and [P7](../07-implementation-phases.md#p7-prove-the-architecture-is-additive-and-language-independent).

Document 03 assigns selection of the second runtime to P7. P7 says to select it during P0 feasibility work. P0 also requires access to the distinct runtime/API's hardware. These are different dependency plans.

If the choice is deferred to P7, contracts may already depend on Vulkan's model and the hardware procurement problem arrives after the candidate freeze. If it is made in P0, the first profile must already account for the second API's actual portable subset and constraints.

**Required correction:** make P0 responsible for the feasibility decision, hardware commitment, and initial shared workload subset. P7 is the independent implementation and no-edit validation. If P0 cannot establish feasibility, mark that dependency open and revise the plan before freezing the contract.

**Closure:** all references name the same decision milestone. The P0 report identifies the selected API, a viable compiler/runtime route, hardware access, known unsupported semantics, and a workload intersection that still satisfies F01. A fallback API needs the same evidence.

## F06

**Performance budgets do not define an executable pass/fail rule.**

**Severity:** Medium. **Class:** measurement-policy gap. **Owner:** performance and release owners. **Resolve by:** P0 budget ratification.

**Location:** [document 08, performance gates](https://github.com/QuasarRay/Quiper/blob/689c4528f227704df989f0f1e3eaabb8ce4b600a/Roadmap/08-production-acceptance.md#L60-L76).

The proposed budgets name a geometric mean, critical workloads, confidence intervals, noise, and tail latency. They do not define workload weights, the statistic being aggregated, how throughput and latency ratios are normalized, a tail-latency threshold, or what happens when the uncertainty interval crosses a limit. These are material because different reasonable evaluators can produce opposite gate decisions from the same measurements.

For example, `new_latency / baseline_latency` and `baseline_throughput / new_throughput` both make larger values worse. Mixing the latter with its inverse makes gains and regressions cancel incorrectly. A mean-latency budget can also pass while p99 latency is unacceptable. The prose says not to hide that result but gives no enforceable limit.

**Required correction:** ratify the exact corpus, per-case baseline, ratio orientation, weights, critical cases, latency percentiles, absolute limits, and statistical decision procedure before collecting release results. Distinguish pass, fail, and inconclusive. State a bounded remeasurement policy and a hardware-comparison policy. Do not pool unrelated native baselines into a single score without declaring what the score means.

**Closure:** give the evaluator synthetic result sets with a throughput regression, one severe critical-case regression, a tail-only regression, and an uncertainty interval spanning the threshold. Expected outcomes must be fixed in advance. Keep performance waivers separate from F03's nonwaivable safety outcomes.
