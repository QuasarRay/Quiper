# S6. Replay proofs and check correctness evidence

**Deliverable:** reproducible model verification, a claim-to-source matrix and an honest implementation proof ledger. Correctness references and checked lemmas must state their limits.

## Reproduce the strict check

Use the official F* 2026.09.27 Linux x86_64 release, compiler commit `7deb38a27c86990664c6b41e4b6093597acdebd4`, with its bundled Z3 4.13.3. Verify the archive SHA-256 `cafbbb8960efeb26f5d30bc447b61ab8a507968948507a2580585afd1397781d` before extraction. The [official release](https://github.com/FStarLang/FStar/releases/tag/v2026.09.27) and [source lock](sources.json) record identity. Ensure the extracted solver is executable; changing a file permission does not change its bytes.

```bash
python3 Roadmap/tools/verify_spec.py \
  --fstar /absolute/path/to/fstar/bin/fstar.exe \
  --report /tmp/quiper-spec-verification.json
python3 Roadmap/tools/check_roadmap.py
```

The runner creates a fresh user-module cache, invokes each module explicitly, rejects proof-bypass syntax and records source/compiler hashes and full results. It uses the pinned release's standard library/proof foundation. Compare the new report's source hashes with the committed record. Do not reuse a result after modifying a module. A passing dependency-root command alone is insufficient; F* may load dependencies without checking their proof obligations as explicit roots.

This standalone pure specification deliberately does not import the Kuiper/Pulse compiler plugin. The existing project pins a different F* fork for source extraction. G-EXTRACT must validate that adapter and its verified source closure separately; successful standalone verification does not establish fork/plugin compatibility.

## Official evidence and its limits

| Official source | Specification connection | What the source supports | What it does not prove |
|---|---|---|---|
| [F* lemma guide](https://fstar-lang.org/tutorial/book/part1/part1_lemmas.html) | All declared `Lemma` facts and recursive `typed_values_arity` | Refinements, lemma obligations and induction as the proof mechanism used here | Correctness of this project's chosen requirements |
| [F* effect refinements](https://fstar-lang.org/tutorial/book/part4/part4_pure.html) | `contract`, preconditions, state relations and composition | Pure pre/post reasoning and specification of imperative behavior | Automatic refinement of a new compiler/runtime |
| [F* inductive semantics example](https://fstar-lang.org/tutorial/book/part2/part2_stlc.html) | `Kernel.executes` and `Host.host_step` | Inductively defined syntax, relations and proof objects | A complete typed KIR soundness theorem in this revision |
| [Pulse basics](https://fstar-lang.org/tutorial/book/pulse/pulse_ch1.html) | Logical permissions, caller preconditions and ownership recovery | Separation-logic pre/post reasoning over resources | Rust destructor/forget safety or a Vulkan memory mapping |
| [Pulse atomic invariants](https://fstar-lang.org/tutorial/book/pulse/pulse_atomics_and_invariants.html) | `Memory.admissible`, atomic events and invariant obligations | Concurrent resource/invariant reasoning that a realization must respect | A proof that any GPU atomic scope/order matches it |
| [Pulse extraction](https://fstar-lang.org/tutorial/book/pulse/pulse_extraction.html) | O1/O2 and pre-erasure capture | Extraction is a distinct stage; proof-only and executable content differ | Preservation by this proposed source-independent exporter |
| [Kuiper launch contract](https://github.com/FStarLang/kuiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/src/lib/kuiper/Kuiper.Kernel.Base.fsti#L18-L38) | `valid_call`, dependencies, successful postcondition publication | Launch consumes a pledged precondition and yields a later postcondition | That a fence proves a failed producer's postcondition |
| [Kuiper assertions](https://github.com/FStarLang/kuiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/src/lib/kuiper/Kuiper.Assert.fsti) and [implementation](https://github.com/FStarLang/kuiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/src/lib/kuiper/Kuiper.Assert.fst) | `assert_pre`, `guard_relation`, failure propagation | Assertions and successful guards have different contracts; existing admitted assumptions must remain visible | That erasing a runtime guard preserves behavior |
| [SPIR-T region and loop definitions](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/lib.rs#L779-L920) | `while_exit`, `valid_carried` and the final-value mapping | Tail-controlled loops, backedge outputs, Select outputs and dominating body values | A proved Kuiper-to-SPIR-T transformation |
| [SPIR-T module API](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/lib.rs#L198-L246) | Private worker ownership and external artifact boundaries | Context-owned module representation at the pin | A stable language-neutral plugin ABI |
| [Vulkan 1.2 feature contract](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVulkan12Features.html) | `sufficient` and `chains_require_feature` | Separate base, device-scope and availability/visibility-chain feature requirements | That the backend's requirement accumulator is complete |
| [Rust forget contract](https://doc.rust-lang.org/std/mem/fn.forget.html) | `safe_retention`, `scope_may_return` and forgotten-handle cases | Safe code need not run a handle's destructor | That an unimplemented Rust scope wrapper is sound |

These are primary documentation/source references. Pin-bound repository claims refer to exact immutable revisions. Web manuals can change; the source lock records the inspection date. The logical claims in this specification are our design and must be reviewed against the intended source behavior. They are not claims made by those projects about this roadmap.

## Proved facts and implementation obligations

| Checked facts | Remaining correspondence |
|---|---|
| Subview containment, disjointness, single-cell frame and U32 range | Actual layouts, alias tracking, narrowing and physical allocations |
| Zero-iteration construction and loop final-value selection/arity | Full source/KIR/SPIR-T loop equivalence, condition effects and early exits |
| Guard success implies predicate; pending use blocks free/scope exit | Collective-safe guard code and actual Rust/runtime ownership |
| Failed dependency blocks submission; completion is not success; safe transitions preserve retention; success requires guard/visibility/post checks | Scheduler, status visibility, resource registry, driver and teardown relations |
| Retired IDs cannot be fresh; redemption is one-time; timeout cannot release | Bounded concrete table, replay transport, client ownership and atomic redemption |
| Additive lookup preservation/composition and unknown-policy rejection | Immutable-installation measurements, protected registry and sound evidence checker |
| Refinement reflexivity/transitivity and retained-call precondition preservation | Every individual O1–O10 relation plus nonvacuity, prefix safety and progress |
| Missing evidence blocks qualification; deferred mandatory rows block replacement; uncertain upper interval cannot pass | Actual gate evaluator, trustworthy results and confidence coverage |

The machine record is the authoritative list of checked files and their hashes. Inspect each lemma's `requires` clause before reusing it. No theorem here establishes overall compiler correctness, full GPU memory-model refinement, every floating-point relation or production readiness.

## Close specification and implementation review separately

Specification review checks whether these definitions express the intended requirements, whether parameter instantiations are adequate, and whether counterexamples reveal missing observations or assumptions. Machine verification checks the lemmas against those definitions. Implementation qualification checks that real code and hardware satisfy the selected contract. Keep all three records; none implies the others.

For release statistics follow [M27](../08-production-acceptance/03-qualify-performance-and-cutover.md). The F* interval classifier assumes valid confidence bounds; it does not prove a sampling model or binomial calculation. For gate admission follow [M26](../08-production-acceptance/02-implement-candidate-bound-admission.md). Every future backend gate remains `not-run` until actual candidate-bound evidence exists.

## Executed checks for this revision

All 11 source modules were checked as explicit F* roots with a fresh user-module cache. Their 33 declared lemmas passed strict verification with the locked compiler and solver. The model source contains no proof bypasses. The proof report records each source hash and successful verification output.

Nine regression fixtures passed for the v2 policy envelope, retained v1 decoding, namespaced/digest-bound policy references, reference admission lookup and the missed-tail/sample-size arithmetic. These fixtures do not implement or qualify the production evidence checker or statistical evaluator. Install the pinned packages in `Roadmap/tools/requirements-validation.txt`, then run `python3 Roadmap/tools/check_v3_contracts.py`.

The document checker validates local links, active milestone coverage, all 29 historical/v2 resolution references, named F* symbols, current proof-file hashes and policy-definition digests. With the local SPIR-T clone supplied, it also checks pinned source paths/line ranges from both repositories. It does not claim to validate the content of mutable web manuals or run GPU gates.

[The specification workflow](../../.github/workflows/roadmap-spec.yml) repeats document, fixture and strict F* checks on relevant pushes and pull requests using the authenticated release archive. Its result is separate from the existing CUDA/Pulse project CI and from production qualification.
