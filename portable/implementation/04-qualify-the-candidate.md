# Qualify the exact release candidate

## Keep executed evidence separate from release gates

[Implementation status](../../validation/implementation-status.json) lists the 31 roadmap milestones and all release gates. Partial implementation is recorded as partial. Every release criterion must be checked in full before marking its gate passed; CPU fixture execution cannot close source extraction, full concurrency, numerical coverage, hardware soak or full replacement.

The integration report binds the actual sources, worker binaries, validator and device to executed cases. The ABI report binds eight strict model lemmas to the F* binary and source hashes. Rebuild and replay after relevant source, dependency, toolchain, profile or target changes. A digest identifies content; it is not a proof or authenticated qualification certificate.

## Close the missing implementation work

1. Extend the tested U32/ref source adapter to the full required Kuiper scope. Bind source predicates, static shapes, permissions and the per-lane ownership relation to the package. The three current source entrypoints execute through core and C; their imported proof closure and source refinement remain open.
2. Complete O1–O10, resource framing, memory-event/convergence refinement, prefix ownership and progress. Replace the experimental assurance boundary only when an independent trusted checker accepts the exact evidence.
3. Extend semantics and emission for shared memory, barriers, subgroup operations, general pointers, calls, floating-point policies, reductions, matrices, tensor operations and required legacy entrypoints. Do not defer mandatory legacy rows while claiming full CUDA replacement.
4. Implement production deployment containment, cancellation/quiescence, worker parent-death behavior, crash recovery, bounded asynchronous scheduling and failure injection. Exercise noncoherent paths, partial acquisition/submission, device loss, lost replies and process descendants.
5. Run the roadmap's exact hardware/driver/workload/performance/install/rollback matrix, including real GPUs, numerical edge cases and required soak. Record inconclusive performance as inconclusive. Complete separate additive compiler, runtime, frontend/binding and semantic-operation qualification.

The current host plan interpreter is bounded and synchronous. Native host-plan compilation, callbacks, distributed scheduling and arbitrary event graphs are not implemented. The C binding exercises a second host interface, not a second independently compiled source frontend.

## Admit a release honestly

Use the candidate-bound admission procedure in [M26](../../Roadmap/08-production-acceptance/02-implement-candidate-bound-admission.md). Bind the real release candidate, gate definition versions, policy, tested matrix and proof certificates. Reject self-declared stronger evidence, missing or stale results, unknown required semantics and unqualified targets before submission.

Keep the legacy CUDA path available until every mandatory replacement row qualifies. The new portable build is CUDA-independent; that property alone does not replace existing Kuiper source compilation or its feature catalogue. Publish this branch as an experimental implementation until the remaining gates close.
