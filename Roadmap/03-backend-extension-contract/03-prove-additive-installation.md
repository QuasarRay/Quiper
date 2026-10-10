# Prove additive installation on an immutable candidate

**Milestone M12.** Before/after protected-file hashes and independent compiler, runtime, distinct-API, frontend, binding and semantic-extension exercises.

## Required inputs and specification

Start from [M10](01-load-an-external-compiler.md), [M11](02-match-artifacts-to-independent-runtimes.md). Use the exit-gate dependencies in [the implementation plan](../milestones.json); proof and investigation work may begin earlier. Read [the declarative specification](../Specification/README.md) before choosing representation details. This milestone implements `Extension.decoupled_addition`, `Extension.admitted_policy`.

## Package layout and installation

Define a package root with a canonical manifest, executable/service descriptors, private dependencies, semantic definitions, optional checker bundles, fixtures, notices, and qualification records. Package content is immutable after installation. An upgrade installs a new identity alongside the old one; it does not overwrite files backing active objects.

Manifest fields must include:

| Field | Meaning and validation |
|---|---|
| `package_id`, `version`, `content_digest` | Stable identity; reject duplicate identity with different content |
| `roles` | Each service role, endpoint, protocol version, and schema digest |
| `accepts`, `produces` | KIR profiles/artifact formats/ABIs and required semantics |
| `requirements` | Declarative feature/limit/numeric constraints, including tuple dependencies |
| `operations`, `passes`, `checkers` | Namespaced registrations with signatures, effects, semantic digests and protocol endpoints |
| `dependencies` | Exact package/private-tool identities and platform constraints |
| `budgets` | Message, memory, time, specialization and output limits |
| `qualification` | Evidence references; never authority to grant itself trust |

Discover only in explicitly configured install roots. Canonicalize paths, reject path traversal and duplicate registrations, verify content digests, then consult protected admission policy before loading executable code. A source package cannot cause network plugin installation. Resolve deterministic priorities from deployment configuration; enumeration order is never selection policy.

All backends build separately. A new backend adds its own manifest and code, then its install root to allowed configuration. The released core, existing Cargo workspace/lockfile, C header definitions, existing package sources and CI vendor dispatch remain unchanged.

## Negotiate before sending work

Use a transport-neutral envelope with protocol ID/major/minor, required extensions, session/generation, request ID, payload length/digest, and a typed payload. Negotiate maximum lengths and supported methods. Reject unknown major versions, required fields, and unsupported semantics. Optional diagnostic fields may be skipped only when their omission cannot affect meaning.

Compiler requests carry canonical KIR identity/bytes, target-profile constraints, numeric/evidence policy, specialization, dependency identities, pass plan, and budgets. Responses carry the artifact and reflection identities, final requirements, evidence results, diagnostics, and complete provenance. Cancellation stops future work or returns a declared result; a killed compiler never publishes a partial successful cache entry.

Runtime requests use the [submission protocol](../05-runtime-and-interop/01-enforce-submission-and-failure-semantics.md), not compiler retry semantics. Every handle belongs to a specific admitted service instance and generation. Service discovery cannot reroute existing resources to a replacement endpoint.

## Keep the C transport extensible

Define fixed-width status values and a versioned function-table prefix containing ABI major/minor and byte length. Clients read only fields within the negotiated length; providers never write beyond supplied buffers. Fixed-width descriptors have specified size, alignment, field offsets, reserved-zero fields, and ownership. Test different compilers/architectures in the claimed ABI set.

Keep vendor data in length-delimited typed extension payloads, not a growing public union of vendor structs. Resolve optional services by namespaced ID and schema version. Old clients reject unknown required services; optional ones do not change existing behavior. The transport can use a small generic request/reply entrypoint plus negotiated typed fast paths, provided both implement the same protocol semantics.

Borrowed input spans live for the call unless an explicit asynchronous ownership-transfer operation accepts them. Output allocation/free routines come from the same provider or a declared caller allocator. No Rust object layout or unwinding crosses the table. An in-process abort is application-fatal; IPC is required for a policy promising application survival from plugin aborts.

## Capability constraints and semantic extensions

Implement a bounded declarative constraint language: conjunction/disjunction, exact IDs/versions, integer bounds, and membership in typed tuples. A matrix shape is tied to its input/accumulator types, scope and numeric relation. Atomic type/op/address-space/scope/order is a tuple. Do not independently intersect flags and invent unsupported combinations.

Evaluate artifact requirements against compiler declarations and actual runtime device queries. Recheck enabled features at load/prepare time. The runtime reports actual alignment/coherence granularity and limits. A device change creates a new checked plan.

An operation extension supplies operand/result schema, effects, reference meaning, requirement predicates, checker/lowering endpoints, and fixtures. Unknown effects are conservatively preserved; no unknown op reaches executable emission. A trusted policy admits checker identities independently of package metadata. Additions that need stronger logic assumptions create a new evidence-policy identity.

## Conformance errors

Use stable errors for incompatible protocol, unknown semantics, unsatisfied capability, invalid evidence, malformed package, exhausted budget, compiler failure, rejected submission, unknown submission outcome, session failure, and device loss. Preserve source/operation IDs across boundaries. Text messages aid humans; code must not parse them to choose behavior.

## Mandatory no-edit tests

**G-ADD-COMPILER:** freeze/hash the released core and adapter contracts. Build an independent compiler backend package against the public SDK. Install it; compile the existing portable corpus; compare hashes before/after. Only new package files and explicit deployment configuration may differ.

**G-ADD-RUNTIME:** repeat with a materially different runtime/API, not just a second Vulkan vendor. A candidate is a Metal path using an independently qualified translation/runtime package; an OpenCL candidate must solve SPIR-T's current `Kernel` limitation or use a separately specified lowering. P0 selects and records the second implementation after a feasibility spike; P7 implements and qualifies that selected route.

Also require the [compiler-only and compatible-runtime substitutions](03-prove-additive-installation.md). The distinct-API exercise is cumulative with those tests; a paired backend with an undocumented private artifact convention cannot satisfy independence.

**G-ADD-LANGUAGE:** add a frontend and a host binding independently; reuse unchanged runtime and compiler packages.

**G-ADD-OP:** add a semantic extension and implementation with its evidence/checker support. Ensure an old core routes it correctly while an installation lacking that support rejects it before execution.

For each test, prohibit central enum edits, manual registration patches, root dependency edits, hidden environment rewrites, frontend conditionals, and updates to portable kernel source. Run against an installed core binary as well as a read-only source checkout. Record exact hashes and the filesystem diff.
## Compatibility and withdrawal

Negotiate protocol major/minor versions and required extensions explicitly. Additive optional fields may be ignored only when they carry no required semantics; unknown required fields/versions must fail closed. Keep a corpus of old valid packages and intentionally incompatible packages.

Support disabling a broken backend package and rolling back its version independently. Cache keys include package and semantic digests. Removing a package must not break artifact inspection or falsely report that its cached binaries remain executable.

## Freeze the protected closure

Before P7, produce a manifest of the installed core binaries, contract readers, built-in validators, existing frontends/bindings, root build files/lockfiles, central workflow logic, and existing backend packages. Hash their content and source revisions. Make the checkout read-only and run the same exercise without a source checkout at all.

The permitted diff consists of a new package/install root and explicit deployment configuration selecting/admitting it. Cache and evidence outputs go to separate writable directories and do not replace installed components. Do not use a rewritten environment, a hidden regenerated registry, or a new core binary to pass the test.

## Test each extension axis separately

| Exercise | New component | Components kept unchanged | Required outcome |
|---|---|---|---|
| G-ADD-COMPILER | Independent compiler worker producing the existing Vulkan artifact contract | Core, frontend, runtime, host bindings, portable corpus | Correct compilation/execution with checked requirements and evidence |
| G-ADD-RUNTIME substitution | Independent runtime consuming that same artifact contract | Core, compiler output bytes, frontend, host bindings | Same semantics and failure ownership on compatible devices |
| G-ADD-RUNTIME distinct API | Selected Metal or other real API package | Core, portable KIR/host contract, existing source and bindings | Same mandatory portable corpus through the second API |
| G-ADD-LANGUAGE | Third frontend and independently implemented host binding | Existing core/compiler/runtime packages | Correct KIR and shared artifact consumption with accurate assurance labels |
| G-ADD-OP | Semantic extension plus lowerer/checker support | Core, existing operation meanings and existing packages | Accepted with admitted support; rejected before execution without it |

The two runtime exercises are cumulative. Another Vulkan implementation tests substitution but does not replace the distinct-API requirement. A Metal runtime is not expected to load Vulkan binaries; format compatibility is checked where claimed.

## Exercise more than the happy path

Use elementwise operations, views, bounded control flow, reductions/matrix kernels, and asynchronous chains in the declared common subset. Include malformed/unknown versions, insufficient capabilities, incompatible evidence policies, missing extensions, old package readers, duplicate package IDs, and conflicting registrations.

Install packages built by an independent implementer from the public specification. A renamed existing package or a paired compiler/runtime sharing undocumented assumptions is insufficient. Run compiler-only and runtime-only substitutions to expose that coupling.

## Preserve feature expressiveness

Keep hardware-specific operations as namespaced extensions with explicit requirements. Add a test operation with side effects that the generic optimizer does not understand; it must preserve effects conservatively and route to the registered lowerer. If an old core cannot express required routing/effect/evidence behavior, fix the provisional contract and restart the freeze. After v1 release, use an explicitly incompatible contract version instead of changing old semantics.

## Record final-candidate evidence

Store before/after protected hashes, filesystem diffs, package/tool identities, corpus digest, target/profile, environment, full pass plan, and outcomes using the [gate-result contract](../08-production-acceptance/02-implement-candidate-bound-admission.md). Any covered core/contract change invalidates the relevant exercise. P8 must use results for the actual release candidate, including changes made after an earlier P7 pass.

## Evidence required to close this milestone

Close **G-ADD-COMPILER, G-ADD-RUNTIME, G-ADD-LANGUAGE, G-ADD-OP** only with the implementation artifacts, positive and negative cases, and source-to-result identities required above. Link the implementation relation to the named F* symbols and the policy's applicable O1–O10 obligations. The specification's proved lemmas are reusable model facts; they do not discharge this implementation correspondence. Record unresolved cases as blockers or explicitly outside the claim. No backend gate is marked passed by this roadmap revision.
