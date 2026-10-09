# Implement typed capture and erasure correspondence

## 1. Locate the checked boundary

Read the official [F* erasure documentation](https://fstar-lang.org/tutorial/book/part4/part4_ghost.html) and [Pulse ghost-computation documentation](https://fstar-lang.org/tutorial/book/pulse/pulse_ghost.html). They distinguish computational data from erased proof/specification content. They do not imply that every implicit argument is erased or that an erased value can be recovered at runtime.

In the pinned project F* fork, inspect [Pulse.Extract.Main](https://github.com/QuasarRay/FStar/blob/0eef57bef411aac090354a75c21e00b674bd420c/pulse/src/checker/Pulse.Extract.Main.fst#L582-L592). `extract_pulse_dv` erases ghost subterms, simplifies, eliminates gotos, and converts the result. Instrument immediately before the first erasure, then trace backwards to the checker call that supplies the term/environment. Confirm the term has passed the intended source checks; a syntactically typed reflection term alone is not a source-proof certificate.

Implement a frontend-private capture record. For each declaration, retain the checked type/effect, source identity, static instantiation, resource/layout facts, numeric policy, proof dependency IDs, and intended executable symbols. Do not export F* heap objects. Translate facts into KIR's specified propositions/constraints or reference separately checked proof artifacts.

## 2. Classify every value before erasure

1. Executable values become KIR values with explicit runtime representations.
2. Static specialization values become canonical package inputs used to produce concrete layouts, workgroup dimensions, and signatures. Their chosen values and justification remain in compilation identity.
3. Proof-only values become evidence references or are erased after the preservation relation says they cannot affect executable meaning.

An erased index used to justify a runtime shape needs a relation to a retained concrete dimension; it cannot be restored by parsing a generated name. Preserve that relation while specializing the function. Reject unbound static parameters before emitting the body.

Start with a real array/view entrypoint containing an erased index, a runtime dimension, a helper call, and a loop. Trace checked Pulse → captured manifest → erased executable form → KIR. Compare all offsets, extents and signatures with the existing extraction for diagnosis, and independently with the source specification for correctness.

## 3. Bind facts to rewritten code

Assign source node IDs before transformation, but treat IDs as provenance only. Maintain a checked relation for inlining, monomorphization, unit/ghost elimination, branch simplification, goto elimination, and resource lowering. Record which source nodes justify each result node, including duplicated or merged operations. A matching function name or digest is not a preservation proof.

Implement a paired manifest/body checker that verifies complete symbol resolution, specialization consistency, layout/effect agreement, and evidence coverage. Mutate one shape, permission, branch effect, or source mapping and require rejection. O1/O2 must describe the actual accepted subset and the actual transformation chain.

## 4. Reuse extraction infrastructure deliberately

Study the existing [Kuiper handlers](https://github.com/FStarLang/kuiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/extraction/ExtractKuiper.fst) and [Pulse2Rust interface](https://github.com/QuasarRay/FStar/blob/0eef57bef411aac090354a75c21e00b674bd420c/pulse/pulse2rust/src/Pulse2Rust.Extract.fsti). Reuse traversal, symbol handling and diagnostics where their assumptions fit. Isolate language-specific constructors in the F* adapter. The final exporter must omit Karamel and generated CUDA text from its mandatory path.

A temporary Karamel importer is comparison scaffolding, with a documented removal gate. If a pre-erasure hook requires a fork change, implement it before interface freeze, expose a versioned frontend-private API, and add a regression on the pinned source. Do not modify generic backend contracts to expose F* internals.

## 5. Exit evidence

G-EXTRACT requires the real-entrypoint trace, all three value classes, deterministic KIR output, rejection fixtures, strict source-check provenance, and the O1/O2 evidence disposition under the selected policy. Source inspection in this roadmap identifies a candidate hook. Building the hook and establishing its correctness remain required implementation work.
