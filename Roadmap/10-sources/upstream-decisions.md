# Record SPIR-T reuse and dependency decisions

## 1. Baseline choice for the implementation spike

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

## 2. Complete the P0 decision record

The compiler owner records the exact selected stack, parent/prerequisite SHAs, patch hashes, license/notices, tests, local changes, update owner and removal condition. Compare each planned custom component with these candidates. Adopt only the minimal understood subset and replay F07–F09 regressions after upgrades.

If upstream churn changes an internal representation, release a new worker package while preserving KIR/artifact/protocol contracts. If it forces an external semantic change, version that contract explicitly and repeat the additive extension exercises. Never silently reinterpret an old artifact under a new meaning.
