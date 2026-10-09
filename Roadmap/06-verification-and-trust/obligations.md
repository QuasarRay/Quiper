# Obligations

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

For `refinement-verified-v1`, O1–O9 require a proof or sound checked translation relation; O10 requires the specified integrity/replay checker and its soundness argument. Only the enumerated foundational/platform assumptions in the evidence policy are permitted. Source verification alone uses the separate `source-verified-v1` label. A package cannot waive an obligation by naming its own trusted boundary. An unexplained admit or a passing test cannot discharge a proof obligation. If a boundary remains empirically qualified only, expose that status in the artifact and release claims.
