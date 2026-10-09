# 04. SPIR-T and GPU lowering

## 1. First production path

Implement a Vulkan compute backend first. Choose a precise Vulkan/SPIR-V target environment during P0, record it in the backend manifest, and validate against that environment. Do not target an unspecified “latest SPIR-V.” Optional features and extensions must be individually queried and enabled. [E3](10-sources.md)

The backend worker reads KIR, constructs SPIR-T directly through a pinned adapter, applies a reviewed pipeline, emits SPIR-V, validates it, prepares reflection/layout metadata, and packages it for the Vulkan runtime. The current upstream `spv::lower` name means SPIR-V → SPIR-T; `spv::lift` is the reverse. Keep that terminology unambiguous in implementation. [S2–S3](10-sources.md)

A serialized SPIR-V detour on input is useful for differential/round-trip tests but is not required for direct construction. A particular backend may emit SPIR-V on output. Neither statement implies SPIR-T is itself a hardware execution API.

## 2. Required lowering sequence

1. Validate KIR and accepted evidence; resolve imports and required semantic extensions.
2. Fix the program's semantic profile and target constraints before optimizations.
3. Specialize kernel parameters, layouts, workgroup shapes, and supported numeric operations. Retain a checked relation to the unspecialized contract.
4. Construct SPIR-T values, regions, globals, types, and exports using the adapter's explicit mapping. Preserve operation IDs and source/evidence correspondence.
5. Normalize control flow and calls. Structurize only when the transformation preserves exits, side effects, convergence, and barriers; reject unsupported control flow.
6. Legalize memory and pointers. Use `QPtr` where its analyzed subset fits; do not make successful legalization a substitute for provenance/bounds validation.
7. Apply approved optimizations with preconditions and evidence bookkeeping. Keep FP-sensitive rewrites behind the requested numeric mode.
8. Lower target operations and remove all unresolved custom operations. Finalize entrypoints, interfaces, bindings, memory model, capabilities, extensions, and execution modes.
9. Emit and validate SPIR-V; cross-check reflection against KIR layouts and runtime dispatch metadata.
10. Record all tool/pass versions, flags, requirements, specialization choices, and output hashes. Publish the cache entry atomically only after all mandatory checks succeed.

Pass ordering is a contract. Do not blindly copy an example's `QPtr` sequence or layout constants: the inspected example is exploratory and even leaves its final write disabled. [S4](10-sources.md)

## 3. Primitive migration matrix

| Current family | Neutral meaning | Vulkan/SPIR-T work | Required check |
|---|---|---|---|
| Grid/block/thread IDs | Workgroup and local invocation coordinates | Built-ins plus explicit dimensional/index conversion | Same indexing and overflow bounds |
| Scalar integer operations | Width-specific arithmetic | Typed SPIR-V operations | Wrapping/checked/division semantics |
| Scalar floats | Explicit format and numeric policy | Operations, conversions, controls, libraries | Rounding/denormal/exceptional-value contract |
| Global arrays and slices | Resource + element layout + offset/extent | Storage buffers and access chains | Bounds, aliasing, byte-size overflow |
| Private references | Per-invocation mutable storage/value | Function/private storage or scalar replacement | Initialization and non-escape |
| Shared-memory slices | Per-workgroup storage with proven layout | Statically allocated or specialization-sized workgroup storage | Alignment, limits, slice disjointness, preserved address relations |
| Block barriers | Uniform control barrier plus memory effects | Correct execution scope, memory scope, storage semantics | Participation and visibility relation |
| Warp barriers | Subgroup participation and memory contract | Explicit subgroup profile and operations | No implicit width 32 or full-mask assumption |
| Integer atomics | Atomic RMW at declared scope/order | Supported integer atomics | Linearizability and allowed memory outcomes |
| Floating atomics | RMW with numeric contract | Queried extension or qualified emulation | Type/scope support, NaNs, progress, ordering |
| Vectorized loads/copies | Typed access preserving layout/effects | Vector/scalar legalizations | Alignment, tail handling, no overread |
| WMMA-like operations | Cooperative matrix operation with shape/scope/relation | Queried cooperative matrix support or valid fallback | Formats, layouts, participation, numerical contract |
| WGMMA | Explicit vendor instruction-family contract | Vendor package or separately justified decomposition | Packed layout, async protocol, numeric relation |
| Fast math | Named approximation with preconditions/error relation | Target implementation or math library | Domain and error bounds, no silent policy weakening |
| Host allocate/copy/free | Resource lifecycle and visibility | Runtime commands | Correct ownership on success and error |
| Streams and epochs | Ordered submission and completion tokens | Queue/event/semaphore mapping | No early ownership redemption |
| Launch bounds | Semantic constraints or optimization hints, explicitly separated | Limits/specialization plus optional hints | Required constraint enforced; hint never assumed as proof |
| Device assertions | Contract checking and failure semantics | Diagnostic buffer or supported mechanism | No success reported after an assertion failure |

P0 must expand this family table to every actual extraction case and emitted primitive. Operations absent from v1 receive an explicit unsupported classification, owner, and later milestone.

## 4. Shared memory and pointer legalization

CUDA dynamic shared-memory byte allocation and Vulkan workgroup storage are not interchangeable API calls. Determine which sizes can be specialized before pipeline creation. Cache specialized variants under the exact size/layout tuple. If a runtime size cannot be legally represented, reject it or use a separately proved transformation; do not silently move shared storage to global memory.

Use KIR resource layouts as the authority. Check that SPIR-T `QPtr` configuration, SPIR-V layout decorations, runtime descriptor ranges, and binding code agree. Internal abstract pointer sizes are not host/device pointer sizes. The current pointer implementation contains 32-bit offset/extent fields; reject out-of-range cases until a wider qualified path exists. [S4](10-sources.md)

Test mixed-type shared storage, unaligned slices, aggregate padding, zero-sized/empty cases, vector tails, and maximum offsets. Any layout optimization that changes a proof-visible address relation needs a corresponding refinement argument.

## 5. Floating point is part of the specification

Define at least two profile families:

- A strict profile whose exact required operations and exceptional behavior are spelled out; reject targets that cannot satisfy it.
- Approximate profiles that name each allowed approximation and its domain/error relation. Different error relations are different capabilities.

Specify FMA contraction, reassociation, signed zero, infinities, NaNs, subnormals/FTZ, conversions, and transcendental functions. Storage support for FP16/BF16 does not imply arithmetic or matrix support. Do not assume a GLSL extended instruction has the same error bounds as a CUDA intrinsic.

For tests, use exact bit comparisons where the contract requires them, ULP/absolute/relative bounds only where permitted, and classification-sensitive cases for exceptional values. For nondeterministic reductions, compare the allowed result relation rather than requiring one accidental execution order.

Existing WGMMA specifications use an opaque hardware result relation. A scalar implementation can replace it only if it satisfies the selected contract; similarity to real matrix multiplication is insufficient. [Q9](10-sources.md)

## 6. Matrix acceleration and vendor features

Keep matrix multiply/load/store/fill semantic operations independent from concrete lane-register layouts. Query cooperative matrix shape/type/scope combinations and specialize accordingly. A device advertising the extension does not support every shape or input/accumulator combination. [E4](10-sources.md)

Handle existing mapped fragment loads, fused epilogues, packed layouts, and fragment access explicitly. Not every such operation maps directly to an opaque cooperative matrix API. Lower through explicit storage or a vendor extension only when its relation and cost are acceptable; otherwise report the missing feature.

WGMMA, asynchronous shared-memory pipelines, and instruction-specific data layouts are extension work. Preserve their expressiveness in the contract without presenting them as universally portable. NVIDIA-only features cannot be a prerequisite for the baseline Vulkan backend.

## 7. Backend sequence

| Backend | Purpose | Planned status |
|---|---|---|
| Existing CUDA/Karamel | Migration reference and legacy compatibility | Optional after cutover; still CUDA-dependent |
| Vulkan/SPIR-V | First production portable compute profile | Required |
| Metal via qualified SPIR-V → MSL translation | Distinct runtime/compiler package for additive-backend validation | Preferred second backend; requires Apple hardware qualification |
| Direct Mesa NIR | Evaluate benefits of bypassing SPIR-V serialization and import | Experimental research track |
| OpenCL `Kernel` | Possible future compute route | Requires explicit dialect/lowering work beyond current upstream SPIR-T scope |
| LLVM/native ISA/PTX/other APIs | Specialized future routes | Separate emitter, runtime, ABI, and evidence qualification |

SPIRV-Cross supplies SPIR-V reflection and MSL translation that can be evaluated for the Metal worker; it does not supply this project's runtime or correctness proof. Pin it and qualify the supported subset. [E7](10-sources.md)

Source-language independence does not prohibit one backend from privately generating MSL or another target language. It prohibits making that language a universal prerequisite or public extraction contract.

## 8. Direct SPIR-T → Mesa NIR track

Mesa NIR is a common optimizing compiler representation inside many Mesa driver compilers. It is not a universal GPU ISA or a public replacement for device submission APIs. [E6](10-sources.md)

Run a bounded experiment after the Vulkan path supplies a correctness and performance baseline:

1. Choose one Mesa revision, one driver, one device, and the baseline compute subset.
2. Implement direct conversion of SPIR-T control flow, types, memory operations, built-ins, and required metadata into NIR through a version-pinned adapter.
3. Integrate at a documented private driver/compiler boundary that can actually produce a loadable shader. Identify who performs layout, relocation, pipeline setup, and command submission.
4. Compare final machine code, runtime, compile latency, memory use, and optimization opportunities against the same driver reached through SPIR-V. Keep workload, numerical mode, and driver options equal.
5. Record information lost by the SPIR-V route. First test whether existing decorations, specialization, or a targeted importer improvement preserves it.
6. Promote the direct path only for measured benefits that justify its additional maintenance and validation cost.

Bypassing SPIR-V may improve compile latency or preserve useful information; it does not automatically improve GPU execution. Direct conversion to each hardware backend IR or ISA multiplies target-specific work and must be isolated in separate packages. Do not block the first production release on a full Mesa/compiler-driver rewrite.
