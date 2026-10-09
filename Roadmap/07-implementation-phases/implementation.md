# Turn the phases into reviewable implementation changes

## 1. Use contract-first review units

Each work package produces: the contract/accepted subset; implementation; focused positive/negative tests; evidence and invalidation impact; and installation/rollback behavior. A change cannot claim completion solely because its types compile. All commands for new KIR/backend/release tools are proposed until those tools are implemented.

| Phase | Concrete implementation sequence | Stop condition |
|---|---|---|
| P0 | Freeze legacy ledger and source assumptions; reproduce existing pipeline; trace pre-erasure capture; compile direct-construction probe; triage annotations/loop/QPtr; select second API and hardware; ratify performance manifest | Missing facts, unsupported required forms, inaccessible hardware or undefined release scope |
| P1 | Specify canonical artifacts and semantics; implement readers/checkers; discovery and role protocols; host/caller obligations; failure/footprint/lifetime models; result evaluator contract | A source/vendor dependency leaks into the public contract or a failure has no resource outcome |
| P2 | Implement typed exporter and body correspondence; port primitive families; add C frontend and C/Rust bindings; execute host plans | Any required data is inferred from generated text or unverified callers receive unjustified postconditions |
| P3 | Implement direct SPIR-T construction and qualified passes; Vulkan buffers/pipelines/dispatch; reflection and environment checks; CUDA-free vertical corpus | Untriaged control/annotation/pointer path, unvalidated output or missing runtime cleanup |
| P4 | Complete layout/atom footprints, barriers/subgroups/atomics and async chains; dedup/query/unknown outcomes; package lifetimes; second Vulkan implementation | Forbidden memory outcome, false completion, unsafe replay or unload |
| P5 | Qualify numerical modes and workload families; matrix/vendor extension lowerings; publish complete disposition ledger | Missing mandatory feature blocks replacement even if portable qualification can proceed |
| P6 | Complete required O1–O10 relations for the selected policy; replay strict proof closure; test evidence tampering and fallback | A required obligation remains replaced by an unauthorized trust statement |
| P7 | Independently add compiler, compatible runtime, distinct-API backend, frontend/binding and operation against frozen installation | Any existing protected source/build/registry/binary changes, or only a renamed implementation is tested |
| P8 | Evaluate exact-candidate gate records; real hardware/performance/soak/failure tests; clean/offline install, canary, drain and rollback | Missing/stale/skipped mandatory result, safety waiver, or incomplete replacement claim |

P0's probes are early feasibility work. Passing a narrow probe does not complete G-EXTRACT or G-VERTICAL. P4/P5 work can start earlier; every P3 workload must already have the synchronization and numerical semantics it uses. Proof work starts in P1 and follows each transformation.

## 2. Correct the release dependency model

G-SPIRT-FEASIBILITY blocks enabling the affected compilation paths and is a P0 gate. Its scope covers explicit annotation restrictions, triaged loop shapes, QPtr eligibility and the dependency strategy. Required excluded features remain in the legacy ledger.

G-COVERAGE classifies the complete ledger and qualifies the claimed profile. G-REPLACEMENT requires all mandatory baseline behavior to have qualified CUDA-independent coverage; classification as deferred is not a pass. G-CUDA-FREE proves a path has no CUDA dependency, not that the path covers all legacy behavior.

G-TRUST evaluates the fixed assurance policy. Lower assurance may be accurately published as a separate profile; it cannot satisfy a requested refinement-verified claim. G-HOST-CODEGEN is conditional on enabling native plan compilation. Bindings around one interpreter are the mandatory first-release path.

P8 reruns affected extension gates after late core/contract changes. If an interface change is needed during P7, keep v1 provisional, refreeze, and repeat. After release, incompatible semantics use a new major contract with side-by-side readers/packages.

## 3. Report progress without conflating evidence

Use the [finding corrections](../Flaws/README/resolutions.md), [gate records](../08-production-acceptance/gate-records.md), and [milestones](../milestones.json). Report exact entrypoints/devices/profiles, proof obligations discharged, known restrictions and next blocker. The original 56–122 person-week estimate remains an unvalidated planning range; P0/P3 must re-estimate it after the added proof, conformance and dependency work. It is not a delivery commitment.
