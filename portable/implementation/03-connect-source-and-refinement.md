# Connect checked source and close refinement

## Capture the checked Pulse body

The executable path currently starts from KIR. The source frontend is unfinished. Do not describe a JSON fixture as a verified Kuiper export, and do not use textual matching over source as typed capture.

The pinned compiler already saves the required pre-erasure body. [`Pulse.Main.fst`](https://github.com/QuasarRay/FStar/blob/0eef57bef411aac090354a75c21e00b674bd420c/pulse/src/checker/Pulse.Main.fst#L109-L150) calls `check_abs`, embeds the checked `st_term`, deeply compresses it and saves a `"pulse"` blob before selecting the extraction implementation. [`FStarC.Tactics.Hooks.fst`](https://github.com/QuasarRay/FStar/blob/0eef57bef411aac090354a75c21e00b674bd420c/src/tactics/FStarC.Tactics.Hooks.fst#L550-L569) stores extension data in signature metadata. [`Pulse_RuntimeUtils.ml`](https://github.com/QuasarRay/FStar/blob/0eef57bef411aac090354a75c21e00b674bd420c/pulse/src/ml/Pulse_RuntimeUtils.ml#L73-L80) identifies the lazy typed-body representation.

Use that blob through a plugin built against the exact compiler/Pulse interfaces. The relevant callback registration is `FStarC.Extraction.ML.Modul.register_extension_extractor`; its signature and implementation are in the pinned compiler. Preserve or explicitly delegate the existing Pulse extraction callback when installing a capture callback. Reject missing metadata, unexpected lazy tags, incompatible compiler hashes and unknown syntax constructors. Never infer checked status from a `.checked` filename or an exporter-provided string.

The nightly package bundles a working verifier and CUDA plugin, but omits Pulse checker interfaces needed by a new native exporter. Its OCaml interfaces also have a different Stdlib CRC from the independently installed OCaml 5.3 toolchain tested here. A compiler-version string is therefore insufficient to establish native plugin compatibility. Build and package the compiler, Pulse and exporter coherently, preserving their interface metadata and dependencies. Bootstrap extraction may be lax; replay user/source verification strictly and record that boundary separately.

The official [Pulse extraction tutorial](https://fstar-lang.org/tutorial/book/pulse/pulse_extraction.html) describes AST extraction through `--codegen Extension`. That extraction facility establishes where to integrate; it does not prove a new Kuiper-to-KIR translation correct.

## Emit executable KIR with source evidence

Traverse typed stateful and pure terms, including binders, qualifiers, effectful applications, branches, locals and loop condition/body terms. Resolve primitive identities and their transitive closures before lowering. Preserve source ranges, resource location/access, scalar widths, initialization, frame facts, divergence and explicit numerical policy.

Start with one actual integer Kuiper kernel and host wrapper. Export it to the same KIR schema used by the independent fixtures. Run the KIR evaluator and both host interfaces without teaching the core about Pulse. Then compile and execute that exact package through the installed workers. Bind source/module/primitive/toolchain hashes, checked options and correspondence to the emitted package. Unsupported primitives must stop export before an executable artifact is published.

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
