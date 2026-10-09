# Implement candidate-bound gate evaluation

## 1. Keep plans separate from results

`milestones.json` is a plan. Every implementation gate remains `not-run` in this documentation revision. Create an append-only result store when implementing the release evaluator. Never edit a planning status to manufacture qualification.

Use the [gate-result schema](../schemas/gate-result.schema.json) for a result envelope. A record includes the version/digest of the gate definition, its candidate, covered inputs, exact support/evidence profile, outcome, skips, waivers, and replayable evidence. Hash canonical records without the record digest/signature itself. Keep signatures in a separate attestation envelope with a policy-admitted signer identity.

Use domain-separated SHA-256 content identities. `candidate_digest` identifies the release manifest of immutable artifacts and policies, not merely a Git commit. `inputs` is the complete gate-specific dependency closure: core binary, contracts, compiler/pass plan, runtime, binding, frontend/checker, toolchain/patches, corpus, harness, target/device/driver configuration, and relevant environment. The evaluator determines required keys from the gate definition; the producer cannot omit a dependency to broaden validity.

Documentation outside the protected executable/contract/corpus/policy closure does not invalidate GPU results. A commit change alone is therefore insufficient to invalidate or preserve a result: compare the covered content identities. A changed closure always requires rerunning affected gates. A reviewed rule may reuse unaffected results by binding the identical covered inputs to a new release manifest and retaining the original provenance; it cannot copy a green status without that comparison.

## 2. Evaluate each claim explicitly

1. Decode the candidate manifest and requested claim: portable qualification, assurance policy, or full replacement.
2. Resolve mandatory phases/gates, entrypoints, profiles, platforms and feature tuples. G-HOST-CODEGEN is mandatory only when native host-plan compilation is enabled; G-MESA-DECISION concerns the optional experiment.
3. Find results whose gate-definition and required input digests match. Missing, inconclusive, stale, invalid or conflicting evidence blocks promotion. An infrastructure failure is not a test pass.
4. Verify every referenced evidence artifact and admitted checker/attestation. Schema validity and matching digests do not establish semantic correctness.
5. Match skipped cases to the support matrix. An unsupported optional case may be explicitly outside the claim; a skipped mandatory case blocks it.
6. Apply the performance-only waiver rule below. Correctness, proof-policy, ownership, failure, cleanup, required soak and extension gates are nonwaivable.
7. Emit a claim report listing covered rows, accepted evidence and remaining blockers. Require G-REPLACEMENT separately before reporting complete CUDA replacement.

Results for G-ADD-COMPILER/RUNTIME/LANGUAGE/OP must cover the final core, contract readers, protected build/registry files, existing packages and corpus. Changes after P7 trigger the affected exercises before P8. Compiler/runtime substitution and distinct-API execution must both be present for G-ADD-RUNTIME.

## 3. Define invalidation ownership

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

## 4. Bound performance waivers

Permit a waiver only for a performance metric with successful correctness and safety qualification. It identifies exact workload/device/profile/candidate inputs, observed measurement, limit, consequence, responsible role, approval identity, expiration and replacement target. An expired or mismatched waiver blocks promotion. Publish the exception with the support claim. No waiver can remove mandatory replacement coverage or change the meaning of a numerical operation.

## 5. Test the evaluator before trusting it

Feed it a passed extension record for candidate A and then candidate B with a changed contract: B must be blocked. Change an unrelated prose page with the same protected inputs: do not schedule a GPU rerun. Tamper with an artifact, omit a required input, reuse a driver result on another device, skip a mandatory row, waive a cleanup failure, or present an unsigned producer assertion under an attestation policy: reject each case.

These are implementation tests to create. The schema and documentation checks in this revision do not execute this release evaluator.
