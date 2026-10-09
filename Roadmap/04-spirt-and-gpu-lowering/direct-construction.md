# Implement the SPIR-T worker and direct construction

## 1. Freeze the initial target profile

The initial Vulkan worker targets **Vulkan 1.2 and SPIR-V 1.5**, `GLCompute`, logical addressing, and the Vulkan memory model. Require/query/enable `vulkanMemoryModel`; require `vulkanMemoryModelDeviceScope` when emitting device-scope operations. This is a selected implementation profile, not a claim that every Vulkan 1.2 device supports all optional features. Baseline entrypoints use literal local sizes. Materialize static specialization values before emission; do not rely on `LocalSizeId` in the unpatched baseline.

Start P3 with 32-bit integer arithmetic, Boolean control, logical storage buffers, explicit views, and independently checked resource layouts. Add floating, shared-memory, atomic and subgroup operations only after their semantic lowerings are ready. Full portable release coverage still requires the declared numerical/workload suite; the small P3 subset is not migration completion.

Keep each required optional feature/limit in the artifact requirements. Pin SPIRV-Headers, SPIRV-Tools, the Rust toolchain, SDK, SPIR-T revision and local patch digest. Use upstream `e8757adba8d14068a7bf1b3bc9f24cac982f4bd3` as the investigated baseline. Every different dependency/patch combination is a distinct compiler identity requiring regression evidence.

## 2. Implement one compilation session

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

## 3. Construct the module directly

Use [the entity and operation mapping](operation-mapping.md) for exact upstream representation names, entrypoint/resource ABI decisions and the first vertical slice. Keep that adapter private to the compiler package.

1. Initialize `Module::new` with the session context, `ModuleDialect::Spv` and the corresponding `ModuleDebugInfo::Spv`. Populate the pinned dialect's version, capabilities/extensions and memory-model fields according to the selected environment. Direct construction still uses SPIR-V-related concepts in this version of SPIR-T.
2. Intern `TypeDef`, `ConstDef` and `AttrSetDef` values using the pinned context API. Maintain structural type/layout keys; never persist interned handles or reuse a handle from another context.
3. Populate `global_vars` for declared storage/workgroup resources and built-in invocation inputs. For each resource, retain a mapping from logical resource ID to global variable, descriptor binding, layout, range and permissions. Host allocation handles never enter the module.
4. Declare functions and their parameter/result signatures in `funcs`. Resolve calls by full semantic identity. Create `FuncDefBody` regions, nodes and `DataInstDef` instructions with explicit types. Follow the pinned region/value ownership model rather than assuming an LLVM block API.
5. Populate exports using the intended entrypoint and interface globals. Compute the interface set from reachable use and verify it against the target environment. Reject unresolved executable imports and unreachable evidence references.
6. Attach semantic decorations and execution modes through the adapter's supported annotation mapping. Keep debug locations separately. Store KIR correspondence outside opaque debug-only annotations so debug stripping cannot delete a requirement.

Use the official [API index](https://rust-gpu.github.io/spirt/spirt/index.html), [IR definitions](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/lib.rs), and [conversion implementation](https://github.com/Rust-GPU/spirt/tree/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/spv) as the construction reference. Write a small compile-checked adapter example against the pinned revision before the full exporter; the roadmap's algorithm is not claimed to be compiled Rust.

## 4. Map the initial operations

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

## 5. First useful fixtures

Construct an integer add kernel without any source-language frontend to test the KIR boundary, then compile an exported Kuiper elementwise kernel through the same worker. Compare outputs, layouts and requirements. Add slices, control flow, and multiple entrypoints progressively. Reproduce construction from canonical bytes and pinned tools. Keep the frontend-free fixture small; the actual Kuiper corpus determines qualification.
