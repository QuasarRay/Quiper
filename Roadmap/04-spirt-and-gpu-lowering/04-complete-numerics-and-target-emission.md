# Complete numerical coverage and additional target emission

**Milestone M16.** Per-family numerical relations and workload coverage, with every optional target route independently qualified.

## Required inputs and specification

Start from [M15](03-preserve-control-memory-and-participation.md). Use the exit-gate dependencies in [the implementation plan](../milestones.json); proof and investigation work may begin earlier. Read [the declarative specification](../Specification/README.md) before choosing representation details. This milestone implements `Operations.matrix_policy`, `Memory.float_policy`, `Operations.catalog_covers`.

## Floating point is part of the specification

Define at least two profile families:

- A strict profile whose exact required operations and exceptional behavior are spelled out; reject targets that cannot satisfy it.
- Approximate profiles that name each allowed approximation and its domain/error relation. Different error relations are different capabilities.

Specify FMA contraction, reassociation, signed zero, infinities, NaNs, subnormals/FTZ, conversions, and transcendental functions. Storage support for FP16/BF16 does not imply arithmetic or matrix support. Do not assume a GLSL extended instruction has the same error bounds as a CUDA intrinsic.

For tests, use exact bit comparisons where the contract requires them, ULP/absolute/relative bounds only where permitted, and classification-sensitive cases for exceptional values. For nondeterministic reductions, compare the allowed result relation rather than requiring one accidental execution order.

Existing WGMMA specifications use an opaque hardware result relation. A scalar implementation can replace it only if it satisfies the selected contract; similarity to real matrix multiplication is insufficient. [Q9](../10-sources/README.md)
## Matrix acceleration and vendor features

Keep matrix multiply/load/store/fill semantic operations independent from concrete lane-register layouts. Query cooperative matrix shape/type/scope combinations and specialize accordingly. A device advertising the extension does not support every shape or input/accumulator combination. [E4](../10-sources/README.md)

Handle existing mapped fragment loads, fused epilogues, packed layouts, and fragment access explicitly. Not every such operation maps directly to an opaque cooperative matrix API. Lower through explicit storage or a vendor extension only when its relation and cost are acceptable; otherwise report the missing feature.

WGMMA, asynchronous shared-memory pipelines, and instruction-specific data layouts are extension work. Preserve their expressiveness in the contract without presenting them as universally portable. NVIDIA-only features cannot be a prerequisite for the baseline Vulkan backend.

## Backend sequence

| Backend | Purpose | Planned status |
|---|---|---|
| Existing CUDA/Karamel | Migration reference and legacy compatibility | Optional after cutover; still CUDA-dependent |
| Vulkan/SPIR-V | First production portable compute profile | Required |
| Metal via qualified SPIR-V → MSL translation | Distinct runtime/compiler package for additive-backend validation | Preferred second backend; requires Apple hardware qualification |
| Direct Mesa NIR | Evaluate benefits of bypassing SPIR-V serialization and import | Experimental research track |
| OpenCL `Kernel` | Possible future compute route | Requires explicit dialect/lowering work beyond current upstream SPIR-T scope |
| LLVM/native ISA/PTX/other APIs | Specialized future routes | Separate emitter, runtime, ABI, and evidence qualification |

SPIRV-Cross supplies SPIR-V reflection and MSL translation that can be evaluated for the Metal worker; it does not supply this project's runtime or correctness proof. Pin it and qualify the supported subset. [E7](../10-sources/README.md)

Source-language independence does not prohibit one backend from privately generating MSL or another target language. It prohibits making that language a universal prerequisite or public extraction contract.
## Direct SPIR-T → Mesa NIR track

Mesa NIR is a common optimizing compiler representation inside many Mesa driver compilers. It is not a universal GPU ISA or a public replacement for device submission APIs. [E6](../10-sources/README.md)

Run a bounded experiment after the Vulkan path supplies a correctness and performance baseline:

1. Choose one Mesa revision, one driver, one device, and the baseline compute subset.
2. Implement direct conversion of SPIR-T control flow, types, memory operations, built-ins, and required metadata into NIR through a version-pinned adapter.
3. Integrate at a documented private driver/compiler boundary that can actually produce a loadable shader. Identify who performs layout, relocation, pipeline setup, and command submission.
4. Compare final machine code, runtime, compile latency, memory use, and optimization opportunities against the same driver reached through SPIR-V. Keep workload, numerical mode, and driver options equal.
5. Record information lost by the SPIR-V route. First test whether existing decorations, specialization, or a targeted importer improvement preserves it.
6. Promote the direct path only for measured benefits that justify its additional maintenance and validation cost.

Bypassing SPIR-V may improve compile latency or preserve useful information; it does not automatically improve GPU execution. Direct conversion to each hardware backend IR or ISA multiplies target-specific work and must be isolated in separate packages. Do not block the first production release on a full Mesa/compiler-driver rewrite.

## Evidence required to close this milestone

Close **G-COVERAGE, G-REPLACEMENT, optional G-MESA-DECISION** only with the implementation artifacts, positive and negative cases, and source-to-result identities required above. Link the implementation relation to the named F* symbols and the policy's applicable O1–O10 obligations. The specification's proved lemmas are reusable model facts; they do not discharge this implementation correspondence. Record unresolved cases as blockers or explicitly outside the claim. No backend gate is marked passed by this roadmap revision.
