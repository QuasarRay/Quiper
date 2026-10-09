# Other backends

## 7. Backend sequence

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
## 8. Direct SPIR-T → Mesa NIR track

Mesa NIR is a common optimizing compiler representation inside many Mesa driver compilers. It is not a universal GPU ISA or a public replacement for device submission APIs. [E6](../10-sources/README.md)

Run a bounded experiment after the Vulkan path supplies a correctness and performance baseline:

1. Choose one Mesa revision, one driver, one device, and the baseline compute subset.
2. Implement direct conversion of SPIR-T control flow, types, memory operations, built-ins, and required metadata into NIR through a version-pinned adapter.
3. Integrate at a documented private driver/compiler boundary that can actually produce a loadable shader. Identify who performs layout, relocation, pipeline setup, and command submission.
4. Compare final machine code, runtime, compile latency, memory use, and optimization opportunities against the same driver reached through SPIR-V. Keep workload, numerical mode, and driver options equal.
5. Record information lost by the SPIR-V route. First test whether existing decorations, specialization, or a targeted importer improvement preserves it.
6. Promote the direct path only for measured benefits that justify its additional maintenance and validation cost.

Bypassing SPIR-V may improve compile latency or preserve useful information; it does not automatically improve GPU execution. Direct conversion to each hardware backend IR or ISA multiplies target-specific work and must be isolated in separate packages. Do not block the first production release on a full Mesa/compiler-driver rewrite.
