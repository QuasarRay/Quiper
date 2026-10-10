# Lock and reproduce the primary-source evidence

**Milestone M30.** A source lock and reproducible upstream probes tied to exact claims and revisions.

## Required inputs and specification

Start from [M01](../README/01-freeze-the-goal.md). Use the exit-gate dependencies in [the implementation plan](../milestones.json); proof and investigation work may begin earlier. Read [the declarative specification](../Specification/README.md) before choosing representation details. This milestone implements `Operations.catalog_covers`, `Extension.evidence_bound`.

## Quiper / Kuiper evidence

All Q-links refer to Quiper commit `413219948f91911ffaf0ac37a5ff941c5d1e55c7`.

| ID | Source | What was used |
|---|---|---|
| Q1 | [README](https://github.com/FStarLang/kuiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/README.md), [AGENTS](https://github.com/FStarLang/kuiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/AGENTS.md) | Current purpose, CUDA pipeline, admitted-proof notice, build conventions |
| Q2 | [ExtractKuiper.fst](https://github.com/FStarLang/kuiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/extraction/ExtractKuiper.fst), [ExtractionUtils.fst](https://github.com/FStarLang/kuiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/extraction/ExtractionUtils.fst) | Actual compiler hooks, primitive mappings, type erasure limitations, launch and math lowering |
| Q3 | [verify.mk](https://github.com/FStarLang/kuiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/verify.mk), [extraction/Makefile](https://github.com/FStarLang/kuiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/extraction/Makefile), [fixup.sed](https://github.com/FStarLang/kuiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/scripts/fixup.sed) | Toolchain/plugin build modes, CUDA extraction rules, textual postprocessing |
| Q4 | [kuiper.h](https://github.com/FStarLang/kuiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/include/kuiper.h), [atomics.h](https://github.com/FStarLang/kuiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/include/kuiper/atomics.h), [nvcc.mk](https://github.com/FStarLang/kuiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/nvcc.mk) | Native CUDA runtime dependence, helpers, target selection, test/extraction roots |
| Q5 | [Barrier.Warp](https://github.com/FStarLang/kuiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/src/lib/kuiper/Kuiper.Barrier.Warp.fsti), [Barrier](https://github.com/FStarLang/kuiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/src/lib/kuiper/Kuiper.Barrier.fsti) | Fixed warp width, existing contract warning, block barrier resources |
| Q6 | [SizeT](https://github.com/FStarLang/kuiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/src/lib/kuiper/Kuiper.SizeT.fst) | Admitted width relation and explicit 32-bit operations |
| Q7 | [SHMem](https://github.com/FStarLang/kuiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/src/lib/kuiper/Kuiper.SHMem.fsti) | Consecutive shared slices and proof-visible base/alignment relations |
| Q8 | [Kernel.Base](https://github.com/FStarLang/kuiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/src/lib/kuiper/Kuiper.Kernel.Base.fsti), [Kernel.Stream](https://github.com/FStarLang/kuiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/src/lib/kuiper/Kuiper.Kernel.Stream.fsti), [Async.Chain example](https://github.com/FStarLang/kuiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/src/examples/Kuiper.Example.Async.Chain.fst) | Queue/epoch/pledge semantics and an existing dependent-launch workload |
| Q9 | [TensorCore.WGMMA](https://github.com/FStarLang/kuiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/src/lib/kuiper/Kuiper.TensorCore.WGMMA.fsti), [WGMMA.Layout](https://github.com/FStarLang/kuiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/src/lib/kuiper/Kuiper.TensorCore.WGMMA.Layout.fsti) | Distinct numerical relation and packed instruction-specific layouts |
| Q10 | [AtomicOps](https://github.com/FStarLang/kuiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/src/lib/kuiper/Kuiper.AtomicOps.fsti) | Current atomic operation interfaces and value contracts |
| Q11 | [CI workflow](https://github.com/FStarLang/kuiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/.github/workflows/ci.yml), [list-admits.py](https://github.com/FStarLang/kuiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/scripts/list-admits.py) | Existing build/verification checks and lexical trust inventory support |
| Q12 | [.gitmodules](https://github.com/FStarLang/kuiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/.gitmodules), [repository tree](https://github.com/QuasarRay/Quiper/tree/413219948f91911ffaf0ac37a5ff941c5d1e55c7) | Repository paths, submodule topology, source/generated surface |

Observed submodule commits: F* `0eef57bef411aac090354a75c21e00b674bd420c`; Karamel `75bc9443b430f5161d85ff02eedb385e9a6db607`. Their full implementation was not audited in this task. The roadmap requires that investigation before selecting extraction hooks or asserting preservation theorems.

The raw file/line counts in document 00 count regular files under the named directories in the clean pinned checkout, using newline-separated bytes. They include comments, interfaces, scripts, and generated/distribution material. The 240-name count is the set of quoted `Kuiper.*` identifiers found lexically in `ExtractKuiper.fst`, not a completeness proof for supported features.

## SPIR-T evidence

All S-links refer to SPIR-T commit `e8757adba8d14068a7bf1b3bc9f24cac982f4bd3`. Its package manifest reports version `0.4.0`; the commit identity is the audit anchor.

| ID | Source | What was used |
|---|---|---|
| S1 | [README](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/README.md) | Shader orientation, evolving scope, OpenCL/text-parser exclusions, available facilities |
| S2 | [IR definitions](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/lib.rs), [Cargo.toml](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/Cargo.toml) | Public construction, SPIR-V-oriented variants, context ownership, safe-Rust policy, dependency/version boundary |
| S3 | [SPIR-V module](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/spv/mod.rs), [legalization](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/passes/legalize.rs) | Conversion terminology, representation, structurization entrypoint |
| S4 | [QPtr model](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/qptr/mod.rs), [layout configuration](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/qptr/layout.rs), [QPtr example](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/examples/spv-lower-link-qptr-lift.rs) | Pointer/provenance assumptions, extent widths, configuration, illustrative pass sequence |
| S5 | [Repository tree](https://github.com/Rust-GPU/spirt/tree/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3), [.gitmodules](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/.gitmodules) | Inspected implementation scope and SPIRV-Headers dependency |

“No direct Mesa emitter found” describes this inspected tree. It is not a claim that no prototype exists anywhere in the ecosystem or in uninspected branches.

## Baseline choice for the implementation spike

Use main at `e8757adba8d14068a7bf1b3bc9f24cac982f4bd3` as the baseline, with literal specialization and typed logical buffers. No unmerged patch is selected as a qualified dependency in this roadmap. General affected loop paths remain blocked pending triage, even for direct construction. This is a conservative starting subset; it is not sufficient for complete Kuiper migration.

| Candidate | Initial decision and reason | Required evidence before adoption |
|---|---|---|
| [#48 ID-bearing/float annotations](https://github.com/Rust-GPU/spirt/pull/48), head `94f5c19c3d3ad258bc628857a350719561d64b44` | Defer from baseline; materialize supported literals and reject other forms | Representation/traversal/renumbering/emission regressions and exact target-feature/numeric tests |
| [#30 loop lifting](https://github.com/Rust-GPU/spirt/pull/30), head `0e40966f27468d4896b51e28ea0e9080a8300bbe`; [#31 report](https://github.com/Rust-GPU/spirt/issues/31) | Blocking triage before affected shapes; no claim of reproduction | Defined-input relation, minimized reproduction, reviewed fix or enforced/proved exclusion |
| [#25 memory separation](https://github.com/Rust-GPU/spirt/pull/25), [#26 control-flow reorganization](https://github.com/Rust-GPU/spirt/pull/26) | Reuse candidates for private SDK design; avoid making their unstable layouts public | Exact heads/prerequisite stack and comparison to current adapter requirements |
| [#42 scalar operations](https://github.com/Rust-GPU/spirt/pull/42), [#43 vectors](https://github.com/Rust-GPU/spirt/pull/43), [#45 aggregates](https://github.com/Rust-GPU/spirt/pull/45) | Evaluate before implementing overlapping transformations; no capability credit from proposal existence | Accepted operation semantics, numeric/control/memory regressions, maintenance cost |
| [#41 Vulkan round-trip layer](https://github.com/Rust-GPU/spirt/pull/41) | Candidate test scaffolding; it is not Kuiper's production runtime | Resource/feature coverage and separation from the independent test oracle |
| [#46 CPU interpreter](https://github.com/Rust-GPU/spirt/pull/46), inspected head `7e664801ece15c16068c3b5edd5fb00895217e24` | Diagnostic reuse only until qualified; cannot replace host plans or independent KIR semantics | Supported operations, failure semantics and independence analysis |

PR #30 and #48 metadata was rechecked during this revision; both remained open, and #30 remained a draft, at the heads above. The other entries preserve the original audit's candidate inventory; their current heads/dependency graph must be rechecked before adoption. No proposed fix has been built or executed here.

## Complete the P0 decision record

The compiler owner records the exact selected stack, parent/prerequisite SHAs, patch hashes, license/notices, tests, local changes, update owner and removal condition. Compare each planned custom component with these candidates. Adopt only the minimal understood subset and replay F07–F09 regressions after upgrades.

If upstream churn changes an internal representation, release a new worker package while preserving KIR/artifact/protocol contracts. If it forces an external semantic change, version that contract explicitly and repeat the additive extension exercises. Never silently reinterpret an old artifact under a new meaning.

## Documentation review performed

This revision reorganizes the roadmap and specifies corrections to the audit. It remains documentation, not an implementation. Its review checks relative document links, pinned repository source paths, milestone/gate consistency, Markdown structure, machine-readable JSON validity, and absence of implementation changes. It does not run the future proof/compiler/runtime release gates. Completion of those gates belongs to the implementation phases above.

## Revision evidence

The original main-only source inspection is supplemented by the historical flaw audit and this revision: official F*/Pulse manuals, pinned Pulse erasure source, Kuiper assertion/launch contracts, SPIR-T context/annotation/QPtr code, and current metadata for PRs #30/#48. See [claim evidence](02-check-each-correctness-claim.md) and [upstream decisions](01-lock-and-reproduce-primary-sources.md). No compiler or GPU qualification was executed in this documentation task.

## Evidence required to close this milestone

Close **G-BASELINE, G-SPIRT-FEASIBILITY** only with the implementation artifacts, positive and negative cases, and source-to-result identities required above. Link the implementation relation to the named F* symbols and the policy's applicable O1–O10 obligations. The specification's proved lemmas are reusable model facts; they do not discharge this implementation correspondence. Record unresolved cases as blockers or explicitly outside the claim. No backend gate is marked passed by this roadmap revision.
