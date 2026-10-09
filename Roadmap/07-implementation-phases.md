# 07. Implementation phases

Each phase produces reviewable artifacts and an objective exit gate. Work can overlap, but a later release claim cannot bypass an earlier unmet gate. The proof work starts with the contract; it is not deferred until after an unverified compiler is complete.

All phase labels and commands in this roadmap are planned unless explicitly shown as existing repository commands. `milestones.json` records dependencies and gate IDs.

## P0. Inventory the current semantics and establish the baseline

**Deliverables**

1. Record exact repository/submodule/tool/driver revisions and build configurations. Reproduce verification, extraction, GPU tests, and benchmarks with their actual prerequisites.
2. Expand the extractor dispatch into a machine-readable primitive ledger: source symbol, signature, effect, current emitted behavior, proof dependency, numeric policy, target requirements, proposed KIR operation, and migration owner.
3. Inventory default F* extraction, all reachable foreign operations, generated instantiation scripts, Klas entrypoints, host plans, and `dist/` provenance. Include features implemented outside the central extractor.
4. Classify proof holes and deliberate axioms by transitive entrypoint closure. Isolate the warp barrier and integer-width assumptions before designing a portable contract.
5. Capture representative correctness and performance baselines: scalar/vector transforms, reductions, softmax/log-softmax, dense tiled GEMM, sparse matrix multiplication, and asynchronous chains.
6. Secure hardware/CI access: at least two Vulkan vendors, a distinct runtime/API for P7, and suitable hardware for any advanced feature to be claimed. Choose exact initial target environments.
7. Investigate the F* typed/pre-erasure hook and direct SPIR-T construction with one real extracted entrypoint.

**G-BASELINE:** complete inventory, reproducible baseline, scoped proof assumptions, and an agreed initial release corpus. Missing hardware or feature evidence is recorded as a blocker to that claim, not converted into a pass.

**Existing build entrypoints:** `make -j$(nproc) prepare`, `make -j$(nproc) verify`, `make -j$(nproc) extract-all`, `make -j$(nproc) test`, and `make -j$(nproc) list-admits`. Initialize appropriate submodules first. The GPU targets require their documented hardware/toolchain. Run strict verification with development bypass variables/options absent and validate cache provenance.

## P1. Define contracts and the semantic core

**Depends on:** P0.

1. Specify KIR, host plans, ABI layouts, operation/effect catalog, target profiles, and numerical policies.
2. Specify package discovery, compiler/runtime protocols, extension semantics, version negotiation, and evidence handling.
3. Build canonical encoders/decoders, validators, reference semantics, and error catalogs. Start the corresponding proof/checker soundness work.
4. Split dependencies so no frontend/runtime vendor SDK appears in the core contract library.
5. Implement plugin discovery and generic build/test discovery immediately. Do not postpone them until several backends already depend on hardcoded registration.

**G-CONTRACT:** reviewed specifications and positive/negative conformance fixtures; two independent readers agree on canonical test vectors. Interfaces remain provisional until the P7 exercise succeeds.

## P2. Extract F*/Pulse into KIR

**Depends on:** P1.

1. Implement typed manifest capture and executable extraction, with deterministic links between them.
2. Port scalar/control/resource/host operations from the inventory without CUDA names in public output.
3. Keep existing CUDA extraction available for comparison. A temporary importer is allowed only as scaffolding with a removal condition.
4. Run the neutral host-plan evaluator for the supported subset.
5. Add the restricted C frontend and C/Rust bindings early enough to expose language-specific assumptions.

**G-EXTRACT:** declared baseline programs extract without Karamel/CUDA as mandatory stages; all runtime/static/proof-only data are classified; unknown primitives fail with source locations. Evidence identifies exactly which extraction relations are discharged.

## P3. Build the first complete SPIR-T/Vulkan path

**Depends on:** P2.

1. Implement the direct KIR → SPIR-T adapter and environment-specific SPIR-V emission.
2. Implement a minimal real Vulkan runtime: discovery, buffers, transfer, module/pipeline creation, dispatch, completion, and cleanup.
3. Execute scalar transforms, array views, a simple reduction, and a baseline matrix kernel end to end. Minimal probes may localize bugs; real corpus cases determine progress.
4. Add reflection/ABI cross-checking, target validation, failure reporting, and cache provenance.
5. Build/install/run the path in an environment without CUDA toolkit/runtime, NVCC, or Karamel dependencies in the new path.

**G-VERTICAL:** repeatable correct end-to-end runs on one real GPU, correct errors on unavailable capabilities, and a CUDA-free dependency/install report. This is an engineering milestone, not production qualification.

## P4. Make memory, synchronization, and asynchronous execution correct

**Depends on:** P3; concurrency/semantics work begins during P1.

1. Complete buffers/views, shared-memory layout, vector access, bounds and overflow handling.
2. Resolve subgroup participation and width assumptions. Replace unsafe contracts rather than merely changing constants.
3. Implement memory scopes/orders, supported atomics, host visibility, and cross-queue dependencies.
4. Preserve stream/epoch/pledge semantics, chained dispatch, failure outcomes, cancellation, and resource lifetimes.
5. Execute portable corpus tests on a second Vulkan vendor and exercise independent contexts/workers.

**G-CONCURRENCY:** required ownership/memory relations are documented and checked; litmus, fault, layout, and async-chain tests pass on the qualified devices. Freeze a **candidate** v1 boundary for the independent plugin exercise; any required core fix restarts that freeze/test.

## P5. Cover numerical and advanced kernel requirements

**Depends on:** P4.

1. Implement and qualify scalar float formats, casts, exceptional-value behavior, and approved math operations.
2. Migrate dense/sparse GEMM, reductions, softmax, fused epilogues, vectorized paths, and relevant layout instantiations.
3. Add cooperative matrix support only for queried and qualified combinations. Record unsupported variants.
4. Isolate WMMA/WGMMA and other vendor-specific semantics in extensions. Port, provide a valid alternative, or explicitly defer each legacy feature.
5. Tune schedules and pass choices against measured workloads without weakening numerical contracts.

**G-COVERAGE:** every inventory row has a disposition; every feature claimed for the release has implementation and numerical/performance evidence. A portable-core release cannot be labeled full CUDA feature parity while mandatory legacy rows remain unimplemented.

## P6. Close the verification argument for release profiles

**Depends for exit on:** P5; starts at P1 and runs throughout P2–P5.

1. Discharge O1–O10 for the chosen assurance level, recording legitimate trusted boundaries.
2. Complete extraction/representation relations, pass validation, layout/ABI checks, and runtime state-machine correspondence.
3. Remove unresolved development admits from released source-proof closures. Separate semantic axioms from missing proofs.
4. Replay strict checks under pinned toolchains and validate evidence/artifact binding.
5. Ensure unsupported or unverified extension behavior cannot receive a stronger assurance label through fallback or cache reuse.

**G-TRUST:** the verification report is complete and reproducible; no unresolved obligation is silently counted as proved. Production and verified-profile labels are bounded by the actual report.

## P7. Prove the architecture is additive and language-independent

**Depends on:** P4 and the P2 language/host boundary; can overlap P5/P6.

1. Freeze/hash a release-candidate core binary and read-only source checkout.
2. Build an out-of-tree second compiler/runtime package. Prefer a narrowly scoped Metal backend using qualified SPIR-V → MSL translation and a native runtime adapter; select it during P0 feasibility work.
3. Run the same portable corpus and host bindings through that package. Add a separate frontend/host adapter and a checked semantic extension.
4. Verify no core/frontends/existing kernels/registries/root build manifests changed. Verify generic build/test discovery works.
5. If the exercise requires an interface change, update the provisional contract, freeze again, and repeat with an independent package. Do not waive the requirement because the change is small.

**G-ADD-COMPILER, G-ADD-RUNTIME, G-ADD-LANGUAGE, G-ADD-OP:** all pass with recorded hashes and diffs. A simulator or another Vulkan device is useful evidence but does not replace the distinct runtime/API gate.

## P8. Qualify, package, cut over, and support

**Depends on:** P5, P6, P7.

1. Complete the production matrix, soak tests, performance budgets, error-path exercises, clean installation, and compatibility corpus.
2. Ship separate portable/compiler/runtime/binding packages with reproducible provenance and dependency/license inventories.
3. Publish exact supported devices/drivers/profiles and assurance levels. Freeze v1 and document upgrade/rollback policy.
4. Run canaries with real workloads. Make the new path the default only after the gates pass.
5. Keep CUDA as an optional legacy/reference package; remove it from default discovery/build/install requirements. Retire compatibility code only under a separate, evidence-backed deprecation decision.

**G-PRODUCTION, G-CUDA-FREE:** release matrix and rollback rehearsal pass; unsupported scope is explicit; no default path accidentally resolves CUDA components.

## PX. Direct Mesa research

**Depends on:** the stable P4 baseline; never blocks P8.

Deliver one-driver prototype, semantics comparison, generated-code/performance evidence, maintenance estimate, and a promote/defer decision. Native ISA generation and direct backend-IR routes receive separate work packages rather than being hidden inside the portable migration.

**G-MESA-DECISION:** measured evidence supports the decision. A negative result is a valid completed experiment.

## Effort and sequencing assumptions

These are planning ranges, not measured implementation estimates or delivery promises. They assume engineers already familiar with compiler construction, GPU memory models, F*/Pulse, and driver APIs, plus access to required hardware.

| Phase | Initial engineering effort range |
|---|---:|
| P0 | 2–4 person-weeks |
| P1 | 4–8 person-weeks |
| P2 | 4–8 person-weeks |
| P3 | 6–12 person-weeks |
| P4 | 8–16 person-weeks |
| P5 | 8–20 person-weeks |
| P6 | 10–24 person-weeks |
| P7 | 6–14 person-weeks |
| P8 | 8–16 person-weeks |

The sum is 56–122 person-weeks before the optional Mesa track. Concurrent work can shorten calendar time but not remove proof and hardware dependencies. Full legacy parity, new proof foundations, upstream compiler changes, or unavailable driver features can exceed this range substantially. Re-estimate after P0 and after the P3 end-to-end path; proof research should not be treated as fixed-duration routine implementation.

Keep review units small and evidence-complete. Parallel teams can own semantics, extraction/compiler, runtime/bindings, and hardware qualification, with one owner for each interface. Do not merge cross-cutting implementation before the owning contracts are reviewable.
