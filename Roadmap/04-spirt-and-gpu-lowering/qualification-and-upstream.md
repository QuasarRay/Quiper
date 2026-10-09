# Qualify the worker and manage upstream changes

## 1. Maintain a version decision record

Record main plus each selected local/upstream patch, its prerequisite commits, rationale, owner, tests and removal/update condition. Investigate the open memory/control-flow/scalar/vector/interpreter work before duplicating infrastructure. The [SPIR-T repository documentation](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/README.md) identifies the project as evolving and its textual display as non-interchange output.

The prototype interpreter and Vulkan round-trip layer may help testing. Their existence does not qualify them as a host executor or independent semantic oracle. Keep the small KIR reference semantics separate from compiler transformations whose bugs the tests are meant to detect. Do not adopt a whole unmerged stack without checking each dependency and regression surface.

## 2. Run a layered corpus

1. **Representation:** types/constants, aggregates, multi-entrypoint modules, symbol imports, canonical identity and annotations.
2. **Scalar/control:** exact integer edges, casts, conditionals, loops, exits, nested calls and invalid/uninitialized inputs.
3. **Memory:** views, strides, offsets, aliasing, initialization, shared storage, tails, zero/maximum sizes and overflow.
4. **Concurrency:** barrier participation, subgroup constraints, atomics, visibility, dependent queues and failure paths.
5. **Numerical:** operation-specific exceptional values, reductions, softmax/log-softmax, dense/sparse GEMM, qualified matrix variants.
6. **Integration:** C/Rust bindings, identical artifacts, clean CUDA-free installation, cache integrity, cancellation, device loss and package rollback.

Use an independent source/KIR oracle, metamorphic relations with justified preconditions, legacy CUDA comparisons, and real-device results as complementary evidence. Minimize failures into pinned regressions. Ordinary agreement on sampled inputs does not discharge universal preservation obligations.

## 3. Upgrade without changing the core

Build the updated private worker as a new package. Keep old core/contracts and existing runtime packages installed. Replay annotation, loop, QPtr, layout, numeric, evidence and full corpus tests. Re-run affected extension gates for the candidate identity. Keep old worker versions loadable for supported old packages until their declared support window ends.

## 4. Treat future targets as independent emitters

For Metal, keep SPIR-V-to-MSL translation and native runtime work inside the new package and qualify binding/resource semantics separately. Select the candidate and secure hardware in P0; P7 performs the independent implementation exercise. An unavailable second API blocks G-ADD-RUNTIME rather than being replaced by a second Vulkan vendor.

For direct Mesa NIR, implement a separate version-pinned emitter/runtime integration experiment. Compare against the same driver's SPIR-V route with identical semantic modes and workloads. Measure compile cost, generated code, execution, maintenance surface and correctness. Keep it optional until evidence justifies promotion. Other native ISA backends also need runtime/loader/ABI work; ISA bytes alone are not a backend.
