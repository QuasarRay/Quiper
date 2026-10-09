# Capabilities

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
