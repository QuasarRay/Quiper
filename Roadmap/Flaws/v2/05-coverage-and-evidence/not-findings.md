# Suspicions checked and not counted as new findings

An aggressive review must reject unsupported accusations as well as find defects. These checks prevent this collection from repeating corrected issues or inflating its count with ordinary implementation work.

| Suspicion | Result |
|---|---|
| A scoped portable release still means complete replacement | V2 separates G-REPLACEMENT and keeps mandatory deferred rows blocking it. No repeat of F01. |
| A package can waive proof obligations by declaring itself trusted | The fixed policy rejects ad hoc trust substitutions. V2-07 concerns representation of a separately admitted policy, not permission for self-approval. |
| Safety or required soak failures can receive performance waivers | The result policy explicitly prohibits that. |
| Late changes can inherit old extension results blindly | Candidate/input closure comparison and affected-gate reruns are now required. |
| The second API is still selected too late | P0 selects it; P7 implements and qualifies it. |
| Paired compiler/runtime installation alone proves independence | V2 now also requires compiler-only and compatible-runtime substitutions. |
| SPIR-T objects must cross a stable ABI for external passes | The compiler protocol keeps them private; external passes need KIR or another specified versioned representation. No such contradiction was established. |
| Direct construction automatically avoids the reported loop bug | V2 explicitly rejects that inference and blocks affected paths pending triage. V2-04 concerns a different documented value convention. |
| ID-bearing annotations and QPtr are now silently assumed supported | V2 restricts the baseline, checks eligibility and qualifies patched profiles separately. |
| The old interpreter PR head is presented as current | The upstream decision page labels it as preserved historical inventory and requires rechecking. |
| Canonical KIR is claimed to be fully specified and implemented already | Grammar, normalization vectors and independent readers are explicit P1 deliverables. Their absence today is not a finding. |
| Missing `legacy-scope.json` and primitive-seed artifacts prove a false implementation claim | The roadmap labels proposed paths/components as future work and assigns the ledger to P0. The audit does not count missing future artifacts as defects. |
| Arbitrary C input receives source verification | Assurance fields explicitly prevent this. Safe execution still needs established memory-safety obligations. |
| The Pulse hook is claimed to be a proven exporter | It is a candidate before erasure, with backward tracing and O1/O2 still required. The source sequence was rechecked. |
| Arbitrary host callbacks inherit refinement verification | The default policy requires checked or enforceably restricted relevant behavior. |
| Native host compilation is mandatory in one place and optional elsewhere | V2 now consistently conditions G-HOST-CODEGEN on offering native compilation. |
| P6 starts proof work only after implementation | Its work starts at P1; P5 is an exit prerequisite. |
| A timeout, worker death or equal numeric epoch establishes success | V2 explicitly rejects all three shortcuts. |
| Noncoherent cache-atom overlap remains ignored | The physical-footprint allocator/flush/invalidate procedure addresses it. |
| Provider disable permits immediate unloading | Active resources retain the exact provider instance; draining/failure precedes unloading. |
| CI manifests can award themselves hardware privileges | Protected admission is separate from package declarations and signing. |
| The schema's digest regex necessarily accepts a trailing newline | The actual pattern was checked in ECMAScript/V8: the tested newline suffix was rejected. No regex defect was established. A different engine needs its own conformance check. |
| A structurally valid result with insufficient proof means the schema is a broken proof checker | The schema explicitly describes a structural envelope; the protected evaluator must validate evidence. Absence of the future evaluator is not counted as a new defect. |
| `not-run` planning gates prove qualification has been claimed | They say the opposite. The documentation checker intentionally protects this snapshot's unexecuted status. |
| A partial-correctness theorem guarantees termination | V2 separates progress/fairness assumptions and exceptional traces. |
| The effort sum is wrong | The phase ranges sum to 56–122 person-weeks and are explicitly unvalidated estimates. |
| Mesa bypass is promised to improve performance | It remains an optional measured experiment with a valid negative outcome. |
| The folder split inherently violates the earlier organization request | All 20 original Markdown paths have matching categorized folders; the checker verifies their mapped files and navigation. |

These dispositions are bounded by the inspected revision. They do not qualify future implementations, prove the unmerged upstream fixes, or exclude further findings from deeper source/hardware work.
