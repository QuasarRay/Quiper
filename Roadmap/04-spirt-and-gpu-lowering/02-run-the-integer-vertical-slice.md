# Run an integer kernel through SPIR-T and Vulkan

**Milestone M14.** An emitted and validated shader, independent reflection, runtime execution and reference comparison for real Kuiper views.

## Required inputs and specification

Start from [M07](../02-language-independent-extraction/01-implement-the-neutral-language.md), [M08](../02-language-independent-extraction/02-export-a-checked-kuiper-program.md), [M11](../03-backend-extension-contract/02-match-artifacts-to-independent-runtimes.md), [M13](01-build-and-qualify-the-worker.md). Use the exit-gate dependencies in [the implementation plan](../milestones.json); proof and investigation work may begin earlier. Read [the declarative specification](../Specification/README.md) before choosing representation details. This milestone implements `Kernel.executes`, `Foundation.well_typed`, `Lowering.layout_valid`.

## Build one audited adapter around the pinned API

The [pinned IR definitions](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/lib.rs) and [SPIR-V dialect definitions](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/spv/mod.rs) are the API reference for this table. Names below are upstream symbols; the KIR mapping and helper functions are new implementation work.

| Portable entity | SPIR-T construction | Adapter invariant |
|---|---|---|
| Module target | `ModuleDialect::Spv(spv::Dialect)` | Version, capabilities, extensions, addressing model and memory model agree with the selected profile |
| Scalar/vector/aggregate type | Intern `TypeDef` with `TypeKind::SpvInst` and its type/constant dependencies | One context; explicit widths/layout; dependency IDs never serialized as portable handles |
| Typed constant | Intern `ConstDef` with its type and `ConstKind::SpvInst` | Bit pattern, signedness and type preserved; use `PtrToGlobalVar` only for the corresponding declared resource |
| Resource | `GlobalVarDecl`, its pointer/value type, storage class and attributes | Resource identity, binding, layout, permission and lifetime still connected to the portable descriptor |
| Function | `FuncDecl` plus `FuncDefBody` | Parameter order, result type, call effects and body-region inputs agree |
| Arithmetic/access operation | `DataInstDef` with `DataInstKind::SpvInst`, inputs and optional output type | Result is present only when the instruction produces one; typed operands and effects checked |
| Call | `DataInstKind::FuncCall` | Full signature resolved; permitted call graph and qualified pointer-argument path |
| Extended math instruction | `DataInstKind::SpvExtInst` | Named instruction set and operation relation; no inferred CUDA-equivalent accuracy |
| Selection | `NodeKind::Select`, cases and result values | Branch order and output arities/types agree; effects remain conditional |
| Loop | `NodeKind::Loop`, initial inputs, region outputs and repeat condition | Tail-controlled mapping preserves source zero-iteration and early-exit behavior |
| Entrypoint | `ExportKey::SpvEntryPoint` to `Exportee::Func` | Correct execution model/name/interface globals and execution modes |
| Annotation | Interned attributes using the accepted annotation representation | Required semantics survive every visitor/transform/emitter; unsupported ID operands rejected |

Use the pinned SPIR-V grammar/spec helpers for operand kinds and instruction numbers; do not scatter handwritten numeric opcodes throughout the exporter. `spv::Inst` carries immediate operands, while the enclosing type/constant/data instruction carries its referenced entities/values. Mixing those channels is a construction error even if a debug print looks plausible.

Write adapter-level constructors that validate these invariants and return structured errors. Validate a completed function/module before entering the next pass. Collect error diagnostics attached to attributes as well as returned errors. Assertions or panics from malformed external input must terminate the failed worker request cleanly under the declared process policy; they must not publish partial output.

## Fix the resource and entrypoint ABI before emitting accesses

The first shader entrypoint uses global interface resources rather than a CUDA-style parameter pointer list. For each exported kernel, construct a deterministic binding table from logical resource IDs. Give scalar parameters an explicit parameter-buffer layout, and define descriptor set/binding numbers in the target-artifact ABI. These choices belong to the artifact contract, not to source-language naming conventions.

For Vulkan storage buffers, define a block structure, member offsets and array stride under a selected legal layout policy. A simple first policy uses ordinary Vulkan storage-buffer base alignment rules, with 32-bit storage values and no dependency on optional scalar-block-layout features. Encode Boolean storage explicitly, such as a documented 32-bit 0/1 representation, and convert to/from shader Boolean values. Float/vector/matrix layouts require separate enabled features and layout checks when introduced.

Represent a view as binding/resource plus checked byte/element offset and extent. Account for descriptor-offset alignment, descriptor range and resource limits. If a slice offset cannot be expressed directly as an aligned descriptor offset, bind a valid containing range and perform the checked internal offset calculation, or reject it. No implicit rebasing may enlarge access permissions. The runtime and shader must agree on offset units and arithmetic widths.

Derive invocation coordinates from declared built-ins. Compute dispatch dimensions with overflow-safe arithmetic, validate workgroup-size and group-count limits, and specify tail-lane behavior. For barrier-free elementwise kernels, a bounds selection can suppress out-of-range accesses. For cooperative kernels, inactive data lanes may still need to participate in collectives; a blanket early return is not generally valid.

The normative environment and layout restrictions are in the [Vulkan SPIR-V environment](https://docs.vulkan.org/spec/latest/appendices/spirvenv.html). The proposed ABI must be tested against an independent SPIR-V reflector, the runtime's descriptor setup and actual transfers.

## Implement and qualify each operation family

1. Implement constants, width-specific arithmetic, comparisons and casts. Distinguish signed/unsigned division and comparisons, modular arithmetic from checked arithmetic, and source-defined shift/division edges from target undefined behavior. Reject or check unsafe operands before the operation.
2. Implement selections and tail-controlled loops using the relation in [control and pointers](03-preserve-control-memory-and-participation.md). Carry every live value explicitly. Permit only the qualified recursion/call subset; inline helpers only within a bounded budget and with updated evidence.
3. Implement loads/stores using typed access chains, explicit layouts and retained resource permissions. Preserve required memory operands; do not route them through an unqualified partial QPtr conversion.
4. Implement workgroup storage and barriers together. Derive execution scope, memory scope and memory-semantics operands from the source primitive and affected storage classes. A control barrier is not interchangeable with a memory-only barrier. Validate participation as well as visibility.
5. Implement atomic tuples and subgroup operations only when their type, operation, storage, scope, order and participation are supported. Strengthening an ordering may require a progress/performance analysis; weakening it requires a proof and is otherwise rejected.
6. Add floating operations, approximations and matrix families under their explicit numeric/shape relations. Require feature enablement and emitted controls. Match input/storage/accumulator formats independently.
7. Implement proved assertion elimination and guarded success/failure as distinct paths. Run the guard/collective cases before enabling device guards in production.

Each family adds positive and negative fixtures, KIR-to-target correspondence, emitted requirement/reflection checks and relevant O4–O7 evidence. The accepted operation catalog is generated from those qualified mappings; accepting an arbitrary upstream opcode does not expand the advertised catalog.

## Complete a reproducible vertical slice

Use a bounded integer elementwise kernel with two input views, an output view and a retained length. Construct KIR directly, then through the F*/Pulse exporter. Require the same ABI/semantics, not necessarily identical debug data. Freeze the layouts, emit literal local size, validate the module, prepare descriptors/pipeline, upload, dispatch, wait, establish visibility and compare against an independent reference.

Exercise zero length, a tail group, nonzero view offsets, maximum supported size, invalid overlap and insufficient device limits. Zero-length handling must preserve any specified host effects even if no dispatch is needed. Record both rejection and success behavior. Extend to an actual Kuiper array/view kernel, then dependent launches, then a reduction/matrix workload after its synchronization/numeric relations are ready. This is the implementation path for G-VERTICAL; no such execution is claimed by this documentation revision.

## Close the annotation gap explicitly

The pinned [SPIR-V input bridge](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/spv/lower.rs#L478-L496) rejects annotations with ID operands beyond the target. The current attribute/emission representation also needs changes for those operands. [PR #48](https://github.com/Rust-GPU/spirt/pull/48), inspected at `94f5c19c3d3ad258bc628857a350719561d64b44`, is a candidate patch, not a qualified dependency.

The initial baseline therefore specializes values before construction and emits supported literal local sizes and supported literal controls. Reject unsupported ID-bearing annotation requirements. A separate compiler profile may adopt a reviewed patch or private adapter implementation, with its own digest and tests. If it emits `LocalSizeId`, require the Vulkan `maintenance4` feature and a compatible target environment; follow the [runtime SPIR-V rule](https://docs.vulkan.org/refpages/latest/refpages/source/RuntimeSpirv.html).

Test import and direct construction separately. Track referenced types/constants through clone, visit, transform and emission paths. Renumber or rewrite a referenced constant and verify that the annotation still denotes the intended value. Unsupported forms must not be accepted with the annotation omitted.

## Derive the final requirements from emitted behavior

After all lowering, compute the actual types, capabilities, extensions, memory model, execution modes, entrypoint interfaces, resource bindings and specialization values. Compare them with KIR requirements and the selected target profile. Requirements introduced by a legalization are new obligations, not assumptions that the runtime will happen to satisfy.

Reflect entrypoint arguments, descriptor binding/type/count, storage layout, workgroup memory use and local sizes. Check these against the host ABI/dispatch plan. An internal reflection structure agreeing with another structure generated by the same faulty code is weak evidence; also inspect the final emitted module using an independent reader.

Use `Module::lift_to_spv_module_emitter` or `lift_to_spv_file` through the private adapter. These are upstream API methods. Decode emitted bytes to confirm that all required annotations and interfaces survived. The output file is a sequence of target words; the runtime must respect the target API's size/alignment/endianness requirements when loading it.

## Validate at several boundaries

Run pinned SPIRV-Tools with the exact target environment, initially `vulkan1.2`, after emission and after any later SPIR-V optimization. Validate structural KIR, SPIR-T diagnostic state, final environment, reflection/ABI agreement and evidence correspondence separately. The official [SPIRV-Tools documentation](https://github.com/KhronosGroup/SPIRV-Tools) explains that its validator is incomplete; acceptance there is a required diagnostic gate, not a semantic proof.

Maintain positive/negative fixtures for missing features, wrong storage layouts, invalid entrypoint interfaces, changed local sizes, removed FP controls, unresolved imports, unknown required extensions and stale evidence. Record rejected invalid inputs as expected tests, not skipped coverage.

## Qualify numerical operations and matrix lowering

Define operation-specific relations before enabling a numeric profile. For an exact binary operation, specify the chosen rounding and exceptional behavior; for an approximate operation, specify domain and error relation. Query [float-control properties](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceFloatControlsProperties.html), then check the actual operation/control combination. Feature presence alone does not prove the requested relation.

Disable contraction/reassociation unless the selected semantics permit it and emitted controls preserve it. For reductions, define the permitted order/result set. Preserve the distinction between storage width, arithmetic width and accumulator width. Classify NaNs, signed zeros, infinities, subnormals, overflow, casts and cancellation cases in the fixtures. Use an independent high-precision/reference calculation where applicable; CUDA agreement alone is insufficient.

Keep matrix operations in semantic form until a qualified target combination is selected. Check supported shape/type/scope tuples, memory layouts and participation. Decompose only with evidence that the requested relation is preserved. WGMMA-specific packed/async/numerical behavior belongs to a vendor extension; a scalar GEMM result does not automatically implement it. Unsupported mandatory vendor semantics keep G-REPLACEMENT blocked.

## Publish an indivisible package

Package target bytes, portable-parent identity, artifact-format/ABI versions, reflection, requirements, specialization/numeric policy, compiler/pass/dependency digests, output hash, and accepted evidence. Compute cache keys from the complete inputs and publish through an atomic temporary-to-final transition after all required checks pass.

On load, verify hashes and repeat runtime-specific compatibility/feature checks. Reject a cache hit if evidence policy, semantic digest, backend version, target identity or relevant driver assumptions changed. Distinguish reproducible portable/device artifacts from driver-native caches whose identity is device/driver-specific.

## Evidence required to close this milestone

Close **G-VERTICAL** only with the implementation artifacts, positive and negative cases, and source-to-result identities required above. Link the implementation relation to the named F* symbols and the policy's applicable O1–O10 obligations. The specification's proved lemmas are reusable model facts; they do not discharge this implementation correspondence. Record unresolved cases as blockers or explicitly outside the claim. No backend gate is marked passed by this roadmap revision.
