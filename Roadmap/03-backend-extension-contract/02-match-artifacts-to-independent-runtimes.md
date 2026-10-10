# Match artifacts and capability tuples to independent runtimes

**Milestone M11.** Compatible compiler/runtime substitution with a bounded ABI/IPC protocol and pre-dispatch capability rejection.

## Required inputs and specification

Start from [M06](../01-target-architecture/02-freeze-portable-artifacts.md). Use the exit-gate dependencies in [the implementation plan](../milestones.json); proof and investigation work may begin earlier. Read [the declarative specification](../Specification/README.md) before choosing representation details. This milestone implements `Extension.capability`, `Extension.supports`, `Runtime.fresh`.

## Runtime boundary

Specify a transport-independent runtime service contract covering device enumeration, feature queries, allocation, mapping, transfers, module loading, pipeline preparation, dispatch, dependencies, event completion, resource release, and device-loss handling.

Provide two transports:

1. A specified IPC protocol for broad language implementation freedom and optional isolation. Use batching and explicit shared-memory/resource-transfer contracts; do not transmit process-local pointers.
2. A versioned C-compatible function table for trusted in-process runtimes where measurements require lower overhead. Define integer widths, struct sizes, alignment, opaque handles, ownership, calling convention, error lifetimes, and thread rules. C linkage is an interoperability mechanism, not a requirement that the implementation be written in C.

Never expose Rust trait objects, `Vec`, `String`, unwinding, or compiler-specific C++ classes across that boundary. A C adapter must prevent unwinding across its non-unwinding ABI. Contain recoverable exceptions within the implementing language; an aborting panic is process-fatal. Advertise that failure boundary explicitly and use IPC when application survival is required. IPC isolation does not by itself provide a security boundary against a GPU driver.

## Capability negotiation

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
## New semantic operations

Provide a namespaced extension envelope from the start. An extension package includes operation schema, typing/effect rules, requirements, reference meaning, checker/lowering registration, and conformance fixtures. Generic passes either understand its registered semantics or conservatively preserve it.

For a verified profile, prefer extension semantics reducible to the fixed trusted core and refinement proofs checked by an existing checker. If an extension requires a new trusted checker, logic axiom, or memory-model assumption, record that as a trust-base change. It can be deployed additively but cannot inherit the old assurance claim automatically.

Keep extension operations in KIR until a worker can lower them into representable SPIR-T operations, validated extension instructions, or an explicitly modeled private form. Do not assume arbitrary new dialects fit unchanged upstream SPIR-T enums. For v1-compatible backends, required operations must fit the existing adapter/extension strategy; otherwise the contract or worker SDK needs a new version.

No unknown operation may reach final emission accidentally. No pass may erase an unknown side effect. No extension may redefine an existing operation's meaning under the same semantic digest.

## Evidence required to close this milestone

Close **G-CONTRACT, G-ADD-RUNTIME** only with the implementation artifacts, positive and negative cases, and source-to-result identities required above. Link the implementation relation to the named F* symbols and the policy's applicable O1–O10 obligations. The specification's proved lemmas are reusable model facts; they do not discharge this implementation correspondence. Record unresolved cases as blockers or explicitly outside the claim. No backend gate is marked passed by this roadmap revision.
