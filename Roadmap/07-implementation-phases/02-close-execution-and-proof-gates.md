# Close execution, coverage and proof gates

**Milestone M23.** A real GPU vertical path, concurrency/numerical qualification and complete evidence for the requested assurance policy.

## Required inputs and specification

Start from [M14](../04-spirt-and-gpu-lowering/02-run-the-integer-vertical-slice.md), [M15](../04-spirt-and-gpu-lowering/03-preserve-control-memory-and-participation.md), [M16](../04-spirt-and-gpu-lowering/04-complete-numerics-and-target-emission.md), [M19](../05-runtime-and-interop/03-qualify-the-vulkan-service.md), [M20](../06-verification-and-trust/01-bind-source-and-transformation-evidence.md), [M21](../06-verification-and-trust/02-admit-immutable-evidence-policies.md). Use the exit-gate dependencies in [the implementation plan](../milestones.json); proof and investigation work may begin earlier. Read [the declarative specification](../Specification/README.md) before choosing representation details. This milestone implements `Memory.admissible`, `Runtime.safe_step`, `Refinement.complete`.

## P3. Build the first complete SPIR-T/Vulkan path

**Depends on:** P2.

1. Implement the direct KIR → SPIR-T adapter and environment-specific SPIR-V emission.
2. Implement a minimal real Vulkan runtime: discovery, buffers, transfer, module/pipeline creation, dispatch, completion, and cleanup.
3. Execute integer transforms and array views first, followed by bounded reductions and matrix kernels whose needed control, memory, and numerical semantics are already implemented. Implement each synchronization/numeric obligation before admitting a workload that needs it; P4/P5 complete coverage rather than retroactively making P3 safe.
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

1. Discharge O1–O10 as required by the fixed evidence policy; record only the assumptions that policy permits. A producer-chosen trusted boundary cannot waive a required relation.
2. Complete extraction/representation relations, pass validation, layout/ABI checks, and runtime state-machine correspondence.
3. Remove unresolved development admits from released source-proof closures. Separate semantic axioms from missing proofs.
4. Replay strict checks under pinned toolchains and validate evidence/artifact binding.
5. Ensure unsupported or unverified extension behavior cannot receive a stronger assurance label through fallback or cache reuse.

**G-TRUST:** the verification report is complete and reproducible; no unresolved obligation is silently counted as proved. Production and verified-profile labels are bounded by the actual report.

## Evidence required to close this milestone

Close **P3–P6** only with the implementation artifacts, positive and negative cases, and source-to-result identities required above. Link the implementation relation to the named F* symbols and the policy's applicable O1–O10 obligations. The specification's proved lemmas are reusable model facts; they do not discharge this implementation correspondence. Record unresolved cases as blockers or explicitly outside the claim. No backend gate is marked passed by this roadmap revision.
