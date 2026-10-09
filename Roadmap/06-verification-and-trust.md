# 06. Verification and trust

## 1. Preserve the claim across every transformation

A source proof and a successfully compiled kernel are not automatically an end-to-end proof. Define the relation between source semantics, extracted KIR, transformed SPIR-T, emitted target code, runtime behavior, and host observation.

For each supported profile, state the intended theorem in terms of observable traces: for a well-formed input satisfying the kernel/host preconditions, successful target executions refine the specified outcomes, preserve required safety properties, and respect the selected numerical relation. Include permitted nondeterminism. State fairness/termination assumptions separately.

Exceptional executions need their own result relation. A device-loss result may establish safe host cleanup without establishing the kernel's successful functional postcondition. Do not derive termination, deadlock freedom, or successful completion from race freedom alone.

## 2. Obligation register

The identifiers below are planned obligations, not existing proved theorems.

| ID | Obligation | Evidence required |
|---|---|---|
| O1 | Frontend extraction preserves executable behavior and the relevant pre/postconditions | Formal refinement for the accepted subset, or a sound checked translation relation |
| O2 | Erasure removes only proof/static content and retains required layout/specialization facts | Erasure relation plus manifest/body consistency checks |
| O3 | KIR typing, layout, effects, ownership, and control-flow checks are sufficient for claimed invariants | Checker specification, soundness argument/proof, negative tests |
| O4 | KIR ↔ SPIR-T conversion preserves values, side effects, memory and control behavior | Per-operation mapping, checked relation and covered control forms |
| O5 | Each enabled transformation preserves the claimed relation | Pass proof or sound translation validation, including side effects and metadata |
| O6 | Barrier/atomic lowering implements the selected concurrency model | Memory-model refinement, participation checks, bounded litmus/model exploration as supporting evidence |
| O7 | Numeric lowering meets the selected exact/approximate contract | Operation-specific evidence and exceptional-value validation |
| O8 | Runtime submission, visibility, epochs, and lifetimes refine the host plan | State-machine refinement and implementation correspondence |
| O9 | Host bindings and ABI marshaling preserve layout and ownership | Layout checks, binding contracts, checked conversions and cross-language tests |
| O10 | Evidence belongs to the exact shipped code, options, dependencies, and device profile | Digest-bound manifests, checker identities, replay and tamper rejection |

For strict verified-profile promotion, an obligation may be discharged or explicitly reduced to a named trusted boundary permitted by that profile. An unexplained admit or a passing test cannot discharge a proof obligation. If a boundary remains empirically qualified only, expose that status in the artifact and release claims.

## 3. Existing assumptions must be inventoried first

Run and improve the existing `scripts/list-admits.py`/`make list-admits` workflow, then classify results. Text search alone is insufficient: inspect `val` declarations, external primitives, solver options, extracted foreign calls, toolchain build modes, and the transitive dependency graph of each released entrypoint.

Distinguish:

- Proved library lemmas.
- Deliberate semantic axioms for devices/external APIs.
- Incomplete development proofs.
- Trusted implementation boundaries.
- Build-time settings that bypass checks.

The current fixed warp contract warning, `SizeT` assumption, shared-memory relations, stream/copy assumptions, and WGMMA numerical relation are explicit audit targets. [Q5–Q9](10-sources.md)

Release gates forbid unresolved development admits in the transitive source proof closure of a claimed verified kernel. Hardware/driver/compiler assumptions remain documented separately; do not hide them by calling them library facts. New assumptions require review and an updated evidence identity.

## 4. Reuse F*/Pulse without tying the interchange to it

Keep current source reasoning in F*/Pulse. Define KIR semantics and key extraction/runtime relations in a proof framework chosen in P1, preferably reusing the existing foundation and libraries. The wire contract carries proposition and semantic identifiers, not F* compiler heap objects.

Other frontends may supply evidence in another system through a checker adapter. A universal automatic translator between proof systems is not required. A checker adapter must identify its logical assumptions and verification coverage. Adding a parser for a proof format is not a soundness proof of the proof language.

Proof automation can search for lemmas or discharge obligations, but only checked proof results count. Never turn a failed goal into an assumption merely to keep the migration moving.

## 5. Translation validation strategy

Use a small, independently specified reference evaluator for the supported deterministic KIR subset. It is valuable for tests and debugging; ordinary execution comparison is not a proof for all inputs.

For transformations with tractable relations, implement translation validators that check each produced artifact. Their soundness is part of the trusted argument. Begin with scalar operations, layouts, index arithmetic, simple control flow, and resource bindings. Extend deliberately to loops, aliasing, and concurrent operations.

For concurrent behavior, evaluate allowed outcome sets and happens-before/visibility relations. A scalar interpreter cannot validate a subgroup barrier or a weak-memory atomic lowering. Use explicit semantics, proof obligations, and bounded model exploration where appropriate; report bounded scope honestly.

Retain the legacy CUDA route for differential tests, and also compare against independent specifications/reference calculations. Agreement between two routes sharing the same extraction error is not independent evidence.

SPIRV-Tools validation is a required format/environment gate, but upstream documents that the validator is incomplete. It cannot establish that emitted code computes the intended function. Vulkan validation layers similarly support API diagnosis, not application functional proofs. [E5](10-sources.md)

## 6. Evidence integrity

Bind evidence to the canonical source/IR digest, semantic profile, operation definitions, compiler pipeline, numerical flags, target requirements, and output artifact. A changed pass sequence or approximation mode invalidates the previous relation unless a new checked relation covers it.

Every pass reports preserved/invalidated facts. Reject an output whose necessary evidence disappeared. Do not attach evidence only to function names or line numbers; rewrites and renaming make those insufficient.

Cache verification results only under the complete proof/checker/toolchain identity. Replaying a previous `.checked` file from a different source or compiler build is not a release verification run. Package seeding may speed local development; release evidence needs validated provenance and a reproducible replay policy.

Unknown checker identities, unsupported proof formats, mismatched digests, unavailable assumptions, or incomplete verification cause verified-profile rejection. Users may explicitly choose an unchecked mode, but it must be a distinct artifact/run policy.

## 7. Trusted computing base record

Publish a per-profile inventory covering the source checker/logic/solver, extraction adapter, unproved compiler passes or validators, serializer, SPIR-T bridge, target compiler/driver, runtime FFI, host binding, OS, firmware, and hardware behavior used in the claim.

Do not describe SPIR-T, Mesa, NVCC, a Vulkan driver, or all of F* as formally verified merely because a Kuiper kernel was checked. Specify what is proved, what is validated per compilation, what is tested, and what is assumed.

Full machine-code correctness is a separate goal unless the project actually implements a proof path through target code generation and the relevant driver/hardware semantics. This roadmap makes that boundary visible rather than using “verified” as a property of the entire software stack.

## 8. Verification release report

For every supported entrypoint/profile pair, publish source-proof status, admitted/axiomatic dependency closure, extraction relation status, enabled pass evidence, runtime/ABI evidence, unsupported features, hardware test identities, numerical policy, and known limitations. Missing evidence must be machine-readable and block a stronger assurance label.

The first release may expose several assurance levels, but the user-selected verified profile must enforce its stated evidence policy at compile/load time. A build flag must never silently downgrade that policy.
