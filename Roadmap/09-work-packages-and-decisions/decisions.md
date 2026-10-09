# Decisions

## 2. Decisions made by this roadmap

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
## 3. P0/P1 decisions still requiring evidence

1. Validate the chosen Vulkan 1.2 / SPIR-V 1.5 baseline and its explicit feature set against the minimum device/driver matrix. Changing that choice requires an updated target-profile digest.
2. Exact typed extraction hook and changes needed in the pinned F* fork.
3. KIR initial operation grammar, canonical encoding, and the proof framework/checker for its semantics.
4. Validate the initial portable corpus. Every baseline extraction root, generated instantiation source, and reachable semantic dependency remains mandatory for full replacement; a smaller portable release cannot retire those obligations.
5. Memory model, subgroup guarantees, failure semantics, and numerical policies to freeze.
6. Host ABI/platform coverage, IPC/in-process transports, and benchmark-derived batching policy.
7. Second runtime hardware availability and the smallest workload subset that honestly tests the same portable contract.
8. Ownership of upstream patches, security/bug triage, driver qualification, and long-term supported versions.

These are explicit engineering decisions with default direction supplied by this roadmap. They should be resolved through spikes and evidence, not by silently adding assumptions to the implementation.
