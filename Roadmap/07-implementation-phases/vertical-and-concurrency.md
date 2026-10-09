# Vertical and concurrency

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
