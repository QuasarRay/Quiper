# Isolate Control arithmetic proof search

The clean-checkout source job on commit `675a5a3234cd91ae28305ab93bdf92da5f5d739c` executed all seven Int32 cases and their short-view guards. It then failed while verifying the complete Control module, before exporting `condition_once`. The failing assertion was `U32.v last == 2` in `zero_increment_math`.

[The failed job](https://github.com/QuasarRay/Quiper/actions/runs/38051035421) is the reproduction. Its successful Int32 executions do not establish a successful Control export or a complete source report.

## Preserve the obligations and narrow the context

Wrap only `add_mod_projection`, `zero_increment_math` and `increment_three_math` with:

```fstar
#push-options "--using_facts_from '-* +Prims +FStar.Pervasives +FStar.UInt +FStar.UInt32 +FStar.Math.Lemmas +Kuiper.Portable.Control'"
```

Place `#pop-options` immediately before `ghost fn prove_zero_increment`. Keep the arithmetic statements, Pulse contracts and runtime bodies intact. The three ordinary lemmas use integer arithmetic and UInt32 projection facts; they do not need the unrelated Kuiper/Pulse SMT context. The ghost wrappers and entrypoints retain their ordinary context.

The diagnosis is a proof-search hypothesis until the complete module passes. A successful isolated arithmetic check does not substitute for that check. F* explains context pollution and [namespace filtering](https://fstar-lang.org/tutorial/book/under_the_hood/uth_smt.html#filtering-the-context). The pinned [option parser](https://github.com/FStarLang/FStar/blob/32822af9504e97e560315109c595f3d444583a16/src/basic/FStarC.Options.fst) and [solver context implementation](https://github.com/FStarLang/FStar/blob/32822af9504e97e560315109c595f3d444583a16/src/smtencoding/FStarC.SMTEncoding.SolverState.fst) define the actual option and filtering behavior. Filtering removes background facts; it does not introduce an axiom or discharge an obligation.

## Replay the complete source path

Run `python3 validation/run_source.py --output validation/results/source-vulkan.json` with the coherent pinned compiler and CPU Vulkan environment. Require all eleven execution cases, their eleven view-boundary cases, the installation check and six admission rejections: 29 cases in total. Also require the eleven source boundary tests. Keep the output report absent if a later case fails.

The source workflow can reuse an exact compiler build keyed by the F* gitlink, pinned OPAM package list, workflow, operating system and architecture. It caches only OPAM and compiler/Pulse build directories, saves after a successful compiler build, and supplies no partial restore key. Each run still obtains the required solver, rebuilds and measures the capture plugin, checks the measured compiler and library inputs, and verifies requested source modules in a fresh cache. The toolchain record continues to say that imported library proofs and complete dependency proof closure were not replayed. The cache is a build optimization, not proof evidence.
