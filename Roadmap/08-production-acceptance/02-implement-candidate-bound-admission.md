# Implement candidate-bound CI and release admission

**Milestone M26.** A protected evaluator that rejects stale, missing, self-authorized, skipped or mismatched evidence.

## Required inputs and specification

Start from [M21](../06-verification-and-trust/02-admit-immutable-evidence-policies.md), [M25](01-qualify-the-correctness-matrix.md). Use the exit-gate dependencies in [the implementation plan](../milestones.json); proof and investigation work may begin earlier. Read [the declarative specification](../Specification/README.md) before choosing representation details. This milestone implements `Extension.admitted_policy`, `Extension.acceptable_evidence`, `Qualification.qualified`.

## Keep plans separate from results

`milestones.json` is a plan. Every implementation gate remains `not-run` in this documentation revision. Create an append-only result store when implementing the release evaluator. Never edit a planning status to manufacture qualification.

Use the [gate-result schema](../schemas/gate-result.schema.json) for a result envelope. A record includes the version/digest of the gate definition, its candidate, covered inputs, exact support/evidence profile, outcome, skips, waivers, and replayable evidence. Hash canonical records without the record digest/signature itself. Keep signatures in a separate attestation envelope with a policy-admitted signer identity.

Use domain-separated SHA-256 content identities. `candidate_digest` identifies the release manifest of immutable artifacts and policies, not merely a Git commit. `inputs` is the complete gate-specific dependency closure: core binary, contracts, compiler/pass plan, runtime, binding, frontend/checker, toolchain/patches, corpus, harness, target/device/driver configuration, and relevant environment. The evaluator determines required keys from the gate definition; the producer cannot omit a dependency to broaden validity.

Documentation outside the protected executable/contract/corpus/policy closure does not invalidate GPU results. A commit change alone is therefore insufficient to invalidate or preserve a result: compare the covered content identities. A changed closure always requires rerunning affected gates. A reviewed rule may reuse unaffected results by binding the identical covered inputs to a new release manifest and retaining the original provenance; it cannot copy a green status without that comparison.

## Evaluate each claim explicitly

1. Decode the candidate manifest and requested claim: portable qualification, assurance policy, or full replacement.
2. Resolve mandatory phases/gates, entrypoints, profiles, platforms and feature tuples. G-HOST-CODEGEN is mandatory only when native host-plan compilation is enabled; G-MESA-DECISION concerns the optional experiment.
3. Find results whose gate-definition and required input digests match. Missing, inconclusive, stale, invalid or conflicting evidence blocks promotion. An infrastructure failure is not a test pass.
4. Verify every referenced evidence artifact and admitted checker/attestation. Schema validity and matching digests do not establish semantic correctness.
5. Match skipped cases to the support matrix. An unsupported optional case may be explicitly outside the claim; a skipped mandatory case blocks it.
6. Apply the performance-only waiver rule below. Correctness, proof-policy, ownership, failure, cleanup, required soak and extension gates are nonwaivable.
7. Emit a claim report listing covered rows, accepted evidence and remaining blockers. Require G-REPLACEMENT separately before reporting complete CUDA replacement.

Results for G-ADD-COMPILER/RUNTIME/LANGUAGE/OP must cover the final core, contract readers, protected build/registry files, existing packages and corpus. Changes after P7 trigger the affected exercises before P8. Compiler/runtime substitution and distinct-API execution must both be present for G-ADD-RUNTIME.

## Define invalidation ownership

| Change | Required reevaluation |
|---|---|
| Source/exporter/erasure relation | G-EXTRACT, affected O1/O2 and all dependent compiled artifacts |
| KIR/ABI/protocol semantics or core routing | G-CONTRACT, affected proof/runtime checks and extension exercises |
| SPIR-T pin, patch, pass or numeric policy | Compiler corpus, O4–O7, affected hardware/performance and compiler substitution |
| Runtime/allocator/binding | O6/O8/O9, lifetime/failure/hardware and relevant substitution exercises |
| Device, driver or enabled feature configuration | Affected hardware/numeric/performance qualification and platform assumptions |
| Proof checker/admission policy | Re-evaluate accepted evidence and all claims relying on the changed policy |
| Corpus, harness, metric or threshold | Rerun/evaluate exactly as the versioned gate specifies; never compare incompatible scores |

Every gate definition lists its inputs, evaluator version, required cases, allowed skips, and invalidation rule. Transitive dependence matters: an unchanged runtime result does not rescue a kernel artifact changed by a compiler upgrade.

## Bound performance waivers

Permit a waiver only for a performance metric with successful correctness and safety qualification. It identifies exact workload/device/profile/candidate inputs, observed measurement, limit, consequence, responsible role, approval identity, expiration and replacement target. An expired or mismatched waiver blocks promotion. Publish the exception with the support claim. No waiver can remove mandatory replacement coverage or change the meaning of a numerical operation.

## Test the evaluator before trusting it

Feed it a passed extension record for candidate A and then candidate B with a changed contract: B must be blocked. Change an unrelated prose page with the same protected inputs: do not schedule a GPU rerun. Tamper with an artifact, omit a required input, reuse a driver result on another device, skip a mandatory row, waive a cleanup failure, or present an unsigned producer assertion under an attestation policy: reject each case.

These are implementation tests to create. The schema and documentation checks in this revision do not execute this release evaluator.

## Separate discovery from execution

Untrusted changes may undergo bounded manifest/schema/static checks in an isolated, unprivileged environment. A manifest requests a role and a capability class; it cannot choose arbitrary commands, runner labels, secrets, signing rights or network privileges.

Keep an administrator-controlled admission file outside contributed package authority. It maps reviewed package/commit identities to runner classes, allowed test entrypoints, budgets and artifact destinations. Generic CI reads that policy and generates work without vendor-specific source dispatch. Adding a policy/configuration entry is allowed by the additive extension contract; editing central workflow logic is not.

Bind review/admission to exact source/dependency/harness digests. A new push invalidates admission for the changed executable closure. Avoid privileged jobs that check out and execute an unreviewed PR head. Follow GitHub's official [secure-use guidance for self-hosted runners](https://docs.github.com/en/actions/reference/security/secure-use#hardening-for-self-hosted-runners).

## Qualify in a clean environment

Use disposable/reimaged GPU hosts or a documented, tested isolation policy with equivalent containment for the admitted workload. Clear relevant caches, pin drivers and harnesses, restrict credentials, and record runner provenance. A container alone does not establish isolation from its privileged GPU driver or the host.

Separate test execution from release signing. Only the protected evaluator can accept qualification evidence and request an attestation. Do not give tested backend code a signing credential. Treat evidence uploaded by a package as an untrusted claim until its provenance and required checks are validated.

## Test the boundary

Reject manifests that request a privileged label, inject a command through a label/path, change code after admission, or forge a passed qualification report. Exercise driver reset, worker crash and interrupted cleanup. Quarantine/reimage a contaminated runner before another qualification job; distinguish that infrastructure failure from a behavioral test result.

The implementation may use GitHub Actions or another executor. The contract is the same: package discovery is extensible data handling, while execution authority comes from independent policy.

## CI and evidence gates

Create separate required jobs for contract conformance, strict source verification, extraction, compiler validation, runtime/binding tests, backend qualification, no-edit installation, packaging, and release provenance. Generic backend discovery requests the relevant matrix from manifests; protected admission policy maps reviewed commit identities to permitted runner classes. Package metadata cannot grant execution or signing authority.

Every skipped feature test must state the missing capability and be matched against the claimed support matrix. Skipped tests cannot count as evidence for a supported feature. Hardware failures, unavailable runners, and compiler errors remain distinct results.

Keep CPU-only checks fast enough for ordinary changes. Run GPU regression jobs on affected profile/backend combinations, then the complete claimed release matrix for a release candidate. Isolate resource-intensive fuzzing/soak tests from deterministic per-change checks. Cached proof/build outputs require complete identity checks.

Before release, run at least a 24-hour sustained workload/queue/resource-lifecycle soak on representative qualified devices, with no unexplained correctness failures, hangs, or growing resource leakage. This is a minimum proposed qualification exercise, not proof of indefinite reliability.

## Evidence required to close this milestone

Close **G-TRUST, G-PRODUCTION** only with the implementation artifacts, positive and negative cases, and source-to-result identities required above. Link the implementation relation to the named F* symbols and the policy's applicable O1–O10 obligations. The specification's proved lemmas are reusable model facts; they do not discharge this implementation correspondence. Record unresolved cases as blockers or explicitly outside the claim. No backend gate is marked passed by this roadmap revision.
