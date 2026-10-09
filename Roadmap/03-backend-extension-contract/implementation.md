# Implement discoverable packages and service protocols

## 1. Package layout and installation

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

## 2. Negotiate before sending work

Use a transport-neutral envelope with protocol ID/major/minor, required extensions, session/generation, request ID, payload length/digest, and a typed payload. Negotiate maximum lengths and supported methods. Reject unknown major versions, required fields, and unsupported semantics. Optional diagnostic fields may be skipped only when their omission cannot affect meaning.

Compiler requests carry canonical KIR identity/bytes, target-profile constraints, numeric/evidence policy, specialization, dependency identities, pass plan, and budgets. Responses carry the artifact and reflection identities, final requirements, evidence results, diagnostics, and complete provenance. Cancellation stops future work or returns a declared result; a killed compiler never publishes a partial successful cache entry.

Runtime requests use the [submission protocol](../05-runtime-and-interop/submission-and-lifecycle.md), not compiler retry semantics. Every handle belongs to a specific admitted service instance and generation. Service discovery cannot reroute existing resources to a replacement endpoint.

## 3. Keep the C transport extensible

Define fixed-width status values and a versioned function-table prefix containing ABI major/minor and byte length. Clients read only fields within the negotiated length; providers never write beyond supplied buffers. Fixed-width descriptors have specified size, alignment, field offsets, reserved-zero fields, and ownership. Test different compilers/architectures in the claimed ABI set.

Keep vendor data in length-delimited typed extension payloads, not a growing public union of vendor structs. Resolve optional services by namespaced ID and schema version. Old clients reject unknown required services; optional ones do not change existing behavior. The transport can use a small generic request/reply entrypoint plus negotiated typed fast paths, provided both implement the same protocol semantics.

Borrowed input spans live for the call unless an explicit asynchronous ownership-transfer operation accepts them. Output allocation/free routines come from the same provider or a declared caller allocator. No Rust object layout or unwinding crosses the table. An in-process abort is application-fatal; IPC is required for a policy promising application survival from plugin aborts.

## 4. Capability constraints and semantic extensions

Implement a bounded declarative constraint language: conjunction/disjunction, exact IDs/versions, integer bounds, and membership in typed tuples. A matrix shape is tied to its input/accumulator types, scope and numeric relation. Atomic type/op/address-space/scope/order is a tuple. Do not independently intersect flags and invent unsupported combinations.

Evaluate artifact requirements against compiler declarations and actual runtime device queries. Recheck enabled features at load/prepare time. The runtime reports actual alignment/coherence granularity and limits. A device change creates a new checked plan.

An operation extension supplies operand/result schema, effects, reference meaning, requirement predicates, checker/lowering endpoints, and fixtures. Unknown effects are conservatively preserved; no unknown op reaches executable emission. A trusted policy admits checker identities independently of package metadata. Additions that need stronger logic assumptions create a new evidence-policy identity.

## 5. Conformance errors

Use stable errors for incompatible protocol, unknown semantics, unsatisfied capability, invalid evidence, malformed package, exhausted budget, compiler failure, rejected submission, unknown submission outcome, session failure, and device loss. Preserve source/operation IDs across boundaries. Text messages aid humans; code must not parse them to choose behavior.
