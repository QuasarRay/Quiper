# Freeze the complete legacy semantic inventory

**Milestone M03.** A reproducible entrypoint and transitive primitive ledger, including foreign code and generated instantiations.

## Required inputs and specification

Start from [M02](../README/02-sequence-the-implementation.md). Use the exit-gate dependencies in [the implementation plan](../milestones.json); proof and investigation work may begin earlier. Read [the declarative specification](../Specification/README.md) before choosing representation details. This milestone implements `Operations.catalog_covers`, `Qualification.full_replacement`.

## Audited baseline

| Component | Revision inspected | Scope |
|---|---|---|
| Quiper / Kuiper | `413219948f91911ffaf0ac37a5ff941c5d1e55c7`, `main` | Source tree, extraction implementation, key proof interfaces, runtime headers, build and CI definitions |
| SPIR-T | `e8757adba8d14068a7bf1b3bc9f24cac982f4bd3`, `main` | IR definitions, SPIR-V bridge, pointer legalization, examples, manifest, and documentation |
| F* submodule | `0eef57bef411aac090354a75c21e00b674bd420c` | Gitlink and selected Pulse erasure/extraction interfaces inspected; no full compiler audit |
| Karamel submodule | `75bc9443b430f5161d85ff02eedb385e9a6db607` | Gitlink recorded; full submodule implementation was not audited |

At the original roadmap inspection, no open pull requests were returned for QuasarRay/Quiper. The later audit was published as PR #1. This revision incorporates its 21 findings and selected upstream SPIR-T work; see [the dependency decisions](../10-sources/01-lock-and-reproduce-primary-sources.md). The old inventory is a dated snapshot, not a claim about today's PR count. No F* rebuild, GPU execution, or end-to-end proof was performed for this documentation change.

Repository instructions were read from `AGENTS.md`, including the referenced kernel guidance. Future builds must use parallel Make invocations. Release evidence must not inherit development proof bypasses.

The checked-out tree contains 430 files and 96,626 raw lines under `src/`, including interfaces and generators; `src/lib/kuiper/` accounts for 120 files and 15,969 raw lines. `extraction/` contains 1,677 raw lines across nine files. `dist/` contains 67,304 raw lines across 147 generated/distribution files. These are descriptive filesystem counts, not logical SLOC, proof complexity, or an effort estimate. A rewrite of a historical “10k core” would not cover the current migration surface.

## Existing pipeline

F*/Pulse modules are verified, extracted through `extraction/ExtractKuiper.fst` to Karamel's representation, translated to CUDA, rewritten by `scripts/fixup.sed`, and compiled through `nvcc.mk`. The extraction plugin is written in F* against compiler internals and built as an OCaml plugin. The host side is also affected: allocation, copies, streams, synchronization, and launch macros are emitted directly. [Q1–Q4](../10-sources/README.md)

The tree uses `src/klas/` for the extracted library instantiations selected by `nvcc.mk`; migration work must follow the actual tree rather than assuming the `src/lib/inst/` path mentioned in agent guidance exists.
## Coupling that must be removed

| Existing location | Observed dependency | Required destination |
|---|---|---|
| `extraction/ExtractKuiper.fst` | F* compiler AST, Karamel expression constructors, CUDA names, launch construction, primitive name matching | F* adapter plus neutral operation catalog; CUDA mappings isolated in legacy backend |
| `extraction/ExtractionUtils.fst` | ML/Karamel-oriented extraction helpers | Reuse only inside the F* adapter; expose neutral data outside it |
| `verify.mk` | `--codegen krml`, `-cuda`, generated `.cu/.h`, plugin and formatter dependencies | Generic extraction/compilation orchestration plus separate legacy rules |
| `include/kuiper.h` | CUDA headers, allocation, launch syntax, stream creation, error termination | Backend runtime and host compatibility shim |
| `include/kuiper/atomics.h` | CUDA atomics and a target-specific fallback | Explicit atomic operations and target-specific implementations |
| `include/kuiper/tensorcores.h`, `wgmma.h` | NVIDIA fragment and instruction behavior | Matrix and vendor extension packages |
| `scripts/fixup.sed` | Output-text repairs for CUDA names/types | No role in the new semantic path |
| `nvcc.mk`, `configure` | NVCC discovery, CUDA architectures, name-based feature filtering | Backend manifests and capability-based test/compilation selection |
| `test/`, `bench/` | CUDA test drivers and comparison infrastructure | Reusable workload fixtures with runtime-specific runners |
| `dist/`, packaging scripts | CUDA source snapshots and legacy toolchain bundles | Neutral packages, backend artifacts, bindings, and evidence manifests |
| `.github/workflows/` | Existing verification/build/hardware workflow assumptions | Separate proof, compiler, driver, interoperability, and packaging gates |

A lexical scan finds 240 distinct quoted `Kuiper.*` identifiers in the extractor. That is an inventory seed, not a claim that 240 independently supported operations exist: names include types, constructors, and multiple cases. Phase P0 must classify the complete dispatch table, default extraction behavior, reachable externals, and emitted host operations.

## Inputs and required artifacts

Read the pinned upstream [Kuiper build rules](https://github.com/FStarLang/kuiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/nvcc.mk), [extractor](https://github.com/FStarLang/kuiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/extraction/ExtractKuiper.fst), and [project documentation](https://github.com/FStarLang/kuiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/README.md). The user's fork has the same implementation revision. Work in a clean checkout with the recorded F*/Karamel gitlinks.

`legacy-scope.json` freezes extraction roots from `nvcc.mk`: example and Klas `.fst` modules, excluding `Inst.fst`, plus `Kuiper.GraphDist`; tracked `.fst.sh` generators identify additional outputs to materialize. It also retains test drivers and their expected-output inputs. Every listed extraction root/generated family is mandatory for full replacement. A nonexecuting/spec-only root can be discharged only by evidence that it contributes no exported executable behavior, retaining its dependent source/evidence role.

`primitive-seeds.json` records quoted Kuiper symbols and occurrence locations from the pinned extractor. It is a lexical seed, not a semantic ledger. A type, helper name, match alternative, overloaded case, and executable primitive are different things; do not count names as feature coverage.

## Produce the complete semantic ledger

1. Materialize generator outputs using the pinned source build; record script/input/output digests. Enumerate declarations reachable from each extraction root, including host plans and imported services.
2. For every extractor match case, record the source symbol, full typed signature, type/static/ghost/runtime arguments, current emitted expression, effect, pre/postconditions, numeric mode, and required hardware behavior. Split one name into multiple rows where the typed cases differ.
3. Trace fallback/default F* extraction and foreign headers. Add rows for behavior not named in the central dispatch. Record an edge from each exported entrypoint to each transitively required row.
4. Link each row to a KIR operation or to an explicit extension. Record frontend, compiler, runtime, ABI, and proof responsibilities separately. A device operation that calls a host helper needs both sides accounted for.
5. Classify implementation status separately from scope: unimplemented, experimental, tested, qualified. Mandatory scope remains mandatory when implementation is difficult. Additive vendor extensions can retain specialized semantics without making them part of the portable intersection.

Required row fields are `row_id`, `source_revision`, `source_location`, `signature`, `effects`, `assumptions`, `entrypoint_users`, `kir_semantics`, `requirements`, `implementation_status`, `evidence_policy`, `owner_role`, and `closure_evidence`. Missing classification is a failed inventory gate.

## Reproduce the baseline

Existing commands, after installing the repository's prerequisites and initializing its submodules:

```bash
git submodule update --init --recursive
make -j$(nproc) prepare
make -j$(nproc) verify
make -j$(nproc) extract-all
make -j$(nproc) list-admits
make -j$(nproc) test
```

The last command requires the relevant NVIDIA/CUDA environment. Missing hardware is a blocked result. Run verification with development bypass settings absent, including an unset `ADMIT`; this Make configuration treats a nonempty value as enabled. Validate the provenance of reused `.checked` outputs or rebuild them. Record compiler/solver/options and each transitive proof assumption. The instructions above are existing entrypoints, not commands executed by this documentation revision.

## Close F01 without shrinking the objective

G-COVERAGE qualifies a named portable release. G-REPLACEMENT additionally requires every mandatory baseline row and entrypoint to have a CUDA-independent qualified implementation under its original or explicitly accepted revised semantic contract. Neither an unsupported diagnostic nor optional legacy CUDA support closes a mandatory row. Retirement requires an explicit reviewed scope change identifying affected users; no automatic scope change is authorized by this roadmap.

Include a release-evaluator fixture where tensor/vendor rows are deferred. Portable release status may pass; migration completion must fail. A WGMMA-specific relation may require a separate vendor backend or remain a blocker. Similar numerical output is insufficient to retire its instruction-specific contract.

## Evidence required to close this milestone

Close **G-BASELINE, G-REPLACEMENT** only with the implementation artifacts, positive and negative cases, and source-to-result identities required above. Link the implementation relation to the named F* symbols and the policy's applicable O1–O10 obligations. The specification's proved lemmas are reusable model facts; they do not discharge this implementation correspondence. Record unresolved cases as blockers or explicitly outside the claim. No backend gate is marked passed by this roadmap revision.
