# Export checked Kuiper source into the integer profile

This frontend observes live checked Pulse, translates a closed U32/ref profile into KIR, and runs the neutral contract checker before publishing a package. It does not parse generated CUDA or Karamel output. Installing SPIR-T and Vulkan workers still adds files without changing the core.

The exported package uses `kuiper.experimental-tested/1`. Source-to-KIR refinement, per-lane lifting, source predicate correspondence, and imported proof dependency closure remain unproved. A successful source check and matching hashes do not establish those properties.

## Build one coherent native toolchain

Use the F* submodule commit recorded by this checkout. The live observer is implemented in [the pinned compiler patch](https://github.com/QuasarRay/FStar/commit/32822af9504e97e560315109c595f3d444583a16). It observes `check_abs`'s result before `deep_compress` unfolds extension lazy nodes and before ghost erasure. The callback is frontend-private; the neutral contracts do not depend on F* or Pulse.

Follow the official [F* installation instructions](https://github.com/FStarLang/FStar/blob/master/INSTALL.md) for OCaml, OPAM dependencies and Z3. The tested compiler is OCaml 5.3.0. All plugin interfaces must come from the same **stage3** build. Stage2 compiler interfaces and stage3 Pulse interfaces are not interchangeable. A matching OCaml version string alone does not establish compatibility.

From the repository root, in that OPAM environment:

```sh
git submodule update --init --recursive
opam install --deps-only ./FStar/fstar.opam --yes --jobs=2
make -j2 -C FStar 3
python3 frontend/kuiper/build_capture.py
make -j2 portable-release
```

For the tested dependency versions, create an OCaml 5.3.0 switch and install `opam-packages.txt` with `xargs -a frontend/kuiper/opam-packages.txt opam install --yes --jobs=2`. This records the tested package versions; archive/toolchain authentication and complete reproducible release packaging remain qualification work. [The source CI workflow](../../.github/workflows/portable-source.yml) builds that coherent toolchain and runs the real source regression independently of the frontend-free backend workflow.

F*'s own build currently includes Karamel in its toolchain build dependencies. The **source export operation** below invokes neither Karamel nor CUDA extraction. The tested local build reused a compatible `inst/bin/krml` with `FSTAR_USE_KRML_EXE=1`; that is a bootstrap choice, not a backend dependency. Bootstrap extraction uses the compiler's normal bootstrap trust boundary. Entry source checking runs strictly.

The version-2 private toolchain record in `_build/toolchain.json` identifies the compiler, source commit, three relevant interface hashes, observer sources, plugin, Z3 4.13.3 and six source/checked library trees. A binary built before the Git checkpoint may print the baseline commit with `-dirty`; its measured bytes and hook source hashes remain explicit in the record. Do not replace it by an unmeasured nightly executable. Version-1 records reject and must be rebuilt. The nightly package used during development omitted native Pulse interfaces and had incompatible OCaml Stdlib interface checksums.

## Export a source entrypoint

Keep the qualified F* module name in the source filename. Put `z3-4.13.3` on `PATH`. Use a fresh output directory:

```sh
python3 frontend/kuiper/export_source.py \
  frontend/kuiper/examples/Kuiper.Portable.Int32.fst \
  --entry increment --local-size 4 --output /tmp/kuiper-increment
```

The exporter checks the selected module in a fresh cache, captures its checked declarations, translates the selected helper closure, and asks `kuiper-source-check` to validate and hash the canonical package. It publishes `package.json`, `source.json` and `capture.json` together only after successful compiler completion and neutral admission. It loads project dependency types from source without the legacy `obj` cache. F* verifies explicit roots; automatically loaded imports and the measured standard/Pulse caches do **not** constitute fresh dependency proof replay. Both proof-closure flags remain false.

The source/compiler/plugin/checker/frontend/solver bytes, library trees and name-sensitive project source identity are checked before and after admission. The measured solver is passed explicitly with `--smt`. Compiler output, source size, capture size, declaration count, syntax depth, instruction count and helper depth are bounded. The CPU compiler's process group is terminated before its leader is reaped, including on failure or timeout. This deadline does not apply to a submitted GPU runtime process.

`source-options.json` supplies the frontend's versioned verification options directly. Export does not invoke the legacy make/configure path, which can probe NVCC on machines that have CUDA installed. The option file's bytes are bound into each source admission record.

## Use the accepted semantics precisely

| Source form | Integer realization |
|---|---|
| Explicit `gpu_ref FStar.UInt32.t` argument | One owned U32 view, accessed at `GlobalInvocationID.x` |
| Explicit `FStar.UInt32.t` argument | Uniform U32 parameter |
| U32 literal constructor | Exact 32-bit constant; no implicit narrowing |
| `add_mod`, `sub_mod`, `mul_mod` | Wrapping U32 arithmetic |
| `div`, `rem` | U32 quotient/remainder; a zero divisor guards without publishing buffers |
| `logand`, `logor`, `logxor` | Exact 32-bit word operations |
| `lognot` | XOR with typed U32 all-ones constant |
| `shift_left`, `shift_right` | 32-bit shifts; counts at least 32 guard without publishing buffers |
| `eq`, `lt`, `lte` | U32 comparison producing Bool |
| `Kuiper.Ref.read` / `write` | Guarded per-lane load/store; access mode derived from actual uses |
| Module-local helpers | Bounded inlining with checked parameter representations and argument qualifiers; only known erased scalar implicit binders may erase |
| Erased U32/Bool/Unit arguments and generated ghost witnesses | Proof slots retained during de Bruijn traversal and barred from executable words |
| Scoped labels and explicit returns | Checked return representation and resolution through the active label continuation |
| Pulse `if` | One condition evaluation followed by one selected arm; isolated regions with matching U32/Bool/Unit result joins |
| Early return from one or both arms | Invoke the target label continuation; skip intervening statements in the returning arm while the other arm continues |

The examples exercise helper calls, preserved input ownership, uniform parameters, wrapping arithmetic, generated erased witnesses, Label/Goto returns, branch-dependent values and memory effects, and nested selections. A stateful condition runs once before the choice. Each arm is translated through its remaining continuation, so a return can skip the statements after an `if` in that arm. Branch-local values reach their enclosing region through declared result tuples. Continuations can duplicate instruction sequences across arms; the existing instruction and neutral-region depth budgets still apply. The [official Pulse conditional tutorial](https://fstar-lang.org/tutorial/book/pulse/pulse_conditionals.html) defines the source constructs and postcondition style; [the pinned SPIR-T region API](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/lib.rs#L779-L920) defines the backend region outputs. These are the source/API basis, not an implementation-refinement proof.

The [pinned UInt32 contracts](https://github.com/FStarLang/FStar/blob/32822af9504e97e560315109c595f3d444583a16/ulib/FStar.UInt32.fsti) define nonzero divisors, shift-count bounds and bitwise results. The complement rewrite is supported by [UInt.logxor_lemma_2](https://github.com/FStarLang/FStar/blob/32822af9504e97e560315109c595f3d444583a16/ulib/FStar.UInt.fsti#L378-L379). The Operations module gives each primitive an exact source postcondition; the regression uses independently calculated integer literals over zero, small values, the signed boundary and unsigned maxima. These source contracts and tests do not prove the adapter's implementation correspondence.

Replicating a scalar reference contract across a view is an **experimental lifting convention**. The adapter has not proved that the source heap, permissions and postcondition relate to the parallel invocation. Resource roles and guard checks establish concrete KIR behavior, not that missing lifting proof.

U64/SizeT, symbolic fractional permissions, arrays/shapes, static layout specialization, loops, locals, residual proof hints, shared memory, barriers, subgroup operations, floating-point policies and tensor operations are outside this source profile. Unsupported executable constructs stop admission. Known local proof bypasses are rejected before erasure, including the pinned library's actual assume/admit/magic/coercion symbols and bypasses inside pure lambdas. A compiler-generated unreachable tail is accepted only when structural jump dominance makes it dead; any unreachable node encountered by executable lowering still rejects. Explicit bypasses remain rejected even inside dead tails. This syntax inspection does not analyze imported symbols' transitive proof closure.

## Execute and replay the evidence

Install the independent workers using [the portable implementation instructions](../../portable/implementation/01-install-and-execute.md). Run the admitted package through the core or C binding. For the complete source regression:

```sh
python3 validation/run_source.py --output validation/results/source-vulkan.json
python3 -m unittest discover -s validation -p test_source_frontend.py -v
```

The integration harness checks actual source entrypoints through strict source admission, the independent evaluator, SPIR-T, installed Vulkan and C. It checks literal results, unsigned division/remainder, masks/complement, shifts, unsigned/equality boundaries, wrapping, branch-specific memory, conditions with side effects, early-return continuations and preserved padding. An executed out-of-view access guards without publishing failed buffers; a selected early return that skips every access succeeds with the shorter view. It rejects U64/fractional/admitted/assumed/nested-bypass source and a module that fails after live capture. Mutation tests reject malformed label targets, mismatched return representations and attempts to use erased input as a runtime word. [The execution report](../../validation/results/source-vulkan.json) records its own successful case and entrypoint counts and source hashes; an older report does not qualify this newer candidate. The expanded matrix requires 45 execution/admission cases covering 17 distinct entries: 19 execution cases, 19 view-boundary cases, installation and six source rejections. Its whole-module checks must pass before claiming this expanded source coverage. The committed report records software Vulkan. [Physical replay](../../portable/implementation/05-replay-on-physical-vulkan.md) consumes the successful current-candidate report's exact packages and all 38 execution/view vectors, verifies checkout and source identities, and replays them through core and C. That is a 39-case hardware matrix including installation. Its workflow and boundary tests do not constitute a successful physical run; the actual job must pass. Physical GPU qualification remains open.

The checked-body fixture in `validation/fixtures` is a mutation regression input. It is not a proof certificate. Regenerate it only through a successful `validation/run_source.py --fixture-output validation/fixtures/source-increment-capture.json` run.

## Close the proof boundary before stronger admission

The official [Pulse extraction tutorial](https://fstar-lang.org/tutorial/book/pulse/pulse_extraction.html) describes AST extraction and extension code generation. The official [F* ghost documentation](https://fstar-lang.org/tutorial/book/part4/part4_ghost.html) explains erasure. Neither proves this new translation. The official [Kuiper code](https://github.com/FStarLang/kuiper/tree/413219948f91911ffaf0ac37a5ff941c5d1e55c7) defines the source resource and operation scope. The pinned [SPIR-T documentation](https://github.com/EmbarkStudios/spirt/tree/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3) supplies the backend API; SPIR-V validation alone does not prove preservation.

Complete [M08](../../Roadmap/02-language-independent-extraction/02-export-a-checked-kuiper-program.md), bind source predicates/static facts to the executable package, and discharge the [declarative refinement obligations](../../Roadmap/Specification/05-establish-spirt-refinement.md) over the actual implementation. Continue to reject stronger assurance policies until independent candidate-bound evidence satisfies their full requirements.
