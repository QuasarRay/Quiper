# Admit source only after complete checking and bypass inspection

**Severity: high for source evidence. Status: observed local admission flaws corrected; imported proof closure remains open.**

Successful strict verifier completion does not imply that the source contains no explicit proof bypass. This repository's pinned libraries expose admits, assumptions and coercions intentionally. A frontend must classify that trust boundary before erasing ghost syntax or publishing an executable package.

## Match the actual primitive names

The initial filter named `Prims.assume` and a coercion alias, but missed the pinned definitions `Prims._assume`, `Prims.magic`, `Prims.unsafe_coerce`, and Pulse's `assume_`, `stt_admit`, `stt_atomic_admit`, `stt_ghost_admit`. Inspect the actual [Prims source](https://github.com/QuasarRay/FStar/blob/0eef57bef411aac090354a75c21e00b674bd420c/ulib/Prims.fst) and [Pulse Core interface](https://github.com/QuasarRay/FStar/blob/0eef57bef411aac090354a75c21e00b674bd420c/pulse/lib/common/Pulse.Lib.Core.fsti); spelling-based expectations about an API are insufficient.

The corrected native capture and translator reject those resolved identities, explicit Pulse admit/unreachable nodes and source pragmas. The native pure visitor also inspects unsupported pure terms before encoding, so a lambda or let cannot conceal a known bypass. It inspects declared types, binder types/attributes and computation pre/post fields; residual proof hints are rejected rather than partially serialized. Unsupported stateful constructs reject even when their result is ghost-erased.

## Test the meaningful counterexamples

The separate `RejectAssume` and `RejectNested` fixtures type-check to verifier completion when the observer is absent. With the observer, direct `Pulse.Lib.Core.assume_` and nested `Prims.magic` reject before publication. The explicit Pulse admit fixture rejects during capture inspection/translation. These are intentional negative fixtures, never proof evidence for executable kernels.

Ghost slots remain in the de Bruijn environment until translation finishes. A mutation that moves an erased initial value into an executable U32 write argument rejects; unit/ghost elimination must not renumber live variables accidentally. Actual helper calls and explicit returns execute in the positive regression.

## Wait for the entire source process

The live observer can capture an earlier declaration before a later declaration fails. Treating capture-file creation as admission would publish incompletely checked source. The exporter now waits for successful strict compiler completion, validates the selected package through the neutral checker, rechecks measured bytes and publishes the three output files together. The regression appends an ill-typed declaration after the valid captured kernels and requires rejection with no output directory.

Bound the compiler log while it is being produced, not only after exit. Enforce a CPU deadline and terminate the reserved process group before reaping its leader. Test noisy output, nonzero exit after a success-looking message, missing verifier completion, timeout descendants and duplicate JSON fields. These checks constrain the source-export process; they do not authorize killing a GPU process before quiescence.

## Keep the remaining proof boundary explicit

Local syntax inspection is not a transitive analysis of imported proof dependencies. Imported axioms, admitted implementations, specifications and foreign primitives require the classified trust closure in [M04](../../00-current-state-and-gaps/02-classify-the-proof-boundary.md). The current manifest records that closure checking and fresh dependency proof replay are false.

Scalar reference contracts also need an explicit ownership/predicate relation to the parallel per-lane view realization. The official [Kuiper resource definitions](https://github.com/FStarLang/kuiper/tree/413219948f91911ffaf0ac37a5ff941c5d1e55c7) and [F* ghost rules](https://fstar-lang.org/tutorial/book/part4/part4_ghost.html) supply the source semantics; [the declarative refinement obligations](../../Specification/05-establish-spirt-refinement.md) specify the missing implementation relation. Continue to use the experimental policy until those relations and candidate-bound checks are established.
