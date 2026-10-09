# 2. SPIR-T feasibility and lowering

All statements about the current SPIR-T implementation below refer to `e8757adba8d14068a7bf1b3bc9f24cac982f4bd3`. Proposed upstream changes are identified separately. Finding a candidate patch does not establish that it is correct, merged, compatible with the selected pipeline, or suitable for production.

## F07

**The pinned bridge rejects ID-bearing annotations needed by some planned paths.**

**Severity:** High. **Class:** confirmed source limitation. **Owner:** compiler and numerical owners. **Resolve by:** P0 feasibility; qualify before P3/P5 use of the affected features.

**Location:** [specialization, construction, and emission](https://github.com/QuasarRay/Quiper/blob/689c4528f227704df989f0f1e3eaabb8ce4b600a/Roadmap/04-spirt-and-gpu-lowering.md#L13-L22), [shared-memory specialization and FP policies](../04-spirt-and-gpu-lowering.md#4-shared-memory-and-pointer-legalization).

**Evidence:** [`spv/lower.rs`, lines 478–496](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/spv/lower.rs#L478-L496) recognizes `OpExecutionModeId` and `OpDecorateId`, but rejects annotations containing more than the target ID. The current [`Attr::SpvAnnotation`](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/lib.rs#L393-L417) representation and [annotation emission](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/spv/lift.rs#L1500-L1538) also require attention when constructing the IR directly.

[PR #48](https://github.com/Rust-GPU/spirt/pull/48), open at inspection, proposes ID-bearing annotation storage and traversal/lowering/lifting changes. Its inspected head is `94f5c19c3d3ad258bc628857a350719561d64b44`. This is relevant both to specialized execution modes and to the float-control work motivating that PR.

The roadmap calls for specialization and explicit numerical controls without naming this dependency. A supported `LocalSizeId` module entering through an importer reaches the current rejection. Building SPIR-T directly does not by itself supply a missing annotation representation and emitter. Vulkan also requires `maintenance4` to be enabled when `LocalSizeId` is used; see [VUID-RuntimeSpirv-LocalSizeId-06434](https://docs.vulkan.org/refpages/latest/refpages/source/RuntimeSpirv.html).

This is **not** evidence that all float modes fail or that the current code silently drops these annotations. The inspected input path rejects them. Literal execution modes and specialization completed before emission may provide a restricted alternative.

**Required correction:** add an explicit annotation-support row to the feasibility ledger. Select and qualify one of three paths: a reviewed patch, a representation/emission change in the private adapter, or a restricted profile that materializes supported literal forms. Record which ID-bearing decorations/execution modes remain unsupported. Preserve their semantic dependencies through transformations and evidence hashing.

**Closure:** test direct construction and import/lift with ordinary modes, ID-bearing modes, specialization values, and relevant float-control annotations. Check referenced constants after transformations, final SPIR-V validation under the exact environment, device feature enablement, reflection, and observed execution. A rejected unsupported case must remain rejected; it must not become a weaker accepted numeric mode.

## F08

**A reported loop-lifting problem has no explicit triage gate.**

**Severity:** High, pending reproduction and applicability analysis. **Class:** upstream report; not reproduced. **Owner:** compiler and verification owners. **Resolve by:** approval of the P3 transformation pipeline.

**Location:** [control-flow normalization and structurization](https://github.com/QuasarRay/Quiper/blob/689c4528f227704df989f0f1e3eaabb8ce4b600a/Roadmap/04-spirt-and-gpu-lowering.md#L13-L24), [compiler tests](../08-production-acceptance.md#3-correctness-suites), and [the original source audit](../10-sources.md#spir-t-evidence).

**Evidence:** [issue #31](https://github.com/Rust-GPU/spirt/issues/31) reports an `OpUndef`-fed loop-carried phi after a lower/structurize/lift sequence. [Draft PR #30](https://github.com/Rust-GPU/spirt/pull/30) proposes a guarded loop-shortcut canonicalization and regression fixtures. Its inspected head is `0e40966f27468d4896b51e28ea0e9080a8300bbe`, based on the same SPIR-T revision used by the roadmap. The author describes a miscompilation; the candidate fix remains unmerged.

The original roadmap did not inspect this upstream development work. Generic compiler fuzzing and a format validator are insufficient substitutes for triaging a known report on the intended control-flow path. Structurally valid output can still compute a different result.

The report's use of undefined values also requires care. A retained `OpUndef` or phi is not, by itself, proof of incorrect compilation. The audit has not established that the fixture begins with fully defined observable behavior, that the reported change violates that behavior, or that Kuiper's direct-construction path produces the same shape.

**Required correction:** introduce a blocking triage task for the selected pipeline. Reproduce at the pinned revision, determine the defined-input/outcome relation, compare the transformed result, and establish reachability from KIR. Evaluate the candidate patch independently. Until resolved, either prove the excluded control-flow shape cannot be emitted, reject it explicitly, or use a qualified alternative pipeline.

**Closure:** preserve a minimized, semantically justified regression; record exact tools, input/output hashes, expected relation, observed result, and applicability to Kuiper. Include zero, one, and multiple iterations; duplicated conditions; early exits; and loop-carried values. Closing this finding requires an explained fix, a proved exclusion, or evidence that the report does not apply. Merely merging the draft or observing valid SPIR-V is insufficient.

## F09

**QPtr's usable subset is narrower than the migration matrix makes actionable.**

**Severity:** High. **Class:** confirmed source limitations. **Owner:** compiler and memory owners. **Resolve by:** G-CONTRACT's supported subset and the P3/P4 lowering plan.

**Location:** [QPtr use and pointer legalization](https://github.com/QuasarRay/Quiper/blob/689c4528f227704df989f0f1e3eaabb8ce4b600a/Roadmap/04-spirt-and-gpu-lowering.md#L17-L24), [shared storage and layouts](../04-spirt-and-gpu-lowering.md#4-shared-memory-and-pointer-legalization), and [KIR calls/control/memory](../02-language-independent-extraction.md#4-define-the-executable-kernel-subset).

The roadmap correctly limits QPtr use to its analyzed subset and notices 32-bit fields. It does not enumerate several current restrictions that affect ordinary views, conditional resource selection, and helper calls:

| Inspected code | Actual behavior | Required design decision |
|---|---|---|
| [`qptr/analyze.rs`, 883–895](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/qptr/analyze.rs#L883-L895) | Certain non-root region inputs/node outputs produce an unsupported-phi diagnostic | Whether pointer-valued merges are eliminated, handled elsewhere, patched, or rejected |
| [`qptr/lift.rs`, 450–458](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/qptr/lift.rs#L450-L458) | Calls with recognized pointer arguments reach an unimplemented-case error | An explicit call/inlining/legalization policy with bounded expansion |
| [`qptr/lower.rs`, 421–434](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/qptr/lower.rs#L421-L434) | Loads/stores with memory operands are left unchanged | A policy for mixed QPtr/SPIR-V instructions and preservation of those operands |

The last row does **not** mean memory operands are discarded. It means the conversion does not cover those instructions. Treating a partially transformed function as fully legalized would be a separate integration error.

**Required correction:** define a pre-QPtr eligibility analysis over the actual pipeline form. Specify when calls are inlined, how pointer merges are represented, and how unconverted operations retain memory semantics. Do not rely on an upstream internal-error diagnostic as the public unsupported-feature check. A patch or alternative lowering needs its own refinement and regression work.

**Closure:** compile fixtures for a helper receiving a slice, a pointer selected by control flow, a loop-carried view, and loads/stores with required memory operands. Each must either use a qualified path or fail before unsafe partial emission with a precise diagnostic. Include both sides of the supported offset/extent limits. Apply the same tests after optimization, since a transform can introduce a previously absent merge or call shape.

## F10

**The reuse and maintenance decision omits active upstream architecture work.**

**Severity:** Medium. **Class:** investigation gap. **Owner:** compiler and architecture owners. **Resolve by:** P0 dependency strategy and P1 SDK design.

**Location:** [reuse requirement](../README.md#1-required-result), [SPIR-T SDK ownership](../01-target-architecture.md#2-proposed-module-ownership), [source scope](../00-current-state-and-gaps.md#1-audited-baseline), and [upstream ownership decision](../09-work-packages-and-decisions.md#3-p0p1-decisions-still-requiring-evidence).

The original audit explicitly limited itself to main. That scope was disclosed, but it was insufficient for committing to a new adapter SDK, reference machinery, and maintenance estimate without an upstream reuse comparison.

The open development work includes [memory-module separation #25](https://github.com/Rust-GPU/spirt/pull/25), [control-flow reorganization #26](https://github.com/Rust-GPU/spirt/pull/26), [native scalar operations #42](https://github.com/Rust-GPU/spirt/pull/42), [vectors #43](https://github.com/Rust-GPU/spirt/pull/43), [aggregate disaggregation #45](https://github.com/Rust-GPU/spirt/pull/45), a [Vulkan round-trip layer #41](https://github.com/Rust-GPU/spirt/pull/41), and the [prototype CPU interpreter #46](https://github.com/Rust-GPU/spirt/pull/46). Several form a dependency stack rather than isolated patches. #46 was explicitly an early limited prototype at inspected head `7e664801ece15c16068c3b5edd5fb00895217e24`.

These are reuse candidates and evidence of likely integration churn. They are not available production features of the pinned main branch. An SPIR-T interpreter also cannot replace the host-plan executor, source extraction, or an independent KIR semantic oracle. Sharing lowering or operation implementations with the system under test reduces its independence as a correctness oracle.

**Required correction:** produce a small dependency decision record: pinned main plus local fixes, a reviewed upstream stack, or a deliberately limited private adapter. Record needed commits, prerequisite chains, tests, ownership, rebase/update policy, and exit conditions for local patches. Compare each planned custom component with applicable upstream work. Re-estimate only the work affected by the decision.

**Closure:** every adopted or rejected candidate has a scope-based rationale. The build manifest pins the selected implementation; no feature is credited merely because a PR exists. Upgrade qualification includes F07–F09 and the Kuiper corpus. Keep a separately specified oracle for claims that require independent evidence.
