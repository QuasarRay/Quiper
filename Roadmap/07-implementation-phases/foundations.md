# Foundations

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

**G-SPIRT-FEASIBILITY (P0):** record the annotation subset, loop-report triage/exclusion, QPtr eligibility and dependency decision before enabling the affected path. Select the second API, hardware and common workload subset in P0; P7 performs the independent implementation. See [the detailed phase procedure](implementation.md).
