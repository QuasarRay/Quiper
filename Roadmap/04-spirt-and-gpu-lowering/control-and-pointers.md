# Implement control-flow and pointer legalization safely

## 1. Establish the control-flow relation first

Represent structured selections/loops directly when possible. Preserve condition evaluation order, carried values, exits, effects and participating invocations. If the importer supplies CFG/SSA, call the pinned `passes::legalize::structurize_func_cfgs` only after checking the accepted subset and recording the input relation. The official [pass source](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/passes/legalize.rs) exposes that entrypoint; it does not prove Kuiper-specific barrier preservation.

Resolve executable imports and determine reachability before final validation. The upstream [round-trip example](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/examples/spv-lower-link-lift.rs) demonstrates APIs and diagnostics. Its pass order is not a normative Kuiper sequence. Declare pre/postconditions for each scheduled pass, including whether it can introduce pointer merges, duplicate control conditions, or alter uniformity.

The pinned [`NodeKind::Loop`](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/lib.rs#L862-L920) is tail-controlled: it executes the body before testing its repeat condition. A source `while` cannot be copied into that node unchanged. Implement an initial condition/selection around the loop or another checked transformation that preserves zero iterations, side effects and carried values. Evaluate each source condition at its required point; duplicating a condition with effects changes behavior. For early exits and nested loops, specify how every carried output is selected, and reject shapes outside the proved/validated mapping. Use separate fixtures for zero iterations, one iteration, condition effects, early exits and nested carried state. This representation issue is distinct from the unconfirmed lifting report below.

## 2. Triage the loop report before enabling affected paths

Treat [issue #31](https://github.com/Rust-GPU/spirt/issues/31) and [draft PR #30](https://github.com/Rust-GPU/spirt/pull/30) as an unresolved qualification dependency. The inspected proposal head is `0e40966f27468d4896b51e28ea0e9080a8300bbe`.

1. Obtain the minimized fixture from that exact proposal and build baseline/patched workers independently with their pinned tools.
2. Establish the defined behavior of the input. The presence of an undefined value on an unreachable path is not itself proof of a bug.
3. Run the exact lower/structurize/lift pipeline and compare the allowed observable outcomes, not only pretty-printed shapes or validator success.
4. Test whether direct KIR construction reaches the same problematic structure. Direct construction does not automatically exclude lifting bugs.
5. Accept a reviewed fix with regression evidence, prove and enforce an exclusion, or document why the report is inapplicable. Until then, block affected control shapes. Never call the report reproduced solely from reading it.

Include zero/one/multiple iterations, early exits, nested control, duplicated predicates, loop-carried values and barrier participation. A bounded unrolled subset can be a temporary path only with its own preservation/size bounds; it does not qualify general loops.

## 3. Use logical resource accesses as the baseline

The first native KIR path keeps resource identity plus integer offset/extent. Generate typed storage-buffer access chains directly. Specialize helpers on resource identity or inline them within a bounded budget; keep ordinary scalar arguments as values. Do not introduce physical pointers, arbitrary pointer reinterpretation, variable-pointer capabilities, or descriptor-indexing requirements merely to mimic CUDA pointer syntax.

A select between offsets in one resource differs from selecting between resources. The latter needs an explicitly supported binding/selection lowering and capability checks. Reject it if unsupported. Validate offset arithmetic in a sufficiently wide checked representation before narrowing to the selected shader index width. Empty logical views do not authorize zero-sized Vulkan allocations or skipped side effects; handle them using a specified host-plan transformation or valid backing representation.

## 4. Make QPtr an eligible optional path

QPtr is not a mandatory pass for already typed logical-buffer lowering. Enable it for an explicitly qualified importer/operation subset. Before entry, reject or legalize the known restrictions:

| Restriction at the pin | Source | Required handling |
|---|---|---|
| Certain pointer-valued region/node merges | [analysis](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/qptr/analyze.rs#L883-L895) | Prove elimination, add a qualified lowering, or reject before QPtr |
| Calls with pointer arguments | [lifting](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/qptr/lift.rs#L450-L458) | Bounded inlining/specialization, a reviewed implementation, or rejection |
| Load/store memory operands | [lowering](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/qptr/lower.rs#L421-L434) | These instructions remain unchanged; validate the mixed form and preserve operands |
| 32-bit internal offsets/extents | [model](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/qptr/mod.rs) | Range-check every relevant value; no truncation or host-pointer-width inference |

Recheck eligibility after any pass that changes calls/control/memory. Check all emitted diagnostics before declaring success; an attached diagnostic is not an approved artifact. Never mask an unsupported internal case with a broad catch-and-continue path.

## 5. Check memory and convergence together

Map workgroup barriers to execution participation plus memory ordering over the actual affected storage classes. Map subgroup operations only for qualified subgroup-size/participation guarantees; the old fixed warp width does not prove portability. Atomic lowering preserves operation, type, address space, scope, ordering and numeric relation as a tuple. A CAS emulation needs its progress/NaN/order argument and must not silently replace an incompatible primitive.

Keep guards distinct from assertions as described in [guard failure semantics](../05-runtime-and-interop/memory-and-failures.md). No optimization may move effects across barriers or erase a potentially failing guard without an appropriate proof. These are O4–O7 obligations, supported by litmus/model exploration and real-device tests, not discharged by SPIR-V format validation.
