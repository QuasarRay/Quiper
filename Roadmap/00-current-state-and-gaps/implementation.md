# Freeze and expand the migration inventory

## 1. Inputs and required artifacts

Read the pinned upstream [Kuiper build rules](https://github.com/FStarLang/kuiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/nvcc.mk), [extractor](https://github.com/FStarLang/kuiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/extraction/ExtractKuiper.fst), and [project documentation](https://github.com/FStarLang/kuiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/README.md). The user's fork has the same implementation revision. Work in a clean checkout with the recorded F*/Karamel gitlinks.

`legacy-scope.json` freezes extraction roots from `nvcc.mk`: example and Klas `.fst` modules, excluding `Inst.fst`, plus `Kuiper.GraphDist`; tracked `.fst.sh` generators identify additional outputs to materialize. It also retains test drivers and their expected-output inputs. Every listed extraction root/generated family is mandatory for full replacement. A nonexecuting/spec-only root can be discharged only by evidence that it contributes no exported executable behavior, retaining its dependent source/evidence role.

`primitive-seeds.json` records quoted Kuiper symbols and occurrence locations from the pinned extractor. It is a lexical seed, not a semantic ledger. A type, helper name, match alternative, overloaded case, and executable primitive are different things; do not count names as feature coverage.

## 2. Produce the complete semantic ledger

1. Materialize generator outputs using the pinned source build; record script/input/output digests. Enumerate declarations reachable from each extraction root, including host plans and imported services.
2. For every extractor match case, record the source symbol, full typed signature, type/static/ghost/runtime arguments, current emitted expression, effect, pre/postconditions, numeric mode, and required hardware behavior. Split one name into multiple rows where the typed cases differ.
3. Trace fallback/default F* extraction and foreign headers. Add rows for behavior not named in the central dispatch. Record an edge from each exported entrypoint to each transitively required row.
4. Link each row to a KIR operation or to an explicit extension. Record frontend, compiler, runtime, ABI, and proof responsibilities separately. A device operation that calls a host helper needs both sides accounted for.
5. Classify implementation status separately from scope: unimplemented, experimental, tested, qualified. Mandatory scope remains mandatory when implementation is difficult. Additive vendor extensions can retain specialized semantics without making them part of the portable intersection.

Required row fields are `row_id`, `source_revision`, `source_location`, `signature`, `effects`, `assumptions`, `entrypoint_users`, `kir_semantics`, `requirements`, `implementation_status`, `evidence_policy`, `owner_role`, and `closure_evidence`. Missing classification is a failed inventory gate.

## 3. Reproduce the baseline

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

## 4. Close F01 without shrinking the objective

G-COVERAGE qualifies a named portable release. G-REPLACEMENT additionally requires every mandatory baseline row and entrypoint to have a CUDA-independent qualified implementation under its original or explicitly accepted revised semantic contract. Neither an unsupported diagnostic nor optional legacy CUDA support closes a mandatory row. Retirement requires an explicit reviewed scope change identifying affected users; no automatic scope change is authorized by this roadmap.

Include a release-evaluator fixture where tensor/vendor rows are deferred. Portable release status may pass; migration completion must fail. A WGMMA-specific relation may require a separate vendor backend or remain a blocker. Similar numerical output is insufficient to retire its instruction-specific contract.
