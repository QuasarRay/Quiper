# 09. Work packages, decisions, and risks

## 1. Suggested review units

These are implementation work packages, not automatically created GitHub issues or commitments by named people. Each change should include its contract, implementation, focused tests, evidence impact, and rollout effect. Use stacked changes when dependencies are real; avoid one branch that rewrites the entire system before it can be exercised.

| ID | Work package | Existing touchpoints | Acceptance evidence |
|---|---|---|---|
| W01 | Primitive/host/evidence inventory | `extraction/`, `src/`, `include/`, generated instantiations | Every extraction/default/foreign case classified |
| W02 | Reproducible baseline | `verify.mk`, `nvcc.mk`, tests/benchmarks/packaging | Pinned logs and workload results |
| W03 | KIR grammar, canonical encoding, semantic profiles | New `contracts/`, semantic specifications | Independent reader agreement; invalid inputs rejected |
| W04 | Discovery and compiler/runtime protocols | Build entrypoints plus new generic orchestrator | Synthetic package installation without registration edits |
| W05 | Typed F* manifest capture | Pinned F* hook and `extraction/` | Static/ghost/runtime classification survives erasure |
| W06 | F* executable KIR export | Current primitive handlers | No CUDA names in the contract; baseline corpus extracts |
| W07 | KIR reference semantics and checker | New validation tools | Specified supported subset and independent test oracle |
| W08 | SPIR-T construction SDK | New `compiler/spirt/` | Typed/control/layout/evidence mapping tests |
| W09 | SPIR-V target emitter | SPIR-T adapter, target validator | Explicit environment, valid modules, reflection agreement |
| W10 | Vulkan runtime basics | New backend package | Real allocation/dispatch/result/cleanup path |
| W11 | Shared memory and layout | `Kuiper.SHMem`, array/views, current helpers | Alignment/layout refinement plus limit/overflow tests |
| W12 | Barriers, subgroups, atomics | Barrier/AtomicOps contracts, runtime | Participation and memory-model obligations resolved |
| W13 | Host plans, queues, epochs | `Kernel.Base`, streams/epochs, async examples | Ordered dependency chains and failure ownership |
| W14 | C/Rust host bindings | New binding packages | Same artifact bytes; lifetime/error/ABI tests |
| W15 | Second source adapter | Independent restricted typed frontend | KIR contract conformance; correct assurance labeling |
| W16 | Numeric policy and math | Float modules and extraction mappings | Exact/approximate tests and evidence per operation |
| W17 | Matrix/vendor extensions | TensorCore/WGMMA interfaces and headers | Qualified shapes/layouts/relations, explicit unsupported cases |
| W18 | Pass/evidence validation | All transformation boundaries | O1–O10 evidence and invalidation behavior |
| W19 | Distinct out-of-tree backend | Independent compiler/runtime package | Frozen-core hashes unchanged; real workload runs |
| W20 | Additive frontend/operation exercise | Public SDK and contracts only | No existing source or build edits |
| W21 | Packaging and CI generalization | Workflows, configure, package scripts | Discovered backend matrix, clean CUDA-free install |
| W22 | Qualification and cutover | Release tooling and support matrix | Performance, soak, canary, rollback, complete claim report |
| W23 | Direct Mesa experiment | Separate version-pinned package | Same-driver comparison and promote/defer decision |

W04's synthetic package exercises discovery only; it does not satisfy W19's real backend gate. W07's evaluator exercises sequential semantics only until concurrency is explicitly modeled. These distinctions must remain in progress reports.

## 2. Decisions made by this roadmap

| Decision | Rationale | Revisit condition |
|---|---|---|
| KIR is the stable boundary; SPIR-T is internal to workers | Avoid language/ABI lock-in and upstream data-layout coupling | A demonstrably stable upstream interchange covers the same semantics/evidence needs |
| F*/Pulse remains the first frontend | Preserve existing kernel/specification work | A separate project requests a frontend rewrite |
| Vulkan is the first new runtime | Matches current shader-oriented SPIR-T scope and provides real compute deployment | P0 exposes a specific blocking semantic/driver requirement |
| Preserve CUDA as an optional migration reference | Enables comparison and staged compatibility | Replacement coverage/evidence justifies retirement |
| Plugin discovery and versioning come before many backends | Prevents permanent centralized dispatch coupling | No exception; implementation details may evolve before freeze |
| Compiler workers own SPIR-T contexts | Avoids unstable ABI and shared-context threading assumptions | Upstream changes are evaluated behind the adapter |
| Metal is the preferred second runtime exercise | Exercises a distinct compiler/runtime boundary with reusable translation tooling | Hardware access or a concrete semantic limitation requires another real API |
| Mesa bypass is a separate experiment | Benefits and maintenance costs require measurement | Evidence establishes a justified production scope |
| Verification status is multi-dimensional | Prevents source proofs from being overstated as compiler/hardware proofs | No exception |

## 3. P0/P1 decisions still requiring evidence

1. Exact Vulkan/SPIR-V environment and mandatory baseline features; minimum device/driver matrix.
2. Exact typed extraction hook and changes needed in the pinned F* fork.
3. KIR initial operation grammar, canonical encoding, and the proof framework/checker for its semantics.
4. Which legacy kernels/features are required for initial production and which need later profiles. The ledger must remain exhaustive even if the first release scope is smaller.
5. Memory model, subgroup guarantees, failure semantics, and numerical policies to freeze.
6. Host ABI/platform coverage, IPC/in-process transports, and benchmark-derived batching policy.
7. Second runtime hardware availability and the smallest workload subset that honestly tests the same portable contract.
8. Ownership of upstream patches, security/bug triage, driver qualification, and long-term supported versions.

These are explicit engineering decisions with default direction supplied by this roadmap. They should be resolved through spikes and evidence, not by silently adding assumptions to the implementation.

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

## 5. Requirements traceability

| User requirement | Implementation mechanism | Decisive gates |
|---|---|---|
| Replace CUDA with SPIR-T | Direct KIR/SPIR-T path; new runtime; optional legacy isolation | G-EXTRACT, G-VERTICAL, G-COVERAGE, G-CUDA-FREE |
| Production-ready | Named support profiles, real workloads/hardware, failures, packaging, rollback | G-CONCURRENCY, G-TRUST, G-PRODUCTION |
| Modular and decoupled | Four adapter axes, contracts, private worker dependencies | G-CONTRACT, G-ADD-COMPILER, G-ADD-RUNTIME |
| Add new GPU backends by addition alone | External manifests/protocols; immutable-core install exercise | G-ADD-COMPILER, G-ADD-RUNTIME, G-ADD-OP |
| Extraction independent of language | Neutral kernel/host package, independent frontend and host bindings | G-EXTRACT, G-ADD-LANGUAGE |
| Preserve verification and semantics | Explicit obligations, evidence levels, checked transformations, trust inventory | G-TRUST and per-profile O1–O10 |
| Retain performance/control | Typed metadata, explicit pipelines, qualified specialization and tuning | G-COVERAGE, G-PRODUCTION, G-MESA-DECISION where applicable |

## 6. Completion reporting

Report progress by completed work packages, passed gates, corpus coverage, and unresolved obligations. Avoid reporting “80% complete” from line counts or the number of backends that compile a minimal example.

A useful phase report states: what works; exact profile/device/revision; what was checked; what remains assumed; unsupported cases; measured regressions; and the next gate. Attach reproducible artifacts and hashes. Never use the existence of this roadmap as evidence that the new implementation exists.
