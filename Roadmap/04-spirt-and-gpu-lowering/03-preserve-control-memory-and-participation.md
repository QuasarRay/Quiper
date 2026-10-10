# Preserve loop values, memory order and participation

**Milestone M15.** Qualified loop/backedge/final-value mapping and memory/atomic/subgroup lowerings with negative fixtures.

## Required inputs and specification

Start from [M14](02-run-the-integer-vertical-slice.md). Use the exit-gate dependencies in [the implementation plan](../milestones.json); proof and investigation work may begin earlier. Read [the declarative specification](../Specification/README.md) before choosing representation details. This milestone implements `Lowering.while_exit`, `Memory.admissible`, `Memory.convergent`, `Lowering.sufficient`.

## Establish the control-flow relation first

Represent structured selections/loops directly when possible. Preserve condition evaluation order, carried values, exits, effects and participating invocations. If the importer supplies CFG/SSA, call the pinned `passes::legalize::structurize_func_cfgs` only after checking the accepted subset and recording the input relation. The official [pass source](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/passes/legalize.rs) exposes that entrypoint; it does not prove Kuiper-specific barrier preservation.

Resolve executable imports and determine reachability before final validation. The upstream [round-trip example](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/examples/spv-lower-link-lift.rs) demonstrates APIs and diagnostics. Its pass order is not a normative Kuiper sequence. Declare pre/postconditions for each scheduled pass, including whether it can introduce pointer merges, duplicate control conditions, or alter uniformity.

The pinned [`NodeKind::Loop`](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/lib.rs#L862-L920) is tail-controlled: it executes the body before testing its repeat condition. A source `while` cannot be copied into that node unchanged. Implement an initial condition/selection around the loop or another checked transformation that preserves zero iterations, side effects and carried values. Evaluate each source condition at its required point; duplicating a condition with effects changes behavior. For early exits and nested loops, specify how every carried output is selected, and reject shapes outside the proved/validated mapping. Use separate fixtures for zero iterations, one iteration, condition effects, early exits and nested carried state. This representation issue is distinct from the unconfirmed lifting report below.

## Triage the loop report before enabling affected paths

Treat [issue #31](https://github.com/Rust-GPU/spirt/issues/31) and [draft PR #30](https://github.com/Rust-GPU/spirt/pull/30) as an unresolved qualification dependency. The inspected proposal head is `0e40966f27468d4896b51e28ea0e9080a8300bbe`.

1. Obtain the minimized fixture from that exact proposal and build baseline/patched workers independently with their pinned tools.
2. Establish the defined behavior of the input. The presence of an undefined value on an unreachable path is not itself proof of a bug.
3. Run the exact lower/structurize/lift pipeline and compare the allowed observable outcomes, not only pretty-printed shapes or validator success.
4. Test whether direct KIR construction reaches the same problematic structure. Direct construction does not automatically exclude lifting bugs.
5. Accept a reviewed fix with regression evidence, prove and enforce an exclusion, or document why the report is inapplicable. Until then, block affected control shapes. Never call the report reproduced solely from reading it.

Include zero/one/multiple iterations, early exits, nested control, duplicated predicates, loop-carried values and barrier participation. A bounded unrolled subset can be a temporary path only with its own preservation/size bounds; it does not qualify general loops.

## Use logical resource accesses as the baseline

The first native KIR path keeps resource identity plus integer offset/extent. Generate typed storage-buffer access chains directly. Specialize helpers on resource identity or inline them within a bounded budget; keep ordinary scalar arguments as values. Do not introduce physical pointers, arbitrary pointer reinterpretation, variable-pointer capabilities, or descriptor-indexing requirements merely to mimic CUDA pointer syntax.

A select between offsets in one resource differs from selecting between resources. The latter needs an explicitly supported binding/selection lowering and capability checks. Reject it if unsupported. Validate offset arithmetic in a sufficiently wide checked representation before narrowing to the selected shader index width. Empty logical views do not authorize zero-sized Vulkan allocations or skipped side effects; handle them using a specified host-plan transformation or valid backing representation.

## Make QPtr an eligible optional path

QPtr is not a mandatory pass for already typed logical-buffer lowering. Enable it for an explicitly qualified importer/operation subset. Before entry, reject or legalize the known restrictions:

| Restriction at the pin | Source | Required handling |
|---|---|---|
| Certain pointer-valued region/node merges | [analysis](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/qptr/analyze.rs#L883-L895) | Prove elimination, add a qualified lowering, or reject before QPtr |
| Calls with pointer arguments | [lifting](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/qptr/lift.rs#L450-L458) | Bounded inlining/specialization, a reviewed implementation, or rejection |
| Load/store memory operands | [lowering](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/qptr/lower.rs#L421-L434) | These instructions remain unchanged; validate the mixed form and preserve operands |
| 32-bit internal offsets/extents | [model](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/qptr/mod.rs) | Range-check every relevant value; no truncation or host-pointer-width inference |

Recheck eligibility after any pass that changes calls/control/memory. Check all emitted diagnostics before declaring success; an attached diagnostic is not an approved artifact. Never mask an unsupported internal case with a broad catch-and-continue path.

## Check memory and convergence together

Map workgroup barriers to execution participation plus memory ordering over the actual affected storage classes. Map subgroup operations only for qualified subgroup-size/participation guarantees; the old fixed warp width does not prove portability. Atomic lowering preserves operation, type, address space, scope, ordering and numeric relation as a tuple. A CAS emulation needs its progress/NaN/order argument and must not silently replace an incompatible primitive.

Keep guards distinct from assertions as described in [guard failure semantics](../05-runtime-and-interop/02-make-memory-and-bindings-safe.md). No optimization may move effects across barriers or erase a potentially failing guard without an appropriate proof. These are O4–O7 obligations, supported by litmus/model exploration and real-device tests, not discharged by SPIR-V format validation.

## Carry loop values through the pinned representation correctly

Implement V2-04 against [RegionDef and NodeDef at the pin](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/lib.rs#L779-L920). The channels are distinct:

| Source meaning | SPIR-T representation |
|---|---|
| Initial loop state | `Loop.initial_inputs`, matching `body.inputs` in arity and type |
| Current iteration state | `Value::RegionInput` of the loop body |
| State for the next iteration | Body-computed values in `body.outputs`; these feed the backedge |
| Whether to repeat | Body-defined `repeat_condition`, evaluated after the body |
| State after a positive iteration count | Dominating body-defined final values used after the loop at this pin |
| State after zero or more source iterations | Outer `Select` result: false arm yields initial values; true arm yields the final body-defined values |

Do not create `Value::NodeOutput` for a Loop. A `Select` has node outputs; a Loop's body outputs are backedge arguments. Maintain an explicit final-value map alongside the backedge map. A body input can still denote the old value during the last iteration; select the updated value that the source returns.

For a pure source condition, evaluate it once before the outer selection and once at the required end-of-iteration point for each positive iteration. For an effectful condition, lower its computation as explicit state/effects at those same points and carry its updated state; a Boolean expression copied twice is not equivalent. Model break/continue/return with explicit control tags and value tuples, then prove/validate their selection and participation mapping. Until that mapping is qualified, reject those shapes rather than claiming all structured loops work.

Implement `Lowering.while_exit`, `valid_carried` and `Kernel.executes` correspondence. The included zero/positive exit lemmas establish value selection only, not a proof of the SPIR-T library or a complete while-to-loop transformation. Compile and execute fixtures returning a modified accumulator after 0, 1 and multiple iterations, plus nested loops and supported early exits. Check the actual final values and absence of Loop node-output references in addition to target validation.

## Primitive migration matrix

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
## Shared memory and pointer legalization

CUDA dynamic shared-memory byte allocation and Vulkan workgroup storage are not interchangeable API calls. Determine which sizes can be specialized before pipeline creation. Cache specialized variants under the exact size/layout tuple. If a runtime size cannot be legally represented, reject it or use a separately proved transformation; do not silently move shared storage to global memory.

Use KIR resource layouts as the authority. Check that SPIR-T `QPtr` configuration, SPIR-V layout decorations, runtime descriptor ranges, and binding code agree. Internal abstract pointer sizes are not host/device pointer sizes. The current pointer implementation contains 32-bit offset/extent fields; reject out-of-range cases until a wider qualified path exists. [S4](../10-sources/README.md)

Test mixed-type shared storage, unaligned slices, aggregate padding, zero-sized/empty cases, vector tails, and maximum offsets. Any layout optimization that changes a proof-visible address relation needs a corresponding refinement argument.

## Admit the complete Vulkan memory-model requirement

Implement V2-05 with a semantic requirement `uses_visibility_chains`, not just an emitted-opcode scan. [Vulkan 1.2 features](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVulkan12Features.html) separately defines `vulkanMemoryModel`, `vulkanMemoryModelDeviceScope` and `vulkanMemoryModelAvailabilityVisibilityChains`. The artifact records all required features; the runtime verifies they are supported and enabled.

Use `Lowering.sufficient` and prove that the requirement accumulator is complete for the selected memory relation. The initial conservative concurrency profile requires all three features when it admits chain-dependent programs. A chain-free profile may omit the third only with a proved and enforced restriction on source programs, lowering and transformations. Queue order, an unused-looking feature bit, or absence of a particular opcode does not establish that restriction.

Run three capability fixtures: missing base memory model; device scope absent for a device-scoped operation; chains absent for a chain-dependent program. Each must reject before dispatch. Run direct and multi-hop publication litmus cases on claimed hardware, and bind O6 evidence to the exact enabled feature set. Recheck after every pass that changes synchronization.

## Evidence required to close this milestone

Close **G-CONCURRENCY, O4–O6** only with the implementation artifacts, positive and negative cases, and source-to-result identities required above. Link the implementation relation to the named F* symbols and the policy's applicable O1–O10 obligations. The specification's proved lemmas are reusable model facts; they do not discharge this implementation correspondence. Record unresolved cases as blockers or explicitly outside the claim. No backend gate is marked passed by this roadmap revision.
