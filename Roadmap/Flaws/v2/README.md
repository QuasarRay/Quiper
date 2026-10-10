# Roadmap v2 audit: remaining flaws and required corrections

V2 fixes the major release-scope, proof-policy and plugin-separation problems identified in the first audit. It still has **8 actionable findings: 3 high and 5 medium**. The most consequential gaps concern failed producer kernels, borrowed asynchronous Rust memory, and an omitted Vulkan memory-model feature dependency.

This audit targets commit [`9dceaf274b46f295f7fc312fb3396d5729d7d97d`](https://github.com/QuasarRay/Quiper/tree/9dceaf274b46f295f7fc312fb3396d5729d7d97d/Roadmap). It reviews all 85 current roadmap instruction/navigation pages, checks the 44 historical audit/resolution pages for traceability, and inspects the five machine-readable/checker files. The [coverage records](05-coverage-and-evidence/README.md) name every file and distinguish semantic review from historical/navigation checks.

## 1. Findings

| ID | Severity | Required correction |
|---|---|---|
| [V2-01](01-runtime-and-ownership/v2-01-dependent-failure.md) | High | Prevent already queued consumers from using a failed producer's postcondition or output |
| [V2-02](01-runtime-and-ownership/v2-02-forgotten-borrows.md) | High | Make safe asynchronous Rust APIs sound when completion objects are forgotten |
| [V2-03](01-runtime-and-ownership/v2-03-operation-retirement.md) | Medium | Bound duplicate-suppression state without replaying retired operations or returning ownership twice |
| [V2-04](02-spirt-and-vulkan/v2-04-loop-exit-values.md) | Medium | Specify the pinned SPIR-T loop's distinct backedge and post-loop value channels |
| [V2-05](02-spirt-and-vulkan/v2-05-memory-model-chains.md) | High, conditional on chain-dependent semantics | Enforce availability/visibility chain support or prove an applicable restriction |
| [V2-06](02-spirt-and-vulkan/v2-06-host-object-synchronization.md) | Medium | Select a concrete host synchronization policy for Vulkan queues and pools |
| [V2-07](03-contracts-and-evidence/v2-07-policy-extensibility.md) | Medium | Reconcile additive assurance policies with the result schema's closed enum |
| [V2-08](04-release-measurement/v2-08-tail-confidence.md) | Medium | Make tail adequacy, degenerate data and bootstrap resolution part of performance decisions |

## 2. Read by implementation area

- [Runtime and ownership](01-runtime-and-ownership/README.md)
- [SPIR-T and Vulkan](02-spirt-and-vulkan/README.md)
- [Contracts and evidence](03-contracts-and-evidence/README.md)
- [Release measurement](04-release-measurement/README.md)
- [Coverage, primary sources and executed diagnostics](05-coverage-and-evidence/README.md)
- [Remediation order and closure requirements](06-remediation/README.md)

Every finding supplies the affected v2 instructions, primary evidence or a concrete counterexample, implementation directions and decisive closure cases. The [v1 resolution review](05-coverage-and-evidence/v1-resolution-review.md) explains which earlier corrections hold and where this audit adds a residual obligation.

## 3. Interpret the result accurately

High means the gap can invalidate a safety, semantic-preservation or qualification claim if the affected design is implemented without correction. Medium means a material protocol, feasibility or acceptance decision is still missing or contradictory. Severity describes the consequence of the roadmap gap, not an observed production incident.

The schema membership check and statistical arithmetic were executed. Runtime examples are design counterexamples. The SPIR-T result convention is confirmed from pinned source. No F* build, SPIR-T build, GPU execution or end-to-end proof was performed. The [diagnostic record](05-coverage-and-evidence/diagnostics.md) states exactly what ran.

Missing future code is not counted as a flaw. Nor are already corrected v1 issues repeated as new findings. [Rejected suspicions](05-coverage-and-evidence/not-findings.md) record checks that did not justify a finding. This is a broad, evidence-based audit; it cannot certify that no further flaws exist.

All eight findings are now corrected in the v3 roadmap. Follow [the resolution records](resolutions/README.md) for current instructions, model evidence and required implementation closure cases. No backend implementation or release gate is marked passed.
