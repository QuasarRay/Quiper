# 10. Sources and audit provenance

The roadmap distinguishes **observed implementation**, **primary specification/documentation**, and **proposed design**. Proposed architecture, timelines, performance budgets, profiles, protocols, and proof obligations are engineering decisions, not claims that upstream already implements them.

Repository evidence was inspected at the full revisions below on 2026-10-09. External `latest` documentation is mutable; P0 must pin the exact API/specification/tool revisions used for implementation and qualification.

## Quiper / Kuiper evidence

All Q-links refer to Quiper commit `413219948f91911ffaf0ac37a5ff941c5d1e55c7`.

| ID | Source | What was used |
|---|---|---|
| Q1 | [README](https://github.com/QuasarRay/Quiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/README.md), [AGENTS](https://github.com/QuasarRay/Quiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/AGENTS.md) | Current purpose, CUDA pipeline, admitted-proof notice, build conventions |
| Q2 | [ExtractKuiper.fst](https://github.com/QuasarRay/Quiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/extraction/ExtractKuiper.fst), [ExtractionUtils.fst](https://github.com/QuasarRay/Quiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/extraction/ExtractionUtils.fst) | Actual compiler hooks, primitive mappings, type erasure limitations, launch and math lowering |
| Q3 | [verify.mk](https://github.com/QuasarRay/Quiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/verify.mk), [extraction/Makefile](https://github.com/QuasarRay/Quiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/extraction/Makefile), [fixup.sed](https://github.com/QuasarRay/Quiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/scripts/fixup.sed) | Toolchain/plugin build modes, CUDA extraction rules, textual postprocessing |
| Q4 | [kuiper.h](https://github.com/QuasarRay/Quiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/include/kuiper.h), [atomics.h](https://github.com/QuasarRay/Quiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/include/kuiper/atomics.h), [nvcc.mk](https://github.com/QuasarRay/Quiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/nvcc.mk) | Native CUDA runtime dependence, helpers, target selection, test/extraction roots |
| Q5 | [Barrier.Warp](https://github.com/QuasarRay/Quiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/src/lib/kuiper/Kuiper.Barrier.Warp.fsti), [Barrier](https://github.com/QuasarRay/Quiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/src/lib/kuiper/Kuiper.Barrier.fsti) | Fixed warp width, existing contract warning, block barrier resources |
| Q6 | [SizeT](https://github.com/QuasarRay/Quiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/src/lib/kuiper/Kuiper.SizeT.fst) | Admitted width relation and explicit 32-bit operations |
| Q7 | [SHMem](https://github.com/QuasarRay/Quiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/src/lib/kuiper/Kuiper.SHMem.fsti) | Consecutive shared slices and proof-visible base/alignment relations |
| Q8 | [Kernel.Base](https://github.com/QuasarRay/Quiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/src/lib/kuiper/Kuiper.Kernel.Base.fsti), [Kernel.Stream](https://github.com/QuasarRay/Quiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/src/lib/kuiper/Kuiper.Kernel.Stream.fsti), [Async.Chain example](https://github.com/QuasarRay/Quiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/src/examples/Kuiper.Example.Async.Chain.fst) | Queue/epoch/pledge semantics and an existing dependent-launch workload |
| Q9 | [TensorCore.WGMMA](https://github.com/QuasarRay/Quiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/src/lib/kuiper/Kuiper.TensorCore.WGMMA.fsti), [WGMMA.Layout](https://github.com/QuasarRay/Quiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/src/lib/kuiper/Kuiper.TensorCore.WGMMA.Layout.fsti) | Distinct numerical relation and packed instruction-specific layouts |
| Q10 | [AtomicOps](https://github.com/QuasarRay/Quiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/src/lib/kuiper/Kuiper.AtomicOps.fsti) | Current atomic operation interfaces and value contracts |
| Q11 | [CI workflow](https://github.com/QuasarRay/Quiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/.github/workflows/ci.yml), [list-admits.py](https://github.com/QuasarRay/Quiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/scripts/list-admits.py) | Existing build/verification checks and lexical trust inventory support |
| Q12 | [.gitmodules](https://github.com/QuasarRay/Quiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/.gitmodules), [repository tree](https://github.com/QuasarRay/Quiper/tree/413219948f91911ffaf0ac37a5ff941c5d1e55c7) | Repository paths, submodule topology, source/generated surface |

Observed submodule commits: F* `0eef57bef411aac090354a75c21e00b674bd420c`; Karamel `75bc9443b430f5161d85ff02eedb385e9a6db607`. Their full implementation was not audited in this task. The roadmap requires that investigation before selecting extraction hooks or asserting preservation theorems.

The raw file/line counts in document 00 count regular files under the named directories in the clean pinned checkout, using newline-separated bytes. They include comments, interfaces, scripts, and generated/distribution material. The 240-name count is the set of quoted `Kuiper.*` identifiers found lexically in `ExtractKuiper.fst`, not a completeness proof for supported features.

## SPIR-T evidence

All S-links refer to SPIR-T commit `e8757adba8d14068a7bf1b3bc9f24cac982f4bd3`. Its package manifest reports version `0.4.0`; the commit identity is the audit anchor.

| ID | Source | What was used |
|---|---|---|
| S1 | [README](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/README.md) | Shader orientation, evolving scope, OpenCL/text-parser exclusions, available facilities |
| S2 | [IR definitions](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/lib.rs), [Cargo.toml](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/Cargo.toml) | Public construction, SPIR-V-oriented variants, context ownership, safe-Rust policy, dependency/version boundary |
| S3 | [SPIR-V module](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/spv/mod.rs), [legalization](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/passes/legalize.rs) | Conversion terminology, representation, structurization entrypoint |
| S4 | [QPtr model](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/qptr/mod.rs), [layout configuration](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/qptr/layout.rs), [QPtr example](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/examples/spv-lower-link-qptr-lift.rs) | Pointer/provenance assumptions, extent widths, configuration, illustrative pass sequence |
| S5 | [Repository tree](https://github.com/Rust-GPU/spirt/tree/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3), [.gitmodules](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/.gitmodules) | Inspected implementation scope and SPIRV-Headers dependency |

“No direct Mesa emitter found” describes this inspected tree. It is not a claim that no prototype exists anywhere in the ecosystem or in uninspected branches.

## Primary external references

| ID | Source | Relevance |
|---|---|---|
| E1 | [Vulkan memory model](https://docs.vulkan.org/spec/latest/appendices/memorymodel.html) | Memory scopes, ordering, availability, and visibility |
| E2 | [Vulkan synchronization and cache control](https://docs.vulkan.org/spec/latest/chapters/synchronization.html) | Runtime dependencies and memory-domain operations |
| E3 | [Vulkan environment for SPIR-V](https://docs.vulkan.org/spec/latest/appendices/spirvenv.html), [shader execution](https://docs.vulkan.org/spec/latest/chapters/shaders.html) | Target-environment requirements and subgroup/execution constraints |
| E4 | [Cooperative matrix properties](https://docs.vulkan.org/refpages/latest/refpages/source/VkCooperativeMatrixPropertiesKHR.html), [property enumeration](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR.html) | Device-specific supported matrix type/shape/scope combinations |
| E5 | [SPIRV-Tools](https://github.com/KhronosGroup/SPIRV-Tools) | Target validation tools and the documented limits of validator completeness |
| E6 | [Mesa NIR documentation](https://docs.mesa3d.org/nir/index.html), [Mesa source tree](https://docs.mesa3d.org/sourcetree.html) | NIR's role and target/compiler/driver separation |
| E7 | [SPIRV-Cross](https://github.com/KhronosGroup/SPIRV-Cross) | Candidate reflection/MSL translation reuse for a separate Metal backend |

These references motivate the engineering plan. They do not establish that the proposed Kuiper integration has passed conformance, preserved proofs, or achieved its performance targets.

## Documentation review performed

This change adds a roadmap only. Its review checks relative document links, pinned repository source paths, milestone/gate consistency, Markdown structure, machine-readable JSON validity, and absence of implementation changes. It does not run the future proof/compiler/runtime release gates. Completion of those gates belongs to the implementation phases above.
