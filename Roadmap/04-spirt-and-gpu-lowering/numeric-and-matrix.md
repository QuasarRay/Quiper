# Numeric and matrix

## 5. Floating point is part of the specification

Define at least two profile families:

- A strict profile whose exact required operations and exceptional behavior are spelled out; reject targets that cannot satisfy it.
- Approximate profiles that name each allowed approximation and its domain/error relation. Different error relations are different capabilities.

Specify FMA contraction, reassociation, signed zero, infinities, NaNs, subnormals/FTZ, conversions, and transcendental functions. Storage support for FP16/BF16 does not imply arithmetic or matrix support. Do not assume a GLSL extended instruction has the same error bounds as a CUDA intrinsic.

For tests, use exact bit comparisons where the contract requires them, ULP/absolute/relative bounds only where permitted, and classification-sensitive cases for exceptional values. For nondeterministic reductions, compare the allowed result relation rather than requiring one accidental execution order.

Existing WGMMA specifications use an opaque hardware result relation. A scalar implementation can replace it only if it satisfies the selected contract; similarity to real matrix multiplication is insufficient. [Q9](../10-sources/README.md)
## 6. Matrix acceleration and vendor features

Keep matrix multiply/load/store/fill semantic operations independent from concrete lane-register layouts. Query cooperative matrix shape/type/scope combinations and specialize accordingly. A device advertising the extension does not support every shape or input/accumulator combination. [E4](../10-sources/README.md)

Handle existing mapped fragment loads, fused epilogues, packed layouts, and fragment access explicitly. Not every such operation maps directly to an opaque cooperative matrix API. Lower through explicit storage or a vendor extension only when its relation and cost are acceptable; otherwise report the missing feature.

WGMMA, asynchronous shared-memory pipelines, and instruction-specific data layouts are extension work. Preserve their expressiveness in the contract without presenting them as universally portable. NVIDIA-only features cannot be a prerequisite for the baseline Vulkan backend.
