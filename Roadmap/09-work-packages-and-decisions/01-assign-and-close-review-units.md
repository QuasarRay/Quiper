# Assign and close implementation review units

**Milestone M28.** Review units with exact inputs, proof symbols, implementation owner, evidence and rejection cases.

## Required inputs and specification

Start from [M02](../README/02-sequence-the-implementation.md), [M04](../00-current-state-and-gaps/02-classify-the-proof-boundary.md). Use the exit-gate dependencies in [the implementation plan](../milestones.json); proof and investigation work may begin earlier. Read [the declarative specification](../Specification/README.md) before choosing representation details. This milestone implements `Refinement.obligations`, `Qualification.qualified`.

## Suggested review units

These are implementation work packages, not automatically created GitHub issues or commitments by named people. Each change should include its contract, implementation, focused tests, evidence impact, and rollout effect. Use stacked changes when dependencies are real; avoid one branch that rewrites the entire system before it can be exercised.

| ID | Work package | Existing touchpoints | Acceptance evidence |
|---|---|---|---|
| W01 | Primitive/host/evidence inventory | `extraction/`, `src/`, `include/`, generated instantiations | Every extraction/default/foreign case classified |
| W02 | Reproducible baseline | `verify.mk`, `nvcc.mk`, tests/benchmarks/packaging | Pinned logs and workload results |
| W03 | KIR grammar, canonical encoding, semantic profiles | New `contracts/`, semantic specifications | Independent reader agreement; invalid inputs rejected |
| W04 | Discovery and compiler/runtime protocols | Build entrypoints plus new generic orchestrator | Synthetic package installation without registration edits |
| W05 | Typed F* manifest capture | Pinned F* hook and `extraction/` | Static/ghost/runtime classification survives erasure |
| W06 | F* executable KIR export | Current primitive handlers | No CUDA names in the contract; baseline corpus extracts |
| W07 | KIR reference semantics and checker | New validation tools | Specified supported subset and independent test oracle |
| W08 | SPIR-T construction SDK | New `compiler/spirt/` | Typed/control/layout/evidence mapping tests |
| W09 | SPIR-V target emitter | SPIR-T adapter, target validator | Explicit environment, valid modules, reflection agreement |
| W10 | Vulkan runtime basics | New backend package | Real allocation/dispatch/result/cleanup path |
| W11 | Shared memory and layout | `Kuiper.SHMem`, array/views, current helpers | Alignment/layout refinement plus limit/overflow tests |
| W12 | Barriers, subgroups, atomics | Barrier/AtomicOps contracts, runtime | Participation and memory-model obligations resolved |
| W13 | Host plans, queues, epochs | `Kernel.Base`, streams/epochs, async examples | Ordered dependency chains and failure ownership |
| W14 | C/Rust host bindings | New binding packages | Same artifact bytes; lifetime/error/ABI tests |
| W15 | Second source adapter | Independent restricted typed frontend | KIR contract conformance; correct assurance labeling |
| W16 | Numeric policy and math | Float modules and extraction mappings | Exact/approximate tests and evidence per operation |
| W17 | Matrix/vendor extensions | TensorCore/WGMMA interfaces and headers | Qualified shapes/layouts/relations, explicit unsupported cases |
| W18 | Pass/evidence validation | All transformation boundaries | O1–O10 evidence and invalidation behavior |
| W19 | Distinct out-of-tree backend | Independent compiler/runtime package | Frozen-core hashes unchanged; real workload runs |
| W20 | Additive frontend/operation exercise | Public SDK and contracts only | No existing source or build edits |
| W21 | Packaging and CI generalization | Workflows, configure, package scripts | Discovered backend matrix, clean CUDA-free install |
| W22 | Qualification and cutover | Release tooling and support matrix | Performance, soak, canary, rollback, complete claim report |
| W23 | Direct Mesa experiment | Separate version-pinned package | Same-driver comparison and promote/defer decision |

W04's synthetic package exercises discovery only; it does not satisfy W19's real backend gate. W07's evaluator exercises sequential semantics only until concurrency is explicitly modeled. These distinctions must remain in progress reports.

## Requirements traceability

| User requirement | Implementation mechanism | Decisive gates |
|---|---|---|
| Replace CUDA with SPIR-T | Direct KIR/SPIR-T path; new runtime; optional legacy isolation | G-EXTRACT, G-VERTICAL, G-COVERAGE, G-CUDA-FREE, G-REPLACEMENT |
| Production-ready | Named support profiles, real workloads/hardware, failures, packaging, rollback | G-CONCURRENCY, G-TRUST, G-PRODUCTION |
| Modular and decoupled | Four adapter axes, contracts, private worker dependencies | G-CONTRACT, G-ADD-COMPILER, G-ADD-RUNTIME |
| Add new GPU backends by addition alone | External manifests/protocols; immutable-core install exercise | G-ADD-COMPILER, G-ADD-RUNTIME, G-ADD-OP |
| Extraction independent of language | Neutral kernel/host package, independent frontend and host bindings | G-EXTRACT, G-ADD-LANGUAGE |
| Preserve verification and semantics | Explicit obligations, evidence levels, checked transformations, trust inventory | G-TRUST and per-profile O1–O10 |
| Retain performance/control | Typed metadata, explicit pipelines, qualified specialization and tuning | G-COVERAGE, G-PRODUCTION, G-MESA-DECISION where applicable |
## Completion reporting

Report progress by completed work packages, passed gates, corpus coverage, and unresolved obligations. Avoid reporting “80% complete” from line counts or the number of backends that compile a minimal example.

A useful phase report states: what works; exact profile/device/revision; what was checked; what remains assumed; unsupported cases; measured regressions; and the next gate. Attach reproducible artifacts and hashes. Never use the existence of this roadmap as evidence that the new implementation exists.

## Expand the existing work packages

| Work packages | Added deliverable | Audit findings | Required review |
|---|---|---|---|
| W01–W02 | Mandatory legacy-scope ledger, source assumptions, feasibility probes and fixed performance manifest | F01, F05–F11 | Semantics, architecture and release |
| W03–W04 | Canonical KIR/protocol contract, caller/import policy, lifecycle and submission state machines | F12–F17, F19–F21 | Frontend, runtime and verification |
| W05–W07 | Pre-erasure hook trace and checked manifest/body correspondence; independent semantics | F11–F14 | F*/Pulse and verification |
| W08–W09 | Direct SPIR-T adapter, annotation/loop/QPtr eligibility and dependency decision record | F07–F10 | Compiler and numerical |
| W10–W14 | Physical synchronization footprints, guard failures, duplicate submission, package lifetimes and safe binding obligations | F12–F13, F16–F17, F19–F21 | Runtime, memory and concurrency |
| W16–W18 | Required numeric/refinement evidence, fixed assurance policies and identity binding | F02, F04, F07–F09, F21 | Verification and numerical |
| W19–W20 | Compiler-only/runtime-only substitution plus distinct API, frontend/binding and operation extension | F05, F15 | Independent implementer and architecture |
| W21–W22 | Protected CI admission, result evaluation, nonwaivable safety gates, statistical performance decisions and exact-candidate release | F01–F06, F17–F20 | Release and operations |

W15 supplies the second source frontend during P2. W20 adds the independent third frontend/binding after freeze. W23 remains a separate optional Mesa experiment. A work-package number identifies ownership and review boundaries; it does not mean an issue, implementation or proof has already been created.

## Freeze decisions with evidence

For each decision record, write the problem, chosen contract/subset, rejected alternatives and reason, source pins, feasibility artifacts, assumptions, owner role, dependent gates and revisit trigger. The initial Vulkan 1.2 / SPIR-V 1.5 profile is the default proposal; feature support must be queried and enabled. The second API is selected in P0, with Metal preferred where the hardware and common subset are available.

The [upstream decision record](../10-sources/01-lock-and-reproduce-primary-sources.md) identifies reuse candidates and the initial restricted-path strategy. No proposed PR is credited as an installed capability. If a required feature forces a patch, record its exact dependency stack and qualify it before changing the compiler identity.

## Scope the extension promise honestly

For every compatible new backend, installation is additive: new package plus explicit deployment/admission configuration. Existing core/frontends/kernels/bindings/build and registry sources remain untouched. The initial migration is allowed to refactor those files to establish v1. New hardware semantics are carried by the versioned extension envelope and independently admitted lowerers/checkers. An incompatible protocol or semantic change needs a new version; no finite frozen interface can guarantee arbitrary future meanings without such a mechanism.

Full replacement remains the goal. Unsupported hardware-specific semantics remain visible blockers until implemented or replaced by a proved equivalent relation; they cannot disappear by relabeling the first release.

## Evidence required to close this milestone

Close **All phase gates** only with the implementation artifacts, positive and negative cases, and source-to-result identities required above. Link the implementation relation to the named F* symbols and the policy's applicable O1–O10 obligations. The specification's proved lemmas are reusable model facts; they do not discharge this implementation correspondence. Record unresolved cases as blockers or explicitly outside the claim. No backend gate is marked passed by this roadmap revision.
