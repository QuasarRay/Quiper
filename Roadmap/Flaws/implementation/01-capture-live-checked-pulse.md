# Capture live checked Pulse before compression

**Severity: high for source-export feasibility. Status: observed implementation flaw corrected; source refinement remains open.**

The first implementation instructions said the pinned compiler's saved `"pulse"` blob contained the typed pre-erasure body and could be consumed by a later ML extraction callback. That inference was wrong. It confused the embedding location with preservation of the embedded value.

## Establish the actual failure

At baseline commit `0eef57bef411aac090354a75c21e00b674bd420c`, `Pulse.Main` calls `check_abs`, embeds the checked stateful term with the lazy tag `pulse_st_term`, then deep-compresses it before saving metadata. The syntax visitor unfolds lazy nodes; the hook's extension case supplies a display placeholder. A real module checked strictly by that compiler produced this metadata probe:

```text
FUNCTION Kuiper.Portable.Int32.add_one
BLOB "((extension pulse_st_term))"
NODE Tm_constant
Failure("Not a Tm_lazy of the expected kind")
```

The checked file exists and source checking succeeded, but the lazy typed body is absent. Adding native interfaces to a nightly package would not fix this loss. The same investigation also found that the relevant ML extractor registration declaration can be absent from Custard's compiled interface when it is not retained as an entrypoint; a source signature alone does not establish native API availability.

## Implement the correct boundary

The [compiler patch](https://github.com/QuasarRay/FStar/commit/32822af9504e97e560315109c595f3d444583a16) adds a frontend-private observer after checked embedding and before deep compression. Its default is a no-op. The native adapter registers one observer and serializes the live Pulse body. The existing metadata and ordinary extraction paths retain their original behavior.

Build the compiler, Pulse and plugin together from the same stage3 artifacts. Do not mix nightly/independently built Stdlib interfaces, or stage2 compiler/stage3 Pulse interfaces. Both mismatches were observed during the spike. Record binary, interface, plugin and hook-source hashes. Treat a compiler version string as diagnostic metadata, not native compatibility evidence.

Follow [the source frontend build/export procedure](../../../frontend/kuiper/README.md). Export actual source, then validate its exact canonical KIR through the independent neutral checker before publishing anything. Keep F*/Pulse syntax private to the frontend; do not add it to the backend contract.

## Verify the correction without overclaiming

[The source harness](../../../validation/run_source.py) now exercises live checked source, erased witnesses, helper calls and explicit Label/Goto returns. Its packages execute through the evaluator, installed SPIR-T/Vulkan workers, core and C on software Vulkan. Native plugin loading confirms coherent interface compatibility for the measured build.

The official [Pulse extraction documentation](https://fstar-lang.org/tutorial/book/pulse/pulse_extraction.html) describes the extension extraction interface; the official [F* ghost rules](https://fstar-lang.org/tutorial/book/part4/part4_ghost.html) explain erasure. They justify investigating the typed boundary, not assuming this adapter preserves source semantics. Complete [M08's real array/view/static-shape/loop trace](../../02-language-independent-extraction/02-export-a-checked-kuiper-program.md) and O1/O2 separately. The new hook's successful regression establishes access to the live body, not the full source refinement theorem.
