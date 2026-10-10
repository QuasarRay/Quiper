# Export without the legacy checked-file cache

## Reproduce the clean-checkout failure

The [first fresh source CI run](https://github.com/QuasarRay/Quiper/actions/runs/38046426604) built the pinned compiler, native Pulse observer and independent Rust packages, then failed before admitting the first entrypoint. The exporter included a missing `obj` directory and passed `--already_cached '*'`. It therefore expected `Kuiper.Divides` to have been checked already. A development checkout's inherited legacy cache had hidden this dependency.

This was a real export failure. A backend fixture passing on software Vulkan could not establish that source export worked from a clean checkout.

## Load project types from source

Remove the `obj` include from the source command. Keep the selected source module in a fresh temporary cache and restrict `--already_cached` to the compiler's measured standard/Pulse library namespaces. Load Kuiper project dependencies from `src` rather than accepting stale project checked files. The independent source workflow must run on a clean checkout and must trigger when project source, contracts, workers, bindings or source validation changes.

The exporter now measures Z3 4.13.3, passes that exact solver with `--smt`, records the six compiler/Pulse source and checked input trees, and binds a name-sensitive digest of project `.fst`/`.fsti` inputs before and after admission. Version-1 private toolchain records reject; rebuild the observer to produce version 2. Changing dependency bytes or relative filenames changes the measured identity. These measurements identify the inputs; they do not authenticate a release or prove its trust closure.

## Preserve the verification boundary

Fresh type loading is not fresh replay of every imported proof. The pinned official [F* option implementation](https://github.com/FStarLang/FStar/blob/0eef57bef411aac090354a75c21e00b674bd420c/src/basic/FStarC.Options.fst#L2188-L2195) restricts verification to explicitly selected modules; the [typechecker environment](https://github.com/FStarLang/FStar/blob/0eef57bef411aac090354a75c21e00b674bd420c/src/typechecker/FStarC.TypeChecker.Env.fst#L261-L266) applies that decision. Automatically loaded Kuiper dependencies and reused standard/Pulse caches do not establish full dependency-proof closure.

Keep `project_dependency_types_loaded_freshly` separate from `dependency_closure_replayed_freshly` and `proof_dependency_closure_checked`. The first records the corrected source-loading path. The latter two remain false. Source-to-KIR refinement, per-lane lifting, transitive proof-bypass classification and candidate-bound production qualification remain open.

## Require an actual regression

Run [source integration](../../../validation/run_source.py) from a checkout with no legacy cache. Check strict source completion, neutral admission and literal evaluator/SPIR-T/Vulkan/C results. Include nested selections, branch-specific stores, stateful conditions, asymmetric early returns and short-view guards. Reject unsupported source, local proof bypasses, a module that fails after live capture and malformed return representations.

A report qualifies only the bytes and tools it records. The earlier successful report remains historical evidence while the expanded regression awaits the current candidate's fresh CI. No source or production gate closes merely because the export command or documentation was corrected.
