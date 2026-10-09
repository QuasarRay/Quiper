# Primitive lowering

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
| Proved assertions | Consume an established predicate | Erase only with the required relation | Never promote an unchecked assertion to proof |
| Runtime guards | Establish a predicate on successful return, with explicit failure | Qualified host/device failure lowering | No invalid effects or successful postcondition after failure; preserve collective participation |

P0 must expand this family table to every actual extraction case and emitted primitive. Operations absent from v1 receive an explicit unsupported classification, owner, and later milestone.
## 4. Shared memory and pointer legalization

CUDA dynamic shared-memory byte allocation and Vulkan workgroup storage are not interchangeable API calls. Determine which sizes can be specialized before pipeline creation. Cache specialized variants under the exact size/layout tuple. If a runtime size cannot be legally represented, reject it or use a separately proved transformation; do not silently move shared storage to global memory.

Use KIR resource layouts as the authority. Check that SPIR-T `QPtr` configuration, SPIR-V layout decorations, runtime descriptor ranges, and binding code agree. Internal abstract pointer sizes are not host/device pointer sizes. The current pointer implementation contains 32-bit offset/extent fields; reject out-of-range cases until a wider qualified path exists. [S4](../10-sources/README.md)

Test mixed-type shared storage, unaligned slices, aggregate padding, zero-sized/empty cases, vector tails, and maximum offsets. Any layout optimization that changes a proof-visible address relation needs a corresponding refinement argument.
