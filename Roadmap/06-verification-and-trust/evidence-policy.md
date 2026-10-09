# Implement fixed evidence policies

## 1. Separate support from assurance

Support status is unimplemented, experimental, tested or qualified for a named profile/device/driver. Assurance is a separate policy identity. Use the following minimum policies; packages cannot relax them:

| Policy | Minimum claim | Mandatory evidence |
|---|---|---|
| `qualified-v1` | The named execution profile passed qualification; no source/compilation proof implied | Structural/ABI/capability checks, actual test matrix, failures/operational gates and complete recorded assumptions |
| `source-verified-v1` | Identified source obligations were checked, conditional on their preconditions and stated assumptions | Strict checked source closure, no unresolved development admits, exact source/tool/solver identity; O1–O10 dispositions remain visible and no end-to-end claim follows automatically |
| `refinement-verified-v1` | Supported successful/exceptional execution traces refine the specified source/KIR/host relations, under the enumerated platform/foundational assumptions | Required O1–O10 evidence below, plus applicable production qualification if production is claimed |

A source-verified package compiled by an empirically qualified pipeline remains labeled with both facts. It must not be promoted to refinement-verified by changing a display string. Unknown policies/checkers and missing required evidence fail closed.

## 2. Require evidence per obligation

| Obligation | `refinement-verified-v1` requirement | Not an acceptable substitute |
|---|---|---|
| O1 extraction | Mechanized preservation proof or sound per-artifact relation checker over the accepted source subset | Trusted extractor declaration chosen by the package |
| O2 erasure/static data | Checked erasure/specialization correspondence, including retained layout and caller facts | Matching names/IDs alone |
| O3 KIR checks | Defined invariants and soundness of the relevant type/layout/effect/evidence checks | Fuzzing alone |
| O4 representation bridge | Per-operation/control/memory relation for the actual KIR↔worker subset used | Round-trip byte comparison alone |
| O5 passes | Proof or sound validation for each enabled pass/composite under its preconditions | A generic upstream pass name or validator success |
| O6 concurrency | Memory-model and participation refinement for each enabled primitive | Data-race tests alone |
| O7 numerical behavior | Operation/algorithm-specific relation including exceptional values and approximations | Ordinary-input agreement or an FP feature flag |
| O8 runtime | State-machine refinement and implementation correspondence, including failures/unknown outcomes | Driver completion treated as universal success |
| O9 bindings | Checked layout/ownership/marshaling relation, caller and host-import obligations | A matching C signature |
| O10 identity | Canonical digest/provenance verification, policy/checker identity, replay/tamper rejection and checker soundness argument | A producer-supplied `verified` field |

The permitted trust boundary is foundational proof checking/logic/solver assumptions, the identified build/extraction/host compiler for proved implementation code, and the explicitly stated OS/driver/firmware/hardware implementation of the chosen API/memory model. Record exact dependencies and the semantic assumptions made at each boundary. This does not permit declaring the Kuiper extraction algorithm, transformation pipeline, runtime algorithm, or binding algorithm correct without the required O1–O9 relation.

If the implementation cannot meet this policy, expose the weaker accurately described policy. Do not remove the obligation. Adding a new logical axiom or platform assumption changes the policy identity and requires review of affected claims.

## 3. Make the policy executable

Define a policy checker that evaluates required evidence kinds and coverage against every released entrypoint/profile/operation. Checker identity comes from protected admission policy. Verify the artifact, semantic definition, assumptions, checker/toolchain and output digests before accepting a result. Signatures establish provenance; they do not prove a producer's assertion.

Where a checker cannot be rerun at load time, accept only a result issued through the configured trusted verification path, bound to the complete input/output identity and policy. Specify that verification/attestation trust boundary explicitly. An arbitrary package cannot write the same result JSON and become trusted.

Reject a fixture replacing mandatory O1 evidence with an ad hoc trusted-boundary entry. Accept a fixture containing an allowed, explicitly recorded driver assumption only when all other requirements hold. Mutation of a pass, numeric flag, input proof or target artifact invalidates the affected relation.

## 4. Mechanize incrementally without overstating scope

Reuse F*/Pulse for existing source obligations. The official [Pulse overview](https://fstar-lang.org/tutorial/book/pulse/pulse.html) describes the relation to PulseCore; [Pulse basics](https://fstar-lang.org/tutorial/book/pulse/pulse_ch1.html) and [atomic invariants](https://fstar-lang.org/tutorial/book/pulse/pulse_atomics_and_invariants.html) motivate explicit pre/postconditions and concurrent resource reasoning. They do not prove the new GPU memory mapping or compiler bridge.

Start with scalar/layout/straight-line extraction relations, then loops/calls, memory, numerical modes, concurrency, runtime and bindings. Define permitted nondeterminism and exceptional traces. A partial-correctness theorem does not establish progress; state and qualify termination/fairness assumptions separately. Keep each accepted subset and remaining obligation visible throughout P1–P6.
