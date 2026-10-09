# Discovery

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

The build/test system discovers the same manifests. Backend packages build independently. Generic CI may derive requested tests from manifests; a protected admission policy, independent of contributed package metadata, chooses permitted runners and privileges. Administrators may add hardware runners and configuration without modifying dispatch logic. Do not bake vendor names into the core workflow.
