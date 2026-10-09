# Risks

## 4. Risk register

| Risk | Consequence | Mitigation / stop condition | Responsible role |
|---|---|---|---|
| SPIR-T APIs or semantics change | Broken integration or stale evidence | Pin revisions, isolate SDK, replay compatibility corpus before upgrade | Compiler owner |
| Pre-erasure information unavailable | Unrecoverable layout/type loss | Typed hook spike before committing to exporter design | Frontend owner |
| Existing proof assumptions are target-specific or incomplete | False portability/verification claims | Transitive assumption inventory, strengthened contracts, block affected verified profile | Verification owner |
| Shader memory/control restrictions reject current kernels | Scope/performance loss | Capability ledger, legalizations with evidence, explicit unsupported diagnostics | Compiler and semantics owners |
| Warp/subgroup mismatch | Hangs or incorrect synchronization | Parameterized contracts plus participation proof; reject unmet constraints | Concurrency owner |
| Floating or matrix semantics differ | Numerically wrong results despite plausible outputs | Per-operation relations, exact/approximate profiles, qualified fallback only | Numerical owner |
| Shared-memory layout changes | Broken alias/alignment proof | Layout refinement and bound checks before lowering | Memory owner |
| Runtime failure model is underspecified | Leaks, reuse of active buffers, false completion | Explicit state machine, device-loss/cancellation tests | Runtime owner |
| Backend SDK requires root build edits | No-edit requirement fails | Immutable-core/out-of-tree installation gate | Architecture owner |
| Extension package claims unsupported purity/proofs | Unsound optimization or verification | Conservative unknown effects, trusted/checkable semantics, digest-bound policy | Verification owner |
| Hardware coverage is missing | Unsupported production claim | Block corresponding qualification; acquire runner or narrow published scope explicitly | Release owner |
| Dynamic/plugin boundaries add overhead | Submission or compile latency regression | Batch/measure; optional stable in-process ABI | Runtime/performance owner |
| Private Mesa interfaces drift | Large ongoing maintenance burden | Version-pinned experimental package; promotion only with measured benefit | Mesa-track owner |
| Evidence/cache identity is incomplete | Old proofs attached to new code | Complete keys, tamper tests, independent replay | Tooling owner |
| Feature parity is declared prematurely | Users lose CUDA-dependent behavior | Exhaustive disposition ledger and separate portable/full-parity release labels | Release owner |

One person can hold several roles. Each unresolved risk needs one accountable owner and an explicit release consequence.
