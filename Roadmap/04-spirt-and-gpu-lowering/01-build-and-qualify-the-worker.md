# Build and qualify a pinned SPIR-T worker

**Milestone M13.** A compile-checked private adapter, pinned dependencies, tested annotation subset and justified loop/QPtr eligibility decisions.

## Required inputs and specification

Start from [M03](../00-current-state-and-gaps/01-freeze-the-semantic-inventory.md), [M06](../01-target-architecture/02-freeze-portable-artifacts.md). Use the exit-gate dependencies in [the implementation plan](../milestones.json); proof and investigation work may begin earlier. Read [the declarative specification](../Specification/README.md) before choosing representation details. This milestone implements `Refinement.refines`, `Lowering.baseline_annotation_supported`.

## First production path

Implement a Vulkan compute backend first. Use the proposed Vulkan 1.2 / SPIR-V 1.5 profile in the direct-construction procedure; ratify its exact optional features and device matrix in P0, record its digest, and validate against it. Do not target an unspecified “latest SPIR-V.” Optional features and extensions must be individually queried and enabled. [E3](../10-sources/README.md)

The backend worker reads KIR, constructs SPIR-T directly through a pinned adapter, applies a reviewed pipeline, emits SPIR-V, validates it, prepares reflection/layout metadata, and packages it for the Vulkan runtime. The current upstream `spv::lower` name means SPIR-V → SPIR-T; `spv::lift` is the reverse. Keep that terminology unambiguous in implementation. [S2–S3](../10-sources/README.md)

A serialized SPIR-V detour on input is useful for differential/round-trip tests but is not required for direct construction. A particular backend may emit SPIR-V on output. Neither statement implies SPIR-T is itself a hardware execution API.
## Required lowering sequence

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

Pass ordering is a contract. Do not blindly copy an example's `QPtr` sequence or layout constants: the inspected example is exploratory and even leaves its final write disabled. [S4](../10-sources/README.md)

## Maintain a version decision record

Record main plus each selected local/upstream patch, its prerequisite commits, rationale, owner, tests and removal/update condition. Investigate the open memory/control-flow/scalar/vector/interpreter work before duplicating infrastructure. The [SPIR-T repository documentation](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/README.md) identifies the project as evolving and its textual display as non-interchange output.

The prototype interpreter and Vulkan round-trip layer may help testing. Their existence does not qualify them as a host executor or independent semantic oracle. Keep the small KIR reference semantics separate from compiler transformations whose bugs the tests are meant to detect. Do not adopt a whole unmerged stack without checking each dependency and regression surface.

## Run a layered corpus

1. **Representation:** types/constants, aggregates, multi-entrypoint modules, symbol imports, canonical identity and annotations.
2. **Scalar/control:** exact integer edges, casts, conditionals, loops, exits, nested calls and invalid/uninitialized inputs.
3. **Memory:** views, strides, offsets, aliasing, initialization, shared storage, tails, zero/maximum sizes and overflow.
4. **Concurrency:** barrier participation, subgroup constraints, atomics, visibility, dependent queues and failure paths.
5. **Numerical:** operation-specific exceptional values, reductions, softmax/log-softmax, dense/sparse GEMM, qualified matrix variants.
6. **Integration:** C/Rust bindings, identical artifacts, clean CUDA-free installation, cache integrity, cancellation, device loss and package rollback.

Use an independent source/KIR oracle, metamorphic relations with justified preconditions, legacy CUDA comparisons, and real-device results as complementary evidence. Minimize failures into pinned regressions. Ordinary agreement on sampled inputs does not discharge universal preservation obligations.

## Upgrade without changing the core

Build the updated private worker as a new package. Keep old core/contracts and existing runtime packages installed. Replay annotation, loop, QPtr, layout, numeric, evidence and full corpus tests. Re-run affected extension gates for the candidate identity. Keep old worker versions loadable for supported old packages until their declared support window ends.

## Treat future targets as independent emitters

For Metal, keep SPIR-V-to-MSL translation and native runtime work inside the new package and qualify binding/resource semantics separately. Select the candidate and secure hardware in P0; P7 performs the independent implementation exercise. An unavailable second API blocks G-ADD-RUNTIME rather than being replaced by a second Vulkan vendor.

For direct Mesa NIR, implement a separate version-pinned emitter/runtime integration experiment. Compare against the same driver's SPIR-V route with identical semantic modes and workloads. Measure compile cost, generated code, execution, maintenance surface and correctness. Keep it optional until evidence justifies promotion. Other native ISA backends also need runtime/loader/ABI work; ISA bytes alone are not a backend.

## Freeze the initial target profile

The initial Vulkan worker targets **Vulkan 1.2 and SPIR-V 1.5**, `GLCompute`, logical addressing, and the Vulkan memory model. Require/query/enable `vulkanMemoryModel`; require `vulkanMemoryModelDeviceScope` when emitting device-scope operations. Require/query/enable `vulkanMemoryModelAvailabilityVisibilityChains` whenever the selected semantic mapping relies on multi-element availability/visibility chains. This is a selected implementation profile, not a claim that every Vulkan 1.2 device supports all optional features. Baseline entrypoints use literal local sizes. Materialize static specialization values before emission; do not rely on `LocalSizeId` in the unpatched baseline.

Start P3 with 32-bit integer arithmetic, Boolean control, logical storage buffers, explicit views, and independently checked resource layouts. Add floating, shared-memory, atomic and subgroup operations only after their semantic lowerings are ready. Full portable release coverage still requires the declared numerical/workload suite; the small P3 subset is not migration completion.

Keep each required optional feature/limit in the artifact requirements. Pin SPIRV-Headers, SPIRV-Tools, the Rust toolchain, SDK, SPIR-T revision and local patch digest. Use upstream `e8757adba8d14068a7bf1b3bc9f24cac982f4bd3` as the investigated baseline. Every different dependency/patch combination is a distinct compiler identity requiring regression evidence.

## Implement one compilation session

Create a private worker crate/executable that implements the compiler protocol. Its session owns a fresh `Rc<spirt::Context>`, one `spirt::Module`, KIR-to-SPIR-T maps, layout tables, requirement accumulator, pass ledger, and evidence correspondence. Do not share the context between threads; use independent sessions/processes for parallel work. The official [Module documentation](https://rust-gpu.github.io/spirt/spirt/struct.Module.html) and [pinned source](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/lib.rs#L198-L246) establish these API/context properties.

The following is an implementation algorithm, not an existing Kuiper API:

```text
decode and check KIR + evidence policy
resolve imports and admitted semantic extensions
fix target, layouts, numeric policy and specialization tuple
construct module dialect/debug metadata and intern types/constants
declare resources and function signatures
construct function regions/instructions and correspondence maps
run only eligible legalizations/optimizations, checking each result
finalize entrypoints, requirements, layouts and execution modes
lift to target bytes, validate, reflect and check evidence binding
publish the complete target package atomically
```

Each step either returns a complete checked result or a structured error. Clear session-local state on failure. Partial IR or target bytes may be retained only as explicitly failed debug artifacts, never as executable cache entries.

## Construct the module directly

Use [the entity and operation mapping](02-run-the-integer-vertical-slice.md) for exact upstream representation names, entrypoint/resource ABI decisions and the first vertical slice. Keep that adapter private to the compiler package.

1. Initialize `Module::new` with the session context, `ModuleDialect::Spv` and the corresponding `ModuleDebugInfo::Spv`. Populate the pinned dialect's version, capabilities/extensions and memory-model fields according to the selected environment. Direct construction still uses SPIR-V-related concepts in this version of SPIR-T.
2. Intern `TypeDef`, `ConstDef` and `AttrSetDef` values using the pinned context API. Maintain structural type/layout keys; never persist interned handles or reuse a handle from another context.
3. Populate `global_vars` for declared storage/workgroup resources and built-in invocation inputs. For each resource, retain a mapping from logical resource ID to global variable, descriptor binding, layout, range and permissions. Host allocation handles never enter the module.
4. Declare functions and their parameter/result signatures in `funcs`. Resolve calls by full semantic identity. Create `FuncDefBody` regions, nodes and `DataInstDef` instructions with explicit types. Follow the pinned region/value ownership model rather than assuming an LLVM block API.
5. Populate exports using the intended entrypoint and interface globals. Compute the interface set from reachable use and verify it against the target environment. Reject unresolved executable imports and unreachable evidence references.
6. Attach semantic decorations and execution modes through the adapter's supported annotation mapping. Keep debug locations separately. Store KIR correspondence outside opaque debug-only annotations so debug stripping cannot delete a requirement.

Use the official [API index](https://rust-gpu.github.io/spirt/spirt/index.html), [IR definitions](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/lib.rs), and [conversion implementation](https://github.com/Rust-GPU/spirt/tree/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/spv) as the construction reference. Write a small compile-checked adapter example against the pinned revision before the full exporter; the roadmap's algorithm is not claimed to be compiled Rust.

## Map the initial operations

| KIR meaning | Initial target construction | Preconditions/evidence |
|---|---|---|
| Width-specific integer constant/arithmetic | SPIR-V integer types/constants and matching arithmetic/comparison operations in SPIR-T | Width, signedness, overflow/division/shift contract matches |
| Boolean control value | Logical Boolean type and structured selection | Storage encoding handled separately |
| Invocation coordinates | Declared built-ins and explicit coordinate conversions | Dispatch bounds and multiplication overflow checked |
| Logical buffer access | Storage-buffer block plus typed access chain and load/store | Descriptor/layout agreement; valid resource identity/offset/alignment/permission |
| Private scalar reference | SSA value or function-local storage | Initialization and nonescape; state relation retained when promoted to SSA |
| Workgroup storage | Statically sized per-workgroup globals | Concrete layout/size; participation and barriers where needed |
| Host allocation/submit/wait | No shader instruction | Remains in the host plan/runtime protocol |

Every unmapped operation is a public unsupported-feature diagnostic before emission. Do not let upstream acceptance of a generic opcode substitute for target-environment or semantic qualification.

## First useful fixtures

Construct an integer add kernel without any source-language frontend to test the KIR boundary, then compile an exported Kuiper elementwise kernel through the same worker. Compare outputs, layouts and requirements. Add slices, control flow, and multiple entrypoints progressively. Reproduce construction from canonical bytes and pinned tools. Keep the frontend-free fixture small; the actual Kuiper corpus determines qualification.

## Evidence required to close this milestone

Close **G-SPIRT-FEASIBILITY** only with the implementation artifacts, positive and negative cases, and source-to-result identities required above. Link the implementation relation to the named F* symbols and the policy's applicable O1–O10 obligations. The specification's proved lemmas are reusable model facts; they do not discharge this implementation correspondence. Record unresolved cases as blockers or explicitly outside the claim. No backend gate is marked passed by this roadmap revision.
