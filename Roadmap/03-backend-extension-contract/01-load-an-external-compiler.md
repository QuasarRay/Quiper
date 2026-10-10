# Load an external compiler without rebuilding the core

**Milestone M10.** An independently packaged worker discovered through versioned data and canonical request/response vectors.

## Required inputs and specification

Start from [M06](../01-target-architecture/02-freeze-portable-artifacts.md). Use the exit-gate dependencies in [the implementation plan](../milestones.json); proof and investigation work may begin earlier. Read [the declarative specification](../Specification/README.md) before choosing representation details. This milestone implements `Extension.endpoint`, `Extension.additive`, `Extension.evidence_bound`.

## Exact guarantee

After KIR/runtime protocol v1 is frozen, a compatible backend must be installable next to a released, immutable core. It may add its own package, build files, dependencies, capability definitions, lowering implementation, runtime adapter, tests, and qualification records. It may add deployment configuration selecting that package. It must not modify existing core/compiler/frontend/kernel files, central registration source, root build files, or another backend.

Existing applications expressing supported portable semantics must not require source changes. A new driver, package installation, compatible device, or changed deployment selection is allowed. An already generated device binary is not expected to become an executable for a different ISA; portable artifacts may be compiled again through the newly installed backend.

An out-of-tree backend is the strongest test. A folder added to a repository while also modifying `enum Backend`, `Cargo.toml`, `build.rs`, a switch statement, and a CI matrix does not satisfy the requirement.
## Package roles and discovery

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

The build/test system discovers the same manifests. Backend packages build independently. Generic CI may derive requested tests from manifests; a protected admission policy, independent of contributed package metadata, chooses permitted runners and privileges. Administrators may add hardware runners and configuration without modifying dispatch logic. Do not bake vendor names into the core workflow.

## Compiler boundary

Use a versioned request/response protocol between the orchestrator and compiler workers. Requests carry KIR bytes/digests, the requested target profile, optimization policy, evidence policy, specialization values, and immutable dependency identities. Responses carry artifacts, complete requirements, diagnostics, provenance, and checked evidence results.

Each compiler worker privately owns its SPIR-T context and links a pinned `compiler/spirt` adapter SDK. Its vendor emitter can operate directly on SPIR-T within that worker. **SPIR-T objects do not cross the stable process boundary.** This permits independently built workers to use different compatible SPIR-T versions without recompiling the core.

Shared passes can be delivered as version-pinned SDK packages inside a worker. An independently installed pass that crosses process boundaries must use a specified KIR form or another explicitly versioned, validated representation. Do not invent a stable SPIR-T binary format by serializing its current Rust internals.

The protocol needs cancellation, bounded messages, streaming of large artifacts, deterministic request IDs, progress, and structured errors. A worker crash is a compilation failure; it cannot leave a successful partial package in the cache. Process isolation also matches SPIR-T's current context ownership constraints. [S2](../10-sources/README.md)

## Evidence required to close this milestone

Close **G-CONTRACT, G-ADD-COMPILER** only with the implementation artifacts, positive and negative cases, and source-to-result identities required above. Link the implementation relation to the named F* symbols and the policy's applicable O1–O10 obligations. The specification's proved lemmas are reusable model facts; they do not discharge this implementation correspondence. Record unresolved cases as blockers or explicitly outside the claim. No backend gate is marked passed by this roadmap revision.
