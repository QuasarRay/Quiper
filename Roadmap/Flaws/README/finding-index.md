# 2. Finding index

**Historical audit record.** This describes the audit snapshot, not the current correction status. See [current resolutions](resolutions.md).

## 2. Finding index

| ID | Severity | Finding | Evidence class |
|---|---|---|---|
| [F01](../01-release-and-gates/f01.md#f01) | High | A scoped release can complete while the CUDA replacement objective remains open | Scope/gate gap |
| [F02](../01-release-and-gates/f02.md#f02) | High | The verified profile has no committed minimum evidence policy | Assurance-policy gap |
| [F03](../01-release-and-gates/f03.md#f03) | High | The final checklist permits exceptions to failure and cleanup tests | Contradictory requirements |
| [F04](../01-release-and-gates/f04.md#f04) | Medium | Gate results have no specified identity or invalidation record | Evidence-enforcement gap |
| [F05](../01-release-and-gates/f05.md#f05) | Medium | Second-runtime selection has two incompatible deadlines | Contradictory requirements |
| [F06](../01-release-and-gates/f06.md#f06) | Medium | Performance budgets do not define an executable pass/fail rule | Measurement-policy gap |
| [F07](../02-spirt-and-lowering/f07.md#f07) | High | The pinned bridge rejects ID-bearing annotations needed by some planned paths | Confirmed source limitation |
| [F08](../02-spirt-and-lowering/f08.md#f08) | High | A reported loop-lifting problem has no explicit triage gate | Upstream report; not reproduced |
| [F09](../02-spirt-and-lowering/f09.md#f09) | High | QPtr's usable subset is narrower than the migration matrix makes actionable | Confirmed source limitations |
| [F10](../02-spirt-and-lowering/f10.md#f10) | Medium | The reuse and maintenance decision omits active upstream architecture work | Investigation gap |
| [F11](../03-extraction-and-verification/f11.md#f11) | Medium | The extraction spike lacks the actual Pulse erasure boundary and correspondence test | Feasibility/evidence gap |
| [F12](../03-extraction-and-verification/f12.md#f12) | High | Conditional kernel proofs are not connected to foreign-caller obligations | Host-contract gap |
| [F13](../03-extraction-and-verification/f13.md#f13) | Medium | Typed host imports have no concrete assurance rule for their implementations | Trust-boundary gap |
| [F14](../03-extraction-and-verification/f14.md#f14) | Medium | Compiled host plans are optional in one place and mandatory in another | Acceptance ambiguity |
| [F15](../04-backends-and-plugins/f15.md#f15) | High | A second complete backend does not prove compiler/runtime independence | Acceptance-test gap |
| [F16](../04-backends-and-plugins/f16.md#f16) | Medium | Panic containment does not distinguish recovery from process termination | Failure-contract ambiguity |
| [F17](../04-backends-and-plugins/f17.md#f17) | High | Independent package withdrawal lacks a lifetime rule for active objects | Lifecycle-contract gap |
| [F18](../04-backends-and-plugins/f18.md#f18) | High | Manifest-driven hardware CI has no defined execution trust boundary | Deployment-design gap |
| [F19](../05-runtime-and-memory/f19.md#f19) | High | IPC submission has no protocol for an unknown acceptance outcome | Protocol gap |
| [F20](../05-runtime-and-memory/f20.md#f20) | High | Byte-range ownership omits noncoherent cache-atom overlap | Memory-contract gap |
| [F21](../05-runtime-and-memory/f21.md#f21) | High | Assertions and guards need different semantics, including failure at barriers | Source distinction and lowering gap |
