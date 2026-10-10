# Admit immutable evidence policies through protected configuration

**Milestone M21.** Versioned policy definitions and checker admission that accept new identities without weakening old meanings.

## Required inputs and specification

Start from [M06](../01-target-architecture/02-freeze-portable-artifacts.md), [M20](01-bind-source-and-transformation-evidence.md). Use the exit-gate dependencies in [the implementation plan](../milestones.json); proof and investigation work may begin earlier. Read [the declarative specification](../Specification/README.md) before choosing representation details. This milestone implements `Extension.admitted_policy`, `Extension.acceptable_evidence`.

## Separate support from assurance

Support status is unimplemented, experimental, tested or qualified for a named profile/device/driver. Assurance is a separate policy identity. Use the following minimum policies; packages cannot relax them:

| Policy | Minimum claim | Mandatory evidence |
|---|---|---|
| `qualified-v1` | The named execution profile passed qualification; no source/compilation proof implied | Structural/ABI/capability checks, actual test matrix, failures/operational gates and complete recorded assumptions |
| `source-verified-v1` | Identified source obligations were checked, conditional on their preconditions and stated assumptions | Strict checked source closure, no unresolved development admits, exact source/tool/solver identity; O1–O10 dispositions remain visible and no end-to-end claim follows automatically |
| `refinement-verified-v1` | Supported successful/exceptional execution traces refine the specified source/KIR/host relations, under the enumerated platform/foundational assumptions | Required O1–O10 evidence below, plus applicable production qualification if production is claimed |

A source-verified package compiled by an empirically qualified pipeline remains labeled with both facts. It must not be promoted to refinement-verified by changing a display string. Unknown policies/checkers and missing required evidence fail closed.

## Require evidence per obligation

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

## Make the policy executable

Define a policy checker that evaluates required evidence kinds and coverage against every released entrypoint/profile/operation. Checker identity comes from protected admission policy. Verify the artifact, semantic definition, assumptions, checker/toolchain and output digests before accepting a result. Signatures establish provenance; they do not prove a producer's assertion.

Where a checker cannot be rerun at load time, accept only a result issued through the configured trusted verification path, bound to the complete input/output identity and policy. Specify that verification/attestation trust boundary explicitly. An arbitrary package cannot write the same result JSON and become trusted.

Reject a fixture replacing mandatory O1 evidence with an ad hoc trusted-boundary entry. Accept a fixture containing an allowed, explicitly recorded driver assumption only when all other requirements hold. Mutation of a pass, numeric flag, input proof or target artifact invalidates the affected relation.

## Mechanize incrementally without overstating scope

Reuse F*/Pulse for existing source obligations. The official [Pulse overview](https://fstar-lang.org/tutorial/book/pulse/pulse.html) describes the relation to PulseCore; [Pulse basics](https://fstar-lang.org/tutorial/book/pulse/pulse_ch1.html) and [atomic invariants](https://fstar-lang.org/tutorial/book/pulse/pulse_atomics_and_invariants.html) motivate explicit pre/postconditions and concurrent resource reasoning. They do not prove the new GPU memory mapping or compiler bridge.

Start with scalar/layout/straight-line extraction relations, then loops/calls, memory, numerical modes, concurrency, runtime and bindings. Define permitted nondeterminism and exceptional traces. A partial-correctness theorem does not establish progress; state and qualify termination/fairness assumptions separately. Keep each accepted subset and remaining obligation visible throughout P1–P6.

## Admit new policy identities without modifying installed readers

Implement V2-07 using the v2 result envelope: `profile.assurance` contains `name` and `definition_digest`. Names use a namespace, for example `org.quiper/refinement-verified-v1`. A protected registry binds that exact pair to an immutable policy definition and admitted checker identities. The short policy names above remain readable aliases for these three existing meanings; never mutate their definitions in place.

Adding a policy installs its definition/checker package and an explicitly authorized admission configuration entry. It does not change the core's schema, source enum, workspace, lockfile or the semantics of any old policy. A syntactically valid new name has no authority on its own. Reject unregistered names, mismatched digests, unavailable checkers, unsupported required semantics and self-admission by the producing package. A genuinely new protocol requirement outside the extension envelope needs an explicit compatible reader package or a new contract major version.

Resolve `Extension.admitted_policy` from protected state before evaluating `acceptable_evidence`. Test one admitted fourth policy using the unchanged v2 reader, then reject the same result under an unadmitted registry and with a changed definition digest. Version-1 results retain their version-1 decoder; migration resolves the old alias against its original immutable definition and produces a new canonical record and digest, without altering historical evidence.

## Trusted computing base record

Publish a per-profile inventory covering the source checker/logic/solver, extraction adapter, unproved compiler passes or validators, serializer, SPIR-T bridge, target compiler/driver, runtime FFI, host binding, OS, firmware, and hardware behavior used in the claim.

Do not describe SPIR-T, Mesa, NVCC, a Vulkan driver, or all of F* as formally verified merely because a Kuiper kernel was checked. Specify what is proved, what is validated per compilation, what is tested, and what is assumed.

Full machine-code correctness is a separate goal unless the project actually implements a proof path through target code generation and the relevant driver/hardware semantics. This roadmap makes that boundary visible rather than using “verified” as a property of the entire software stack.
## Verification release report

For every supported entrypoint/profile pair, publish source-proof status, admitted/axiomatic dependency closure, extraction relation status, enabled pass evidence, runtime/ABI evidence, unsupported features, hardware test identities, numerical policy, and known limitations. Missing evidence must be machine-readable and block a stronger assurance label.

The first release may expose several assurance levels, but the user-selected verified profile must enforce its stated evidence policy at compile/load time. A build flag must never silently downgrade that policy.

## Evidence required to close this milestone

Close **G-TRUST, G-ADD-OP** only with the implementation artifacts, positive and negative cases, and source-to-result identities required above. Link the implementation relation to the named F* symbols and the policy's applicable O1–O10 obligations. The specification's proved lemmas are reusable model facts; they do not discharge this implementation correspondence. Record unresolved cases as blockers or explicitly outside the claim. No backend gate is marked passed by this roadmap revision.
