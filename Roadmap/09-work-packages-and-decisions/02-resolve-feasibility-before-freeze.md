# Resolve feasibility and risk before contract freeze

**Milestone M29.** Recorded decisions for target features, upstream restrictions, hardware, trust and migration blockers.

## Required inputs and specification

Start from [M03](../00-current-state-and-gaps/01-freeze-the-semantic-inventory.md). Use the exit-gate dependencies in [the implementation plan](../milestones.json); proof and investigation work may begin earlier. Read [the declarative specification](../Specification/README.md) before choosing representation details. This milestone implements `Extension.decoupled_addition`, `Lowering.sufficient`.

## Decisions made by this roadmap

| Decision | Rationale | Revisit condition |
|---|---|---|
| KIR is the stable boundary; SPIR-T is internal to workers | Avoid language/ABI lock-in and upstream data-layout coupling | A demonstrably stable upstream interchange covers the same semantics/evidence needs |
| F*/Pulse remains the first frontend | Preserve existing kernel/specification work | A separate project requests a frontend rewrite |
| Vulkan is the first new runtime | Matches current shader-oriented SPIR-T scope and provides real compute deployment | P0 exposes a specific blocking semantic/driver requirement |
| Preserve CUDA as an optional migration reference | Enables comparison and staged compatibility | Replacement coverage/evidence justifies retirement |
| Plugin discovery and versioning come before many backends | Prevents permanent centralized dispatch coupling | No exception; implementation details may evolve before freeze |
| Compiler workers own SPIR-T contexts | Avoids unstable ABI and shared-context threading assumptions | Upstream changes are evaluated behind the adapter |
| Metal is the preferred second runtime exercise | Exercises a distinct compiler/runtime boundary with reusable translation tooling | Hardware access or a concrete semantic limitation requires another real API |
| Mesa bypass is a separate experiment | Benefits and maintenance costs require measurement | Evidence establishes a justified production scope |
| Verification status is multi-dimensional | Prevents source proofs from being overstated as compiler/hardware proofs | No exception |
## P0/P1 decisions still requiring evidence

1. Validate the chosen Vulkan 1.2 / SPIR-V 1.5 baseline and its explicit feature set against the minimum device/driver matrix. Changing that choice requires an updated target-profile digest.
2. Exact typed extraction hook and changes needed in the pinned F* fork.
3. KIR initial operation grammar, canonical encoding, and the proof framework/checker for its semantics.
4. Validate the initial portable corpus. Every baseline extraction root, generated instantiation source, and reachable semantic dependency remains mandatory for full replacement; a smaller portable release cannot retire those obligations.
5. Memory model, subgroup guarantees, failure semantics, and numerical policies to freeze.
6. Host ABI/platform coverage, IPC/in-process transports, and benchmark-derived batching policy.
7. Second runtime hardware availability and the smallest workload subset that honestly tests the same portable contract.
8. Ownership of upstream patches, security/bug triage, driver qualification, and long-term supported versions.

These are explicit engineering decisions with default direction supplied by this roadmap. They should be resolved through spikes and evidence, not by silently adding assumptions to the implementation.

## Risk register

| Risk | Consequence | Mitigation / stop condition | Responsible role |
|---|---|---|---|
| SPIR-T APIs or semantics change | Broken integration or stale evidence | Pin revisions, isolate SDK, replay compatibility corpus before upgrade | Compiler owner |
| Pre-erasure information unavailable | Unrecoverable layout/type loss | Typed hook spike before committing to exporter design | Frontend owner |
| Existing proof assumptions are target-specific or incomplete | False portability/verification claims | Transitive assumption inventory, strengthened contracts, block affected verified profile | Verification owner |
| Shader memory/control restrictions reject current kernels | Scope/performance loss | Capability ledger, legalizations with evidence, explicit unsupported diagnostics | Compiler and semantics owners |
| Warp/subgroup mismatch | Hangs or incorrect synchronization | Parameterized contracts plus participation proof; reject unmet constraints | Concurrency owner |
| Floating or matrix semantics differ | Numerically wrong results despite plausible outputs | Per-operation relations, exact/approximate profiles, qualified fallback only | Numerical owner |
| Shared-memory layout changes | Broken alias/alignment proof | Layout refinement and bound checks before lowering | Memory owner |
| Runtime failure model is underspecified | Leaks, reuse of active buffers, false completion | Explicit state machine, device-loss/cancellation tests | Runtime owner |
| Backend SDK requires root build edits | No-edit requirement fails | Immutable-core/out-of-tree installation gate | Architecture owner |
| Extension package claims unsupported purity/proofs | Unsound optimization or verification | Conservative unknown effects, trusted/checkable semantics, digest-bound policy | Verification owner |
| Hardware coverage is missing | Unsupported production claim | Block corresponding qualification; acquire runner or narrow published scope explicitly | Release owner |
| Dynamic/plugin boundaries add overhead | Submission or compile latency regression | Batch/measure; optional stable in-process ABI | Runtime/performance owner |
| Private Mesa interfaces drift | Large ongoing maintenance burden | Version-pinned experimental package; promotion only with measured benefit | Mesa-track owner |
| Evidence/cache identity is incomplete | Old proofs attached to new code | Complete keys, tamper tests, independent replay | Tooling owner |
| Feature parity is declared prematurely | Users lose CUDA-dependent behavior | Exhaustive disposition ledger and separate portable/full-parity release labels | Release owner |

One person can hold several roles. Each unresolved risk needs one accountable owner and an explicit release consequence.

## Evidence required to close this milestone

Close **G-BASELINE, G-SPIRT-FEASIBILITY, G-CONTRACT** only with the implementation artifacts, positive and negative cases, and source-to-result identities required above. Link the implementation relation to the named F* symbols and the policy's applicable O1–O10 obligations. The specification's proved lemmas are reusable model facts; they do not discharge this implementation correspondence. Record unresolved cases as blockers or explicitly outside the claim. No backend gate is marked passed by this roadmap revision.
