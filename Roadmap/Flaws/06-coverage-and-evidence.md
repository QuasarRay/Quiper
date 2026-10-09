# 6. Audit coverage, evidence, and limits

## 1. What this audit establishes

The findings are based on the complete roadmap, selected implementation paths, upstream issue/PR inventories, and primary documentation. They establish defects or unresolved decisions in the migration plan at the pinned revision. They do not establish that a future implementation has failed a test, or that all possible defects have been found.

The review separates four questions:

1. Does the roadmap contradict itself or permit a weaker result than the requested goal?
2. Does the inspected dependency actually support the required operation or transformation?
3. Can a concrete failure case pass through an unspecified contract or insufficient gate?
4. What observation, proof, policy decision, or regression would close the finding?

Missing implementation code was not counted as a flaw. No conclusion rests solely on a `TODO`, a raw line count, a search hit, or the existence of an upstream PR.

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

## 3. Coverage of every roadmap file

| Roadmap file | Review focus | Findings or disposition |
|---|---|---|
| [README](../README.md) | User goal, completion meaning, architecture and no-edit boundary | F01, F15; bounded contract-version guarantee is sound |
| [00 — Current state](../00-current-state-and-gaps.md) | Accuracy and limits of the original repository audit | F07–F11, F21 add material dependency/source evidence; pre-existing warp/width risks already acknowledged |
| [01 — Architecture](../01-target-architecture.md) | Module dependencies, representations, four extension axes, planner | F11, F15, F17; no evidence that its DAG requires each pass to be single-shot |
| [02 — Extraction](../02-language-independent-extraction.md) | Erasure, kernel/host contract, caller obligations, foreign imports, independent languages | F11–F14, F21 |
| [03 — Extensions](../03-backend-extension-contract.md) | Discovery, ABI, capabilities, no-edit exercises, withdrawal | F05, F15–F19; exact capability tuples are already required |
| [04 — Lowering](../04-spirt-and-gpu-lowering.md) | Direct construction, control flow, pointer subset, specialization, numeric/matrix behavior | F07–F10, F21; no assumed universal matrix/FP fallback was found |
| [05 — Runtime](../05-runtime-and-interop.md) | Asynchronous lifetime, failure, host ABI, visibility, package operations | F12–F14, F16–F17, F19–F21 |
| [06 — Verification](../06-verification-and-trust.md) | O1–O10, trusted base, conditional theorem, evidence integrity | F02, F04, F08, F11–F13, F19–F21; source proof is already distinguished from compiler proof |
| [07 — Phases](../07-implementation-phases.md) | Dependencies, feasibility, freeze/retest timing, effort claims | F01, F04–F05, F11, F14–F15; phase dependency and effort arithmetic checks did not expose an error |
| [08 — Acceptance](../08-production-acceptance.md) | Scope, hardware, oracles, performance, CI, exceptions, release | F01–F04, F06, F15, F18–F21 |
| [09 — Work packages](../09-work-packages-and-decisions.md) | Assignment of obligations and unresolved decisions | F01–F02, F05, F10–F11, F14; remediation maps remaining items to owning roles |
| [10 — Sources](../10-sources.md) | Provenance, audit breadth, mutable external specifications | F07–F11, F21; disclosed audit limits were respected rather than treated as false claims |
| [milestones.json](../milestones.json) | Dependency graph, gate criteria, planning status, evidence model | F01–F04, F14; valid JSON and planning data do not constitute executed gate evidence |

Every file was reviewed. This is document coverage, not proof that every source primitive or every GPU execution was checked.

## 4. Suspicions checked and not counted as findings

1. **“SPIR-T is treated as a runtime.”** The roadmap explicitly distinguishes IR compilation from device execution and runtime responsibilities.
2. **“The no-edit promise covers every unknown future semantic change.”** It explicitly bounds compatibility and requires versioning for incompatible semantics. The problem is testing the actual separations, covered by F15.
3. **“P6 starts verification only after the compiler is finished.”** P6's exit depends on P5, but its work starts at P1. `milestones.json` states that dependency semantics.
4. **“P7 is missing a P2 dependency.”** P7 depends on P4, which transitively depends on P2. The prose additionally names the language/host boundary.
5. **“The effort total is arithmetically wrong.”** The listed ranges sum to 56–122 person-weeks. They are explicitly tentative, exclude optional Mesa work, and are not a calendar promise. Their adequacy remains an implementation-planning question.
6. **“The roadmap permits evidence to survive any compiler change.”** Document 06 requires invalidation and P4 requires a repeated freeze/test. F04 concerns enforcement and result identity, not absence of those rules.
7. **“QPtr drops memory operands.”** The inspected branches leave those loads/stores unchanged. F09 concerns the mixed/unhandled subset.
8. **“All float controls are silently lost.”** The inspected ID-bearing input path rejects unsupported annotations. F07 is specific about that limitation and does not generalize it to every FP mode.
9. **“A retained `OpUndef` proves the loop report.”** It does not. F08 requires a defined-behavior analysis and reproduction before treating the report as a confirmed miscompilation.
10. **“Two Vulkan vendors prove independent runtime addition.”** The roadmap already requires a distinct API. F15 adds independent substitutions rather than replacing that requirement.
11. **“An unchecked C frontend falsely receives source verification.”** The proposed fields explicitly prevent that. F12 concerns safe execution and caller obligations, not the existence of an unchecked mode.
12. **“A scalar interpreter or `spirv-val` proves concurrent functional correctness.”** The roadmap explicitly rejects those conclusions.
13. **“The successful-execution theorem proves termination.”** It explicitly separates exceptional behavior and fairness/termination assumptions. A release still needs its declared operational evidence; partial correctness is not itself a contradiction.
14. **“The failed-resource diagram says a timeout completes work.”** The text explicitly says it does not. F19 concerns unknown submission acceptance, a separate state that the protocol must handle.
15. **“Mesa bypass is promised to improve runtime performance.”** The plan makes it an optional measured experiment and allows a negative result.
16. **“A DAG planner forbids fixed-point optimization.”** The text does not prohibit a bounded iterative composite pass or repeated scheduled instances. The pass specification should settle details; no contradiction was established.

## 5. Primary specifications used in this audit

| Reference | Use | Limit |
|---|---|---|
| [Vulkan runtime SPIR-V rules](https://docs.vulkan.org/refpages/latest/refpages/source/RuntimeSpirv.html) | `LocalSizeId` feature requirement in F07 | Does not establish SPIR-T support or qualify a driver |
| [Vulkan flush semantics](https://docs.vulkan.org/refpages/latest/refpages/source/vkFlushMappedMemoryRanges.html), [mapped range validity](https://docs.vulkan.org/refpages/latest/refpages/source/VkMappedMemoryRange.html) | Physical synchronization footprint in F20 | Example is a contract counterexample, not a reproduced Kuiper race |
| [Vulkan shader execution](https://docs.vulkan.org/spec/latest/chapters/shaders.html) | Workgroup/subgroup context for collective failure analysis | Exact execution/memory environment remains a P0/P1 choice |
| [Rust `catch_unwind`](https://doc.rust-lang.org/std/panic/fn.catch_unwind.html) | Recoverable unwind versus abort in F16 | Does not describe every language's exception runtime |
| [GitHub Actions secure use](https://docs.github.com/en/actions/reference/security/secure-use#hardening-for-self-hosted-runners) | Hardware-runner trust boundary in F18 | Proposed CI threat model, not an allegation about current runner configuration |

These external pages were consulted on 2026-10-09 UTC and use mutable documentation URLs. Qualification must pin the actual SDK/specification/tool versions chosen for the implementation. Repository source evidence is pinned by commit.

## 6. Remaining investigation and validation

| Not performed here | Why it matters | Required next evidence |
|---|---|---|
| Build/run the SPIR-T loop fixture or candidate fixes | Distinguishes a report from a confirmed applicable compiler bug | F08 reproduction and defined-outcome analysis; F07/F09 patch qualification |
| Build the full pinned F*/Pulse/Karamel toolchain | Source inspection cannot prove hook availability or extraction preservation | F11 real-entrypoint trace and O1/O2 evidence |
| Run Vulkan/Metal/CUDA hardware tests | No measured portability, correctness, performance, or device-loss claim is possible | P0 baseline and qualified P3–P8 hardware matrix |
| Prove KIR/runtime/checker soundness | The proposed contracts and validators do not exist yet | Versioned semantics, permitted trusted boundaries, O1–O10 evidence |
| Fully review every branch, closed issue, fork, or dependency | Inventories do not establish global ecosystem completeness | Focused dependency/reuse work under F10; expand when a release dependency changes |
| Exhaustively enumerate all reachable primitive/proof closures | Selected interfaces do not establish complete parity | W01 semantic ledger and per-entrypoint transitive proof inventory |
| Inspect deployed CI runners or credentials | This audit does not know their operational configuration | F18 protected admission and isolation review |

The audit leaves these limits visible. They are required implementation/research tasks, not reasons to treat the listed findings as resolved.

## 7. Documentation validation

The collection was checked for unique finding IDs, agreement between the index and detailed severities, required location/correction/closure fields, balanced Markdown fences, local link targets and heading anchors, and final newlines/whitespace. Pinned Quiper and SPIR-T source paths and referenced line ranges were checked against the corresponding git objects. Cited F* paths were checked against the retrieved pinned tree and file contents.

The result is eight Markdown files containing 21 findings, with 13 high and 8 medium severities. The change is confined to additions under `Roadmap/Flaws`. This validation checks the audit documents; none of the proposed implementation closure tests is reported as executed.
