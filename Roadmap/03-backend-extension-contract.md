# 03. Additive backend extension contract

## 1. Exact guarantee

After KIR/runtime protocol v1 is frozen, a compatible backend must be installable next to a released, immutable core. It may add its own package, build files, dependencies, capability definitions, lowering implementation, runtime adapter, tests, and qualification records. It may add deployment configuration selecting that package. It must not modify existing core/compiler/frontend/kernel files, central registration source, root build files, or another backend.

Existing applications expressing supported portable semantics must not require source changes. A new driver, package installation, compatible device, or changed deployment selection is allowed. An already generated device binary is not expected to become an executable for a different ISA; portable artifacts may be compiled again through the newly installed backend.

An out-of-tree backend is the strongest test. A folder added to a repository while also modifying `enum Backend`, `Cargo.toml`, `build.rs`, a switch statement, and a CI matrix does not satisfy the requirement.

## 2. Package roles and discovery

Discover packages from an explicit configured search path and a directory of versioned manifests. Initial core code must already implement discovery, compatibility negotiation, deterministic conflict handling, dependency resolution, and role dispatch.

Each manifest declares:

- Stable package identity, version, role set, semantic contract versions, and content digests.
- Compiler worker/runtime endpoint locations and protocols.
- Accepted KIR profiles and extensions; produced artifact formats and target environments.
- Required device features, limits, numeric modes, and runtime assumptions.
- Pass/lowering registrations, prerequisite analyses, effects, and evidence requirements.
- Dependencies, supported OS/architectures, driver constraints, licenses/notices, and qualification status.
- Resource budgets, timeout policy, diagnostic catalog, and reproducibility settings.

Installation is explicit. Compiling a source file must not fetch and run a plugin suggested by that file. Resolve duplicate identities and conflicting implementations using recorded configuration; never depend on filesystem enumeration order. Package signatures establish provenance only, not semantic correctness.

The build/test system discovers the same manifests. Backend packages build independently; generic CI can generate a test matrix from installed manifests and available runner labels. Administrators may add hardware runners and configuration without modifying dispatch logic. Do not bake vendor names into the core workflow.

## 3. Compiler boundary

Use a versioned request/response protocol between the orchestrator and compiler workers. Requests carry KIR bytes/digests, the requested target profile, optimization policy, evidence policy, specialization values, and immutable dependency identities. Responses carry artifacts, complete requirements, diagnostics, provenance, and checked evidence results.

Each compiler worker privately owns its SPIR-T context and links a pinned `compiler/spirt` adapter SDK. Its vendor emitter can operate directly on SPIR-T within that worker. **SPIR-T objects do not cross the stable process boundary.** This permits independently built workers to use different compatible SPIR-T versions without recompiling the core.

Shared passes can be delivered as version-pinned SDK packages inside a worker. An independently installed pass that crosses process boundaries must use a specified KIR form or another explicitly versioned, validated representation. Do not invent a stable SPIR-T binary format by serializing its current Rust internals.

The protocol needs cancellation, bounded messages, streaming of large artifacts, deterministic request IDs, progress, and structured errors. A worker crash is a compilation failure; it cannot leave a successful partial package in the cache. Process isolation also matches SPIR-T's current context ownership constraints. [S2](10-sources.md)

## 4. Runtime boundary

Specify a transport-independent runtime service contract covering device enumeration, feature queries, allocation, mapping, transfers, module loading, pipeline preparation, dispatch, dependencies, event completion, resource release, and device-loss handling.

Provide two transports:

1. A specified IPC protocol for broad language implementation freedom and optional isolation. Use batching and explicit shared-memory/resource-transfer contracts; do not transmit process-local pointers.
2. A versioned C-compatible function table for trusted in-process runtimes where measurements require lower overhead. Define integer widths, struct sizes, alignment, opaque handles, ownership, calling convention, error lifetimes, and thread rules. C linkage is an interoperability mechanism, not a requirement that the implementation be written in C.

Never expose Rust trait objects, `Vec`, `String`, unwinding, or compiler-specific C++ classes across that boundary. A C adapter must catch/contain implementation exceptions and panics. IPC isolation does not by itself provide a security boundary against a GPU driver.

## 5. Capability negotiation

Capabilities must describe semantics and limits, not only names such as `supports_atomics`.

| Capability family | Required constraints |
|---|---|
| Scalars | Storage and arithmetic support separately; widths; conversion behavior |
| Memory | Address spaces, allocation/binding limits, alignment, coherence, maximum offsets |
| Dispatch | Local/global dimensions, workgroup limits, shared storage, specialization limits |
| Subgroups | Permitted sizes, ability to request a fixed size, participation and operation guarantees |
| Atomics | Type, operation, address space, scope, ordering, arithmetic semantics |
| Matrices | Input/accumulator formats, shapes, scope, layouts, numerical relation, participation |
| Floating point | Rounding, denormals, contraction, approximations, signed zero/NaN behavior |
| Interoperability | Handle type, import/export ownership, synchronization, supported external APIs |

Compute compatibility from three sources: artifact requirements, compiler/backend declarations, and actual device/driver queries. The selected implementation must satisfy their intersection. Recheck load-time requirements even for cached artifacts. Selection should produce a human-readable explanation of which requirement failed.

Fallback is allowed only to an implementation that preserves the requested semantics and satisfies configured performance policy. Report fallback explicitly. A scalar matrix fallback must not silently replace an instruction-specific numerical contract.

## 6. New semantic operations

Provide a namespaced extension envelope from the start. An extension package includes operation schema, typing/effect rules, requirements, reference meaning, checker/lowering registration, and conformance fixtures. Generic passes either understand its registered semantics or conservatively preserve it.

For a verified profile, prefer extension semantics reducible to the fixed trusted core and refinement proofs checked by an existing checker. If an extension requires a new trusted checker, logic axiom, or memory-model assumption, record that as a trust-base change. It can be deployed additively but cannot inherit the old assurance claim automatically.

Keep extension operations in KIR until a worker can lower them into representable SPIR-T operations, validated extension instructions, or an explicitly modeled private form. Do not assume arbitrary new dialects fit unchanged upstream SPIR-T enums. For v1-compatible backends, required operations must fit the existing adapter/extension strategy; otherwise the contract or worker SDK needs a new version.

No unknown operation may reach final emission accidentally. No pass may erase an unknown side effect. No extension may redefine an existing operation's meaning under the same semantic digest.

## 7. Mandatory no-edit tests

**G-ADD-COMPILER:** freeze/hash the released core and adapter contracts. Build an independent compiler backend package against the public SDK. Install it; compile the existing portable corpus; compare hashes before/after. Only new package files and explicit deployment configuration may differ.

**G-ADD-RUNTIME:** repeat with a materially different runtime/API, not just a second Vulkan vendor. A candidate is a Metal path using an independently qualified translation/runtime package; an OpenCL candidate must solve SPIR-T's current `Kernel` limitation or use a separately specified lowering. P7 selects the smallest honest second implementation after a feasibility spike.

**G-ADD-LANGUAGE:** add a frontend and a host binding independently; reuse unchanged runtime and compiler packages.

**G-ADD-OP:** add a semantic extension and implementation with its evidence/checker support. Ensure an old core routes it correctly while an installation lacking that support rejects it before execution.

For each test, prohibit central enum edits, manual registration patches, root dependency edits, hidden environment rewrites, frontend conditionals, and updates to portable kernel source. Run against an installed core binary as well as a read-only source checkout. Record exact hashes and the filesystem diff.

## 8. Compatibility and withdrawal

Negotiate protocol major/minor versions and required extensions explicitly. Additive optional fields may be ignored only when they carry no required semantics; unknown required fields/versions must fail closed. Keep a corpus of old valid packages and intentionally incompatible packages.

Support disabling a broken backend package and rolling back its version independently. Cache keys include package and semantic digests. Removing a package must not break artifact inspection or falsely report that its cached binaries remain executable.
