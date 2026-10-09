# 4. Suspicions checked and not counted as findings

**Historical audit record.** This describes the audit snapshot, not the current correction status. See [current resolutions](../README/resolutions.md).

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
