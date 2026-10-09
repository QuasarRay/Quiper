# Roadmap audit: flaws, unresolved contracts, and release blockers

The roadmap needs another design pass before it can govern the migration. The overall separation of extraction, SPIR-T compilation, runtime, and host bindings is useful. Several acceptance rules remain too weak to establish the required result, and the upstream investigation missed concrete SPIR-T limitations and relevant unmerged work.

This collection records **21 findings: 13 high and 8 medium**. Each finding identifies the affected text, the evidence, a failure case, the required correction, and a test or decision that would close it. All findings are open. A proposed closure test is not a test that has already passed.

## 1. Exact scope

The audit targets [roadmap commit `689c4528f227704df989f0f1e3eaabb8ce4b600a`](https://github.com/QuasarRay/Quiper/tree/689c4528f227704df989f0f1e3eaabb8ce4b600a/Roadmap), inspected on **2026-10-09 UTC**. The underlying Kuiper implementation is unchanged from `413219948f91911ffaf0ac37a5ff941c5d1e55c7`. SPIR-T was inspected at `e8757adba8d14068a7bf1b3bc9f24cac982f4bd3`, together with its open issue/PR inventory and selected proposed changes. The pinned F* submodule was inspected selectively around Pulse extraction and erasure.

The subject is the roadmap. Missing future implementations are expected; they are not counted as defects. Findings concern contradictions, material omissions in the investigation, and contracts or gates that admit a concrete bad outcome. Some decisions are already marked pending in the roadmap. Those findings explain the consequence of leaving them pending and the evidence needed to resolve them.

**High** means the issue can invalidate a production, verification, replacement, or independent-extension claim if left unresolved. **Medium** means a material planning, feasibility, or acceptance ambiguity must be resolved before its affected milestone. Severity does not establish that a bug has occurred in deployed code.

Evidence classes distinguish direct source inspection, contradictory requirements, contract/gate gaps, feasibility gaps, and upstream reports. In particular, **F08 is an upstream-reported compiler problem that was not reproduced in this audit**. No GPU qualification, compiler build, or end-to-end proof was performed. [Coverage and limits](06-coverage-and-evidence.md) specify what was inspected and what remains untested. No finite audit can establish that no further flaws exist.

## 2. Finding index

| ID | Severity | Finding | Evidence class |
|---|---|---|---|
| [F01](01-release-and-gates.md#f01) | High | A scoped release can complete while the CUDA replacement objective remains open | Scope/gate gap |
| [F02](01-release-and-gates.md#f02) | High | The verified profile has no committed minimum evidence policy | Assurance-policy gap |
| [F03](01-release-and-gates.md#f03) | High | The final checklist permits exceptions to failure and cleanup tests | Contradictory requirements |
| [F04](01-release-and-gates.md#f04) | Medium | Gate results have no specified identity or invalidation record | Evidence-enforcement gap |
| [F05](01-release-and-gates.md#f05) | Medium | Second-runtime selection has two incompatible deadlines | Contradictory requirements |
| [F06](01-release-and-gates.md#f06) | Medium | Performance budgets do not define an executable pass/fail rule | Measurement-policy gap |
| [F07](02-spirt-and-lowering.md#f07) | High | The pinned bridge rejects ID-bearing annotations needed by some planned paths | Confirmed source limitation |
| [F08](02-spirt-and-lowering.md#f08) | High | A reported loop-lifting problem has no explicit triage gate | Upstream report; not reproduced |
| [F09](02-spirt-and-lowering.md#f09) | High | QPtr's usable subset is narrower than the migration matrix makes actionable | Confirmed source limitations |
| [F10](02-spirt-and-lowering.md#f10) | Medium | The reuse and maintenance decision omits active upstream architecture work | Investigation gap |
| [F11](03-extraction-and-verification.md#f11) | Medium | The extraction spike lacks the actual Pulse erasure boundary and correspondence test | Feasibility/evidence gap |
| [F12](03-extraction-and-verification.md#f12) | High | Conditional kernel proofs are not connected to foreign-caller obligations | Host-contract gap |
| [F13](03-extraction-and-verification.md#f13) | Medium | Typed host imports have no concrete assurance rule for their implementations | Trust-boundary gap |
| [F14](03-extraction-and-verification.md#f14) | Medium | Compiled host plans are optional in one place and mandatory in another | Acceptance ambiguity |
| [F15](04-backends-and-plugins.md#f15) | High | A second complete backend does not prove compiler/runtime independence | Acceptance-test gap |
| [F16](04-backends-and-plugins.md#f16) | Medium | Panic containment does not distinguish recovery from process termination | Failure-contract ambiguity |
| [F17](04-backends-and-plugins.md#f17) | High | Independent package withdrawal lacks a lifetime rule for active objects | Lifecycle-contract gap |
| [F18](04-backends-and-plugins.md#f18) | High | Manifest-driven hardware CI has no defined execution trust boundary | Deployment-design gap |
| [F19](05-runtime-and-memory.md#f19) | High | IPC submission has no protocol for an unknown acceptance outcome | Protocol gap |
| [F20](05-runtime-and-memory.md#f20) | High | Byte-range ownership omits noncoherent cache-atom overlap | Memory-contract gap |
| [F21](05-runtime-and-memory.md#f21) | High | Assertions and guards need different semantics, including failure at barriers | Source distinction and lowering gap |

## 3. How to act on this collection

Start with the [remediation order](07-remediation-order.md). Resolve the release meaning and evidence policy, then run the upstream and extraction feasibility probes. Specify the runtime and plugin contracts before freezing v1. Re-run the independent-extension exercises against the actual release candidate.

The detailed findings preserve distinctions that matter: an unsupported input is not a silent miscompilation; an upstream PR is not a qualified fix; an unchecked frontend is not inherently a design error; and an explicit future task is not evidence that the task is complete.

This change adds audit documents only. It leaves the original roadmap and implementation intact so each correction can be reviewed against the audited text.
