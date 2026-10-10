# Close independent-extension and release gates

**Milestone M24.** Independent additions rerun on the final candidate, complete replacement coverage and a defensible release decision.

## Required inputs and specification

Start from [M12](../03-backend-extension-contract/03-prove-additive-installation.md), [M23](02-close-execution-and-proof-gates.md), [M26](../08-production-acceptance/02-implement-candidate-bound-admission.md), [M27](../08-production-acceptance/03-qualify-performance-and-cutover.md). Use the exit-gate dependencies in [the implementation plan](../milestones.json); proof and investigation work may begin earlier. Read [the declarative specification](../Specification/README.md) before choosing representation details. This milestone implements `Extension.decoupled_addition`, `Qualification.full_replacement`.

## P7. Prove the architecture is additive and language-independent

**Depends on:** P4 and the P2 language/host boundary; can overlap P5/P6.

1. Freeze/hash a release-candidate core binary and read-only source checkout.
2. Build an out-of-tree second compiler/runtime package. Prefer a narrowly scoped Metal backend using qualified SPIR-V → MSL translation and a native runtime adapter; select it during P0 feasibility work.
3. Run the same portable corpus and host bindings through that package. Add a separate frontend/host adapter and a checked semantic extension.
4. Verify no core/frontends/existing kernels/registries/root build manifests changed. Verify generic build/test discovery works.
5. If the exercise requires an interface change, update the provisional contract, freeze again, and repeat with an independent package. Do not waive the requirement because the change is small.

**G-ADD-COMPILER, G-ADD-RUNTIME, G-ADD-LANGUAGE, G-ADD-OP:** all pass with candidate-bound hashes, dependency closures, and diffs. Include compiler-only and runtime-only substitutions for a shared target-artifact contract in addition to the distinct-API package. A simulator or another Vulkan device is useful evidence but does not replace the distinct runtime/API gate.
## P8. Qualify, package, cut over, and support

**Depends on:** P5, P6, P7.

1. Complete the production matrix, soak tests, performance budgets, error-path exercises, clean installation, and compatibility corpus.
2. Ship separate portable/compiler/runtime/binding packages with reproducible provenance and dependency/license inventories.
3. Publish exact supported devices/drivers/profiles and assurance levels. Freeze v1 and document upgrade/rollback policy.
4. Run canaries with real workloads. Make the new path the default only after the gates pass.
5. Keep CUDA as an optional legacy/reference package; remove it from default discovery/build/install requirements. Retire compatibility code only under a separate, evidence-backed deprecation decision.

**G-PRODUCTION, G-CUDA-FREE:** release matrix and rollback rehearsal pass for the exact candidate; unsupported scope is explicit; no default path accidentally resolves CUDA components. **G-REPLACEMENT** additionally requires CUDA-independent coverage of every mandatory legacy-scope row. A scoped portable release cannot satisfy that gate by deferring rows.

## Use contract-first review units

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

## Correct the release dependency model

G-SPIRT-FEASIBILITY blocks enabling the affected compilation paths and is a P0 gate. Its scope covers explicit annotation restrictions, triaged loop shapes, QPtr eligibility and the dependency strategy. Required excluded features remain in the legacy ledger.

G-COVERAGE classifies the complete ledger and qualifies the claimed profile. G-REPLACEMENT requires all mandatory baseline behavior to have qualified CUDA-independent coverage; classification as deferred is not a pass. G-CUDA-FREE proves a path has no CUDA dependency, not that the path covers all legacy behavior.

G-TRUST evaluates the fixed assurance policy. Lower assurance may be accurately published as a separate profile; it cannot satisfy a requested refinement-verified claim. G-HOST-CODEGEN is conditional on enabling native plan compilation. Bindings around one interpreter are the mandatory first-release path.

P8 reruns affected extension gates after late core/contract changes. If an interface change is needed during P7, keep v1 provisional, refreeze, and repeat. After release, incompatible semantics use a new major contract with side-by-side readers/packages.

## Report progress without conflating evidence

Use the [finding corrections](../Flaws/README/resolutions.md), [gate records](../08-production-acceptance/02-implement-candidate-bound-admission.md), and [milestones](../milestones.json). Report exact entrypoints/devices/profiles, proof obligations discharged, known restrictions and next blocker. The original 56–122 person-week estimate remains an unvalidated planning range; P0/P3 must re-estimate it after the added proof, conformance and dependency work. It is not a delivery commitment.

## PX. Direct Mesa research

**Depends on:** the stable P4 baseline; never blocks P8.

Deliver one-driver prototype, semantics comparison, generated-code/performance evidence, maintenance estimate, and a promote/defer decision. Native ISA generation and direct backend-IR routes receive separate work packages rather than being hidden inside the portable migration.

**G-MESA-DECISION:** measured evidence supports the decision. A negative result is a valid completed experiment.
## Effort and sequencing assumptions

These are planning ranges, not measured implementation estimates or delivery promises. They assume engineers already familiar with compiler construction, GPU memory models, F*/Pulse, and driver APIs, plus access to required hardware.

| Phase | Initial engineering effort range |
|---|---:|
| P0 | 2–4 person-weeks |
| P1 | 4–8 person-weeks |
| P2 | 4–8 person-weeks |
| P3 | 6–12 person-weeks |
| P4 | 8–16 person-weeks |
| P5 | 8–20 person-weeks |
| P6 | 10–24 person-weeks |
| P7 | 6–14 person-weeks |
| P8 | 8–16 person-weeks |

The sum is 56–122 person-weeks before the optional Mesa track. Concurrent work can shorten calendar time but not remove proof and hardware dependencies. Full legacy parity, new proof foundations, upstream compiler changes, or unavailable driver features can exceed this range substantially. Re-estimate after P0 and after the P3 end-to-end path; proof research should not be treated as fixed-duration routine implementation.

Keep review units small and evidence-complete. Parallel teams can own semantics, extraction/compiler, runtime/bindings, and hardware qualification, with one owner for each interface. Do not merge cross-cutting implementation before the owning contracts are reviewable.

## Evidence required to close this milestone

Close **P7–P8, optional PX** only with the implementation artifacts, positive and negative cases, and source-to-result identities required above. Link the implementation relation to the named F* symbols and the policy's applicable O1–O10 obligations. The specification's proved lemmas are reusable model facts; they do not discharge this implementation correspondence. Record unresolved cases as blockers or explicitly outside the claim. No backend gate is marked passed by this roadmap revision.
