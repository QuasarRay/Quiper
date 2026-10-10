# Sequence implementation by evidence dependencies

**Milestone M02.** An ordered implementation plan with named exit gates and no inferred completion.

## Required inputs and specification

Start from [M01](01-freeze-the-goal.md). Use the exit-gate dependencies in [the implementation plan](../milestones.json); proof and investigation work may begin earlier. Read [the declarative specification](../Specification/README.md) before choosing representation details. This milestone implements `Refinement.complete`.

The result must be a reusable compiler/runtime system. Adding a supported backend after v1 freezes must add a package and deployment configuration while leaving the released core, existing frontends, kernels, bindings, root build manifests, and central CI dispatch unchanged. The first refactor can change those existing files to establish the boundary. Subsequent compatible additions cannot.

## Commit to the result before implementing it

1. Use [replacement scope](../00-current-state-and-gaps/01-freeze-the-semantic-inventory.md) and `legacy-scope.json` to retain the existing extraction surface as mandatory migration work. A useful portable release is an intermediate deliverable. G-REPLACEMENT remains blocked while mandatory behavior requires CUDA.
2. Use [evidence policies](../06-verification-and-trust/02-admit-immutable-evidence-policies.md) to decide exactly which claim is being built. A source proof, a tested backend, and a checked compilation relation are distinct fields.
3. Implement the [dependency boundaries](../01-target-architecture/01-isolate-the-contract-core.md), [package protocol](../03-backend-extension-contract/03-prove-additive-installation.md), and [KIR contract](../02-language-independent-extraction/01-implement-the-neutral-language.md) before vendor code becomes a core dependency.
4. Complete the typed extraction spike and the SPIR-T feasibility probes before freezing any v1 representation. A failed probe blocks that feature; it does not justify silently weakening its semantics.

## Build the first complete route

Build F*/Pulse → KIR → a private SPIR-T worker → Vulkan SPIR-V → the Vulkan runtime. Use the [direct-construction procedure](../04-spirt-and-gpu-lowering/01-build-and-qualify-the-worker.md), [control/pointer procedure](../04-spirt-and-gpu-lowering/03-preserve-control-memory-and-participation.md), [emission procedure](../04-spirt-and-gpu-lowering/02-run-the-integer-vertical-slice.md), and [runtime procedure](../05-runtime-and-interop/03-qualify-the-vulkan-service.md). Run unchanged portable artifacts from C and Rust bindings.

SPIR-T provides a compiler representation and transformations. Its official API does not provide Kuiper's package protocol, host-plan engine, or a ready-made Vulkan runtime. Those components are specified here as new work. See the [claim-to-source map](../10-sources/02-check-each-correctness-claim.md) for the source of each dependency fact and the proof still required for the new integration.

## Establish addition-only extension

Freeze an installed core and its full dependency/contract closure. Add an independently built compiler, an independently built runtime for a shared artifact contract, a distinct API backend, a new frontend/binding, and a semantic extension. Follow the [substitution tests](../03-backend-extension-contract/03-prove-additive-installation.md). Package admission and trusted checker selection are explicit configuration; installing an extension never grants itself trust.

Passing the tests establishes extensibility for the declared contract and tested semantic coverage. Incompatible future semantics require a new contract version. Preserve old readers and behaviors; never reinterpret an old package under a new meaning.

## Qualify the actual release

Evaluate candidate-bound evidence, real-device results, failure ownership, performance, clean installation, and rollback. Safety failures cannot be waived. Mark all implementation gates `not-run` until their recorded evidence exists. Official references justify the design constraints; they are not proofs that the proposed implementation is correct.

Start reading at [the roadmap index](README.md). All instructions use the repository's existing Kuiper name. Proposed paths and APIs are specifications to implement, unless a command or symbol is expressly identified as already present.

## Read the roadmap in this order

| Document | What it decides |
|---|---|
| [00 — Current implementation and gaps](../00-current-state-and-gaps/README.md) | Actual coupling points, proof risks, dependency revisions, and audit limits |
| [01 — Target architecture](../01-target-architecture/README.md) | Module ownership, dependency direction, artifacts, and optimization control |
| [02 — Language-independent extraction](../02-language-independent-extraction/README.md) | Kernel and host semantics, data layouts, erasure, input languages, output languages |
| [03 — Additive backend contract](../03-backend-extension-contract/README.md) | Discovery, protocols, capabilities, versioning, extension rules, and no-edit tests |
| [04 — SPIR-T and GPU lowering](../04-spirt-and-gpu-lowering/README.md) | Direct IR construction, Vulkan, matrix operations, numerical behavior, Mesa research |
| [05 — Runtime and interoperability](../05-runtime-and-interop/README.md) | Resource lifetimes, queue ordering, host ABI, failures, interop, and deployment |
| [06 — Verification and trust](../06-verification-and-trust/README.md) | Proof obligations, evidence checking, trusted components, and claim boundaries |
| [07 — Implementation phases](../07-implementation-phases/README.md) | Dependency-ordered work, deliverables, exit gates, and effort assumptions |
| [08 — Production acceptance](../08-production-acceptance/README.md) | Hardware coverage, regression suites, performance budgets, release and rollback |
| [09 — Work packages and decisions](../09-work-packages-and-decisions/README.md) | Concrete review units, risk ownership, unresolved decisions, acceptance checklist |
| [10 — Sources](../10-sources/README.md) | Pinned repository evidence and primary external references |
| [Milestones](../milestones.json) | Machine-readable phase dependencies and acceptance-gate identifiers |

## Evidence required to close this milestone

Close **P0–P8** only with the implementation artifacts, positive and negative cases, and source-to-result identities required above. Link the implementation relation to the named F* symbols and the policy's applicable O1–O10 obligations. The specification's proved lemmas are reusable model facts; they do not discharge this implementation correspondence. Record unresolved cases as blockers or explicitly outside the claim. No backend gate is marked passed by this roadmap revision.
