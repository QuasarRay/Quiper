# Implement the dependency boundaries

## 1. Create the shared components once

| Proposed component | Public input/output | Internal responsibility | Forbidden dependency |
|---|---|---|---|
| `contracts` | Versioned byte schemas and semantic identifiers | Canonical encoding, size limits, compatibility rules | GPU SDKs, F* AST, SPIR-T object layouts |
| `semantics` | Operation definitions and proof/checker contracts | KIR/host state meaning and refinement relations | Vendor dispatch or device discovery |
| `core` | Packages, policy, requests, responses | Admission, planning, cache identity, generic role routing | Named backend implementations or their build dependencies |
| `frontend-fstar` | Pinned checked source → KIR/evidence | Typed capture, specialization, erasure correspondence | Vulkan objects or CUDA source text in public artifacts |
| `compiler-spirt-sdk` | Private worker API | Construct SPIR-T, manage approved passes, emit target formats | Stable process ABI commitments about Rust internals |
| Compiler package | Compiler protocol → target package | Backend-specific lowering/toolchain and provenance | Patching the installed core or another backend |
| Runtime package | Runtime protocol → execution results | Device resources, queues, visibility, driver errors | Source language or compiler heap objects |
| Binding package | Language API → common host/runtime contract | Ownership wrappers and checked marshaling | Backend-specific kernel recompilation |
| Checker package | Evidence request → checked result | One recognized relation/proof format | Self-authorizing trust or changing old semantic IDs |

Implement `core` against protocol clients and opaque artifact identities. Keep concrete backend crates out of its Cargo dependency graph. Backend packages have their own build roots, lockfiles, native dependencies, and release artifacts. Installing a backend must not amend the root workspace, feature list, link flags, or workflow vendor matrix.

SPIR-T workers may privately use different pinned SPIR-T revisions. Their public input remains KIR and their public output remains a versioned artifact package. Shared SDK reuse is an implementation choice within the package. The official [SPIR-T Module API](https://rust-gpu.github.io/spirt/spirt/struct.Module.html) exposes context-owned entities and a non-thread-safe context; it does not define this plugin ABI. Isolate that implementation behind the worker.

## 2. Define composition using data

Resolve a route as `(frontend contract, KIR profile, compiler service, artifact format/ABI, runtime service, host binding, evidence policy)`. Each edge has a schema/version and semantic requirements. Match declared constraints with checked actual artifact requirements and device capabilities. Reject missing or conflicting edges before execution.

Use namespaced identifiers and content digests for roles, formats, operations, and checkers. The core knows the role protocols and extension envelope, not an enum of GPU vendors. A compiler may produce several formats, and a runtime may consume several compatible formats. A package may implement multiple roles, but its roles remain separately addressable and substitutable.

No routing rule may inspect source-language syntax, CUDA symbol names, or backend package names to determine semantics. Unknown required fields/operations fail closed. The core may copy opaque vendor payloads after validating their declared envelope; only the designated worker/runtime interprets their private meaning.

## 3. Preserve information across representations

Keep kernel semantics, host plans, layout, requirements, and evidence in the portable package. Keep SPIR-T entity handles, analysis caches, native pointers, and driver handles process-local. Target packages contain device bytes plus reflection, requirements, specialization and evidence identities.

A pass result records its input/output digests and established/preserved/invalidated facts. Rewrites must map one-to-many or many-to-one operations explicitly where evidence depends on that correspondence. Debug-only source spans may be dropped; semantic preconditions, effects, and numeric policies cannot disappear with them.

## 4. Implement the planner

Separate mandatory legalization from optional optimization. Select only passes whose input dialect, side-effect model, required analyses, capability constraints, and evidence policy are satisfied. Build a deterministic DAG of scheduled pass instances. A fixed-point pass is a bounded composite with a termination budget and declared evidence relation; do not encode an unbounded cycle in the planner.

Store the selected plan before compilation and include it in cache identity. On pass failure, discard partial success outputs. A fallback may change implementation but must satisfy the same semantics and evidence policy. Requests to weaken those policies are new explicit requests, not retries of the original one.

## 5. Verify decoupling continuously

Inspect dependency graphs and dynamic loads. Run an installed core with a read-only source checkout and no vendor SDK present. Discovery, inspection, and unsupported-target diagnostics must work. Add a backend package externally and prove only the new package/configuration/cache changed. Use the [complete substitution matrix](../03-backend-extension-contract/addition-only-tests.md) before v1 freeze and for the final release candidate.
