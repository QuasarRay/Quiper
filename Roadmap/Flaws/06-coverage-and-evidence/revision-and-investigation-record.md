# 2. Revision and investigation record

**Historical audit record.** This describes the audit snapshot, not the current correction status. See [current resolutions](../README/resolutions.md).

## 2. Revision and investigation record

| Item | Anchor | Inspected scope |
|---|---|---|
| Roadmap | [`689c4528f227704df989f0f1e3eaabb8ce4b600a`](https://github.com/QuasarRay/Quiper/tree/689c4528f227704df989f0f1e3eaabb8ce4b600a/Roadmap) | All 12 Markdown files and `milestones.json` |
| Kuiper implementation | [`413219948f91911ffaf0ac37a5ff941c5d1e55c7`](https://github.com/QuasarRay/Quiper/tree/413219948f91911ffaf0ac37a5ff941c5d1e55c7) | Extraction, relevant proof interfaces, assertions/guards, runtime helpers, build/verification configuration; original source audit cross-checked selectively |
| SPIR-T main | [`e8757adba8d14068a7bf1b3bc9f24cac982f4bd3`](https://github.com/Rust-GPU/spirt/tree/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3) | IR attributes, SPIR-V lowering/lifting, QPtr analysis/lowering/lifting, examples and existing source-scope record |
| SPIR-T upstream work | [Issues](https://github.com/Rust-GPU/spirt/issues), [PRs](https://github.com/Rust-GPU/spirt/pulls), [branches](https://github.com/Rust-GPU/spirt/branches) | Inventory of 27 open issue/PR records: 21 PRs and 6 issues; 21 closed PR records; 27 branches. Targeted inspection of relevant descriptions and patches, especially #30/#31 and #48 |
| F* fork | [`0eef57bef411aac090354a75c21e00b674bd420c`](https://github.com/QuasarRay/FStar/tree/0eef57bef411aac090354a75c21e00b674bd420c) | Recursive path inventory; `Pulse.Extract.Main` interface/implementation and selected `Pulse2Rust.Extract` interface; no full compiler audit |
| Karamel fork | `75bc9443b430f5161d85ff02eedb385e9a6db607` | Gitlink and existing Quiper integration evidence only; no new full Karamel analysis |
| Target project PRs | QuasarRay/Quiper, before this audit PR | No open PRs returned at inspection; this does not establish absence of work in forks or uninspected branches |

These inventories were retrieved on 2026-10-09 UTC. Each collection fit within the requested 100-record page. Counts are a snapshot, not a permanent property of the repositories. Closed-PR and branch inventories were screened for related work; their complete implementations were not reviewed.

Important upstream anchors:

| Work | Observed state | Exact inspected head where used | Audit consequence |
|---|---|---|---|
| [#48: ID-bearing annotations/float flags](https://github.com/Rust-GPU/spirt/pull/48) | Open, not draft | `94f5c19c3d3ad258bc628857a350719561d64b44` | F07; patch inspected, not built |
| [#30: loop shortcut canonicalization](https://github.com/Rust-GPU/spirt/pull/30) | Open, draft | `0e40966f27468d4896b51e28ea0e9080a8300bbe` | F08; report/proposal examined, reproducer not run |
| [#31: loop-carried phi report](https://github.com/Rust-GPU/spirt/issues/31) | Open issue | Issue body at inspection | F08; source author report remains unconfirmed here |
| [#46: `spirti` prototype](https://github.com/Rust-GPU/spirt/pull/46) | Open, draft, based on stacked work | `7e664801ece15c16068c3b5edd5fb00895217e24` | F10; reuse candidate, no production or oracle claim |
| [#22: specialization/function-pointer constants](https://github.com/Rust-GPU/spirt/pull/22) | Closed without merge | `c16c5e9452825436d9f4bce2a1358969a3816535` | Additional reuse-history lead; no support credited to main from this proposal |
| #25/#26/#33–#45 | Open development work | Inventory/selected descriptions | F10; integration strategy must account for dependent architectural changes |

The branch inventory includes QPtr/legalization, memory, interpreter, scalar/vector, and Vulkan-header work. A branch name is a lead for implementation research, not proof that it fixes F07–F09. Issue #47 requests Metal support; a request is not an available Metal backend.
