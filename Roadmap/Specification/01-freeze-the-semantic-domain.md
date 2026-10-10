# S1. Freeze the semantic domain and profile

**Deliverable:** a reviewed profile whose meanings are independent of the implementation, with every required source operation mapped to a semantic definition. Complete this before freezing KIR or adding a vendor backend.

## Define the objects that implementations must preserve

Use `Foundation.resource` as allocation identity plus generation. A reused storage address is a different logical resource when its generation changes. `location.cell` and `view.start/count` are logical cells; `cell_bytes` fixes their byte interpretation. Arithmetic uses mathematical integers/naturals until the target relation explicitly checks and narrows it. A host pointer, Vulkan object or SPIR-T entity ID cannot be a portable resource identity.

A scalar's type is checked by `well_typed`. U32 arithmetic is modulo 2^32; signed I32 values occupy the stated interval; F32 values are IEEE bit patterns, not host-language floating values. Boolean shader values and Boolean storage encodings remain separate. `contains`, `subview`, `disjoint` and `compatible` specify logical access. Atomic/atomic permission compatibility still requires the selected atomic memory relation; it does not permit arbitrary conflicting atomics or mixed atomic/non-atomic access.

Define a canonical external encoding after these meanings are reviewed. Provide at least two independent encoders/decoders and golden malformed/canonical vectors. Prove decoded logical quantities agree, reject out-of-range values before narrowing, and bind every layout to its exact schema. Mathematical `content` equality is equality of bytes. A digest implementation refines it under the declared collision-resistance assumption; a digest string is not an unconditional proof of equality.

## Close the extensible semantic parameters

| Parameter | Required definition before qualification | Forbidden shortcut |
|---|---|---|
| `Kernel.meaning` | Concrete preconditions, state/result/event relation and frame behavior for every invoked semantic identity | Unknown operation treated as pure or successful |
| `Kernel.authority` | Invocation permissions derived from allocation/view ownership and globally compatible reservations | Arbitrary `True` authorization |
| `Memory.admissible profile` | Selected source concurrent memory semantics, including reads-from, coherence, scopes, visibility and progress assumptions | Reusing only the race-free predicate as a complete memory model |
| `Memory.float_policy` and `Operations.matrix_policy` | Exact selected rounding, contraction, exceptional values, subnormals, tolerances and shape/format semantics | A vendor feature bit or a host `float` operation substituted for the relation |
| `Host.host_step quiescent` | An observation relation justified by completed work or safe platform teardown for that resource generation | Assuming worker death implies DMA completion |
| `Refinement.contract` | Independently defined input precondition and all allowed successful/exceptional observations | Choosing the implementation's own behavior as its specification |
| `Extension.policy_registry` and trusted checker relation | Protected immutable policy definitions, authorized checker identities and soundness evidence | A producer admitting its own evidence |

Parameterization is how the specification stays implementation agnostic and extensible. It also leaves a concrete obligation: qualify an actual definition for each claimed profile. Keep unresolved parameters visible in the evidence ledger. The generic lemmas do not supply these definitions.

## Freeze coverage from the source closure

Generate the legacy ledger from the pinned extractor, all reachable default extraction and foreign calls, headers, generated instantiations and public kernel entrypoints. For each row record signature, effects, caller assumptions, source/target numerical meaning, synchronization, host/device placement, semantic identity, definition digest and implementation owner. Include unsupported rows; deleting a difficult primitive does not complete replacement.

`Operations.family` supplies the classification; `catalog_covers` quantifies over the entire frozen required set. Instantiate that set from the ledger, never from the subset the backend already supports. New operation definitions are additions under new identities. A changed meaning gets a new definition/version and fresh evidence. A stronger platform assumption cannot silently enter an existing identity.

## Acceptance evidence

Close the semantic-domain part of G-CONTRACT with the ledger, chosen profile relations, canonical vectors, checked arithmetic boundaries and permission tests. Reuse `subview_preserves_bounds`, `disjoint_excludes_shared_cell`, `write_frame` and `add_u32_range`. Prove the concrete encoding and allocator mappings separately. Full G-REPLACEMENT remains blocked until every mandatory ledger row has a qualified CUDA-independent implementation.
