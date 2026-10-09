# Resolve all 21 roadmap findings

This revision corrects the specification and acceptance requirements for all 21 findings. It does not claim to have implemented Kuiper's new compiler/runtime or executed the proposed closure tests. In particular, F08 remains an unreproduced upstream report; the corrected roadmap blocks affected paths until triage is complete.

| Finding | Correction | Instructions | Roadmap | Implementation evidence |
|---|---|---|---|---|
| [F01](../01-release-and-gates/f01.md) | Mandatory legacy-scope coverage and separate G-REPLACEMENT; deferral cannot complete migration. | [Procedure](../../00-current-state-and-gaps/implementation.md) | Corrected | Pending |
| [F02](../01-release-and-gates/f02.md) | Fixed assurance policies with required O1–O10 evidence and an enumerated trust boundary. | [Procedure](../../06-verification-and-trust/evidence-policy.md) | Corrected | Pending |
| [F03](../01-release-and-gates/f03.md) | Nonwaivable safety, ownership, failure, cleanup and applicable soak gates; performance-only waivers. | [Procedure](../../08-production-acceptance/gate-records.md) | Corrected | Pending |
| [F04](../01-release-and-gates/f04.md) | Candidate/input-bound append-only results, dependency invalidation and final-candidate extension tests. | [Procedure](../../08-production-acceptance/gate-records.md) | Corrected | Pending |
| [F05](../01-release-and-gates/f05.md) | Select second API/hardware/common subset in P0; implement and test it in P7. | [Procedure](../../07-implementation-phases/implementation.md) | Corrected | Pending |
| [F06](../01-release-and-gates/f06.md) | Fixed ratio direction, weights, limits, uncertainty, repeated-look budget and evaluator fixtures. | [Procedure](../../08-production-acceptance/performance-procedure.md) | Corrected | Pending |
| [F07](../02-spirt-and-lowering/f07.md) | Restricted literal baseline; reject unsupported annotations; qualify a patched profile separately. | [Procedure](../../04-spirt-and-gpu-lowering/emission-and-validation.md) | Corrected | Pending |
| [F08](../02-spirt-and-lowering/f08.md) | Block affected loop paths pending justified reproduction, fix, or enforced/proved exclusion. | [Procedure](../../04-spirt-and-gpu-lowering/control-and-pointers.md) | Corrected | Pending |
| [F09](../02-spirt-and-lowering/f09.md) | Explicit optional QPtr eligibility with call/merge/memory-operand/range checks before and after passes. | [Procedure](../../04-spirt-and-gpu-lowering/control-and-pointers.md) | Corrected | Pending |
| [F10](../02-spirt-and-lowering/f10.md) | Baseline decision and per-candidate adoption criteria, prerequisite/pin/upgrade ownership and independent oracle. | [Procedure](../../10-sources/upstream-decisions.md) | Corrected | Pending |
| [F11](../03-extraction-and-verification/f11.md) | Actual pre-erasure candidate hook and checked body/metadata correspondence through rewrites. | [Procedure](../../02-language-independent-extraction/typed-capture-procedure.md) | Corrected | Pending |
| [F12](../03-extraction-and-verification/f12.md) | Static/dynamic/caller-evidence/trusted obligations with asynchronous ownership and mutation control. | [Procedure](../../02-language-independent-extraction/caller-and-host-contracts.md) | Corrected | Pending |
| [F13](../03-extraction-and-verification/f13.md) | Bind each host import to its semantic contract and checked/restricted/permitted-trust implementation. | [Procedure](../../02-language-independent-extraction/caller-and-host-contracts.md) | Corrected | Pending |
| [F14](../03-extraction-and-verification/f14.md) | Interpreter plus independent bindings is mandatory; native host compilation has optional G-HOST-CODEGEN. | [Procedure](../../02-language-independent-extraction/caller-and-host-contracts.md) | Corrected | Pending |
| [F15](../04-backends-and-plugins/f15.md) | Independent compiler and compatible-runtime substitutions, plus the distinct-API installation exercise. | [Procedure](../../03-backend-extension-contract/addition-only-tests.md) | Corrected | Pending |
| [F16](../04-backends-and-plugins/f16.md) | Distinguish recoverable errors/unwinds from process-fatal abort; IPC for application-survival policy. | [Procedure](../../05-runtime-and-interop/submission-and-lifecycle.md) | Corrected | Pending |
| [F17](../04-backends-and-plugins/f17.md) | Pin resources and callbacks to immutable provider generation; drain/fail before unloading. | [Procedure](../../05-runtime-and-interop/submission-and-lifecycle.md) | Corrected | Pending |
| [F18](../04-backends-and-plugins/f18.md) | Protected commit-bound runner/admission policy separate from package manifest and release signing. | [Procedure](../../08-production-acceptance/ci-admission.md) | Corrected | Pending |
| [F19](../05-runtime-and-memory/f19.md) | Session-scoped operation identity, acceptance/dedup/query, unknown outcomes and fail-stop restart policy. | [Procedure](../../05-runtime-and-interop/submission-and-lifecycle.md) | Corrected | Pending |
| [F20](../05-runtime-and-memory/f20.md) | Physical atom-footprint ownership, safe allocator/flush/invalidate policy and noncoherent qualification. | [Procedure](../../05-runtime-and-interop/memory-and-failures.md) | Corrected | Pending |
| [F21](../05-runtime-and-memory/f21.md) | Separate assertions and guards; collective-safe failure and suppression of invalid effects/postconditions. | [Procedure](../../05-runtime-and-interop/memory-and-failures.md) | Corrected | Pending |

Use [resolution data](../../resolutions.json) and [milestones](../../milestones.json) to track implementation. The historical severity remains 13 high and 8 medium. Corrected documentation must never be used as a passed release gate.
