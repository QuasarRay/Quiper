# Export a checked Kuiper program before erasure

**Milestone M08.** A real entrypoint exported with typed facts, executable KIR and independently bound proof evidence.

## Required inputs and specification

Start from [M07](01-implement-the-neutral-language.md). Use the exit-gate dependencies in [the implementation plan](../milestones.json); proof and investigation work may begin earlier. Read [the declarative specification](../Specification/README.md) before choosing representation details. This milestone implements `Refinement.obligations`, `Operations.catalog_covers`.

## Extract before essential information disappears

The current extractor observes a lowered F* ML representation. Comments around fragment extraction already document lost indices. A late adapter cannot reconstruct every erased layout or matrix parameter reliably. [Q2](../10-sources/README.md)

Implement the F* adapter in two coordinated steps:

1. At the typed boundary where runtime-relevant indices, effects, and proof obligations are available, collect a neutral semantic manifest. Classify each value as executable, static specialization data, or proof-only.
2. Extract executable bodies into KIR and bind them to that manifest using deterministic symbol/operation IDs. Reject missing or inconsistent static information instead of recovering it from generated names or CUDA text.

Start the P0 hook investigation immediately before `Pulse.Extract.Main.extract_pulse_dv` calls `erase_ghost_subterms` in the pinned F* fork. Trace the checked term and environment rather than assuming the exported ML AST retains these facts. The existing Karamel extension hook is evidence of extensibility, not proof that the required pre-erasure information is available at that hook. Make any required frontend-hook changes once, before contract freeze, and keep them confined to the F* adapter.

A temporary Karamel-IR importer may accelerate comparison tests, but it cannot be the permanent public interface if it requires CUDA names or loses necessary semantics. The production extraction path must be able to omit Karamel entirely.

## Locate the checked boundary

The [implemented source spike](../../frontend/kuiper/README.md) now captures live checked Pulse in the [pinned observer patch](https://github.com/QuasarRay/FStar/commit/32822af9504e97e560315109c595f3d444583a16), before deep compression as well as erasure. Saved checked-file metadata is insufficient: the baseline compressor unfolds the embedded extension node into a display string. Follow [the implementation findings](../Flaws/implementation/01-capture-live-checked-pulse.md) for the observed failure and regression. The closed U32/ref spike includes structured selections and scoped early-return continuations. Its current source regression remains experimentally admitted; it does not close this milestone's shape/loop/static-value and refinement requirements.

Read the official [F* erasure documentation](https://fstar-lang.org/tutorial/book/part4/part4_ghost.html) and [Pulse ghost-computation documentation](https://fstar-lang.org/tutorial/book/pulse/pulse_ghost.html). They distinguish computational data from erased proof/specification content. They do not imply that every implicit argument is erased or that an erased value can be recovered at runtime.

In the pinned project F* fork, inspect [Pulse.Extract.Main](https://github.com/QuasarRay/FStar/blob/0eef57bef411aac090354a75c21e00b674bd420c/pulse/src/checker/Pulse.Extract.Main.fst#L582-L592). `extract_pulse_dv` erases ghost subterms, simplifies, eliminates gotos, and converts the result. Instrument immediately before the first erasure, then trace backwards to the checker call that supplies the term/environment. Confirm the term has passed the intended source checks; a syntactically typed reflection term alone is not a source-proof certificate.

Implement a frontend-private capture record. For each declaration, retain the checked type/effect, source identity, static instantiation, resource/layout facts, numeric policy, proof dependency IDs, and intended executable symbols. Do not export F* heap objects. Translate facts into KIR's specified propositions/constraints or reference separately checked proof artifacts.

## Classify every value before erasure

1. Executable values become KIR values with explicit runtime representations.
2. Static specialization values become canonical package inputs used to produce concrete layouts, workgroup dimensions, and signatures. Their chosen values and justification remain in compilation identity.
3. Proof-only values become evidence references or are erased after the preservation relation says they cannot affect executable meaning.

An erased index used to justify a runtime shape needs a relation to a retained concrete dimension; it cannot be restored by parsing a generated name. Preserve that relation while specializing the function. Reject unbound static parameters before emitting the body.

Start with a real array/view entrypoint containing an erased index, a runtime dimension, a helper call, and a loop. Trace checked Pulse → captured manifest → erased executable form → KIR. Compare all offsets, extents and signatures with the existing extraction for diagnosis, and independently with the source specification for correctness.

## Bind facts to rewritten code

Assign source node IDs before transformation, but treat IDs as provenance only. Maintain a checked relation for inlining, monomorphization, unit/ghost elimination, branch simplification, goto elimination, and resource lowering. Record which source nodes justify each result node, including duplicated or merged operations. A matching function name or digest is not a preservation proof.

Implement a paired manifest/body checker that verifies complete symbol resolution, specialization consistency, layout/effect agreement, and evidence coverage. Mutate one shape, permission, branch effect, or source mapping and require rejection. O1/O2 must describe the actual accepted subset and the actual transformation chain.

## Reuse extraction infrastructure deliberately

Study the existing [Kuiper handlers](https://github.com/FStarLang/kuiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/extraction/ExtractKuiper.fst) and [Pulse2Rust interface](https://github.com/QuasarRay/FStar/blob/0eef57bef411aac090354a75c21e00b674bd420c/pulse/pulse2rust/src/Pulse2Rust.Extract.fsti). Reuse traversal, symbol handling and diagnostics where their assumptions fit. Isolate language-specific constructors in the F* adapter. The final exporter must omit Karamel and generated CUDA text from its mandatory path.

A temporary Karamel importer is comparison scaffolding, with a documented removal gate. If a pre-erasure hook requires a fork change, implement it before interface freeze, expose a versioned frontend-private API, and add a regression on the pinned source. Do not modify generic backend contracts to expose F* internals.

## Exit evidence

G-EXTRACT requires the real-entrypoint trace, all three value classes, deterministic KIR output, rejection fixtures, strict source-check provenance, and the O1/O2 evidence disposition under the selected policy. The current live observer and structured U32/ref regression establish a tested hook and bounded execution slice. The required array/view entrypoint, bound static dimensions/layouts, loop trace, dependency proof closure and source implementation relation remain open.

## Evidence required to close this milestone

Close **G-EXTRACT** only with the implementation artifacts, positive and negative cases, and source-to-result identities required above. Link the implementation relation to the named F* symbols and the policy's applicable O1–O10 obligations. The specification's proved lemmas are reusable model facts; they do not discharge this implementation correspondence. Record unresolved cases as blockers or explicitly outside the claim. No backend gate is marked passed by this roadmap revision.
