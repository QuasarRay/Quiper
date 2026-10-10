# Connect checked source and close refinement

## Capture the checked Pulse body

The [closed U32/ref frontend](../../frontend/kuiper/README.md) now exports actual checked Pulse into KIR. General source coverage and implementation refinement remain unfinished. Do not describe a hand-written JSON fixture as a verified Kuiper export, and do not use textual matching over source as typed capture.

The earlier instruction to read the saved `"pulse"` blob was incorrect. At the baseline compiler commit, [`Pulse.Main.fst`](https://github.com/QuasarRay/FStar/blob/0eef57bef411aac090354a75c21e00b674bd420c/pulse/src/checker/Pulse.Main.fst#L109-L150) embeds the checked `st_term`, deeply compresses it and saves extension metadata. Compression traverses and unfolds the extension lazy node. A real strict-source probe then reads the constant string `"((extension pulse_st_term))"` from checked-file metadata, rather than the typed body; `unembed_st_term_for_extraction` rejects it. The presence of the blob does not establish body preservation.

Use the [live observer patch](https://github.com/QuasarRay/FStar/commit/32822af9504e97e560315109c595f3d444583a16) immediately after `check_abs` and embedding, **before** deep compression and extraction. Register the private native observer from the source adapter, unembed that live term and serialize its typed body. Reject unexpected representations, incompatible compiler hashes and unsupported constructs. Publish nothing until the complete strict source process and neutral admission both succeed. A declaration may have been captured before a later declaration fails. Never infer checked status from a `.checked` filename or an exporter-provided string.

The nightly package bundles a working verifier and CUDA plugin, but omits Pulse checker interfaces needed by a new native exporter. Its OCaml interfaces also have a different Stdlib CRC from the independently installed OCaml 5.3 toolchain tested here. Stage2 compiler and stage3 Pulse interfaces can also disagree. Build compiler, Pulse and exporter coherently, using the stage3 interfaces and binary together. Preserve their measured bytes and dependencies. Bootstrap extraction may be lax; replay user/source verification strictly and record that boundary separately.

The official [Pulse extraction tutorial](https://fstar-lang.org/tutorial/book/pulse/pulse_extraction.html) describes AST extraction through `--codegen Extension`. That extraction facility establishes where to integrate; it does not prove a new Kuiper-to-KIR translation correct.

Export project dependency types from source in a fresh cache without the legacy `obj` directory. Measure the solver, standard/Pulse source and checked trees, and name-sensitive project source inputs before and after admission. Imported type loading is not fresh proof replay. Follow [the clean-checkout finding](../../Roadmap/Flaws/implementation/03-export-without-a-legacy-cache.md) and preserve the manifest's false closure flags until that separate obligation is proved.

## Emit executable KIR with source evidence

Traverse typed stateful and pure terms, including binders, qualifiers, effectful applications, branches, locals and loop condition/body terms. Resolve primitive identities and their transitive closures before lowering. Preserve source ranges, resource location/access, scalar widths, initialization, frame facts, divergence and explicit numerical policy.

The current [source integration](../../validation/run_source.py) exercises actual U32/ref source through the evaluator, installed SPIR-T/Vulkan workers, core and C. The adapter now lowers structured conditionals through typed region results and active return continuations, with checked helper qualifiers. Extend the regression across branch-specific memory, nested selections, stateful conditions and asymmetric early returns; literal outputs and preserved padding must agree across all interfaces. [The report](../../validation/results/source-vulkan.json) binds its measured source, toolchain and package identities; an earlier report does not qualify a newer candidate. The parallel mapping from scalar `gpu_ref` contracts to per-lane views, source predicate correspondence and imported proof closure remain unproved. Do not treat experimental evidence as O1/O2 or G-EXTRACT closure.

Do not silently narrow Kuiper's SizeT/U64 indices into the U32 profile, erase resource permissions or replace a source while with a do-while. Every widening, narrowing, effect erasure, loop-carried local and source numerical operation needs an explicit semantic mapping.

## Discharge the specification's obligations

Replay all eleven declarative F* modules with a fresh cache. Replay [CheckedViews](../proofs/Kuiper.Portable.CheckedViews.fst) separately. Its eight lemmas establish checked-address containment, nonwrapping address representation, invalid-index rejection, disjoint index addresses, store framing and Boolean/signed-word representation facts.

```sh
python3 Roadmap/tools/verify_spec.py --fstar /path/to/fstar.exe --report validation/results/declarative-model.json
python3 validation/verify_bridges.py --fstar /path/to/fstar.exe --report validation/results/abi-bridge.json
```

These are model theorems. They do not prove that Rust parsing, the evaluator, SPIR-T lowering, target reflection or Vulkan execution implements the model. Follow [the refinement milestone](../../Roadmap/Specification/05-establish-spirt-refinement.md) and [proof replay](../../Roadmap/Specification/06-replay-proofs-and-evidence.md) to establish O1–O10, prefix ownership and progress over actual implementation transitions.

The official F* documents distinguish [pure total computations](https://fstar-lang.org/tutorial/book/part4/part4_computation_types_and_tot.html), [lemma refinements](https://fstar-lang.org/tutorial/book/part4/part4_pure.html) and [erased ghost computations](https://fstar-lang.org/tutorial/book/part4/part4_ghost.html). Use those rules to prove the source-to-KIR and KIR-to-target relations; a model containing a symbolic relation is not an implementation proof of that relation.

The official [Kuiper repository](https://github.com/FStarLang/kuiper/tree/413219948f91911ffaf0ac37a5ff941c5d1e55c7) defines the legacy scope, including shared memory, barriers, vector operations and tensor cores. Those features remain outside this integer implementation. Their declarative extension predicates must acquire concrete semantics and implementation proofs before claiming replacement coverage.
