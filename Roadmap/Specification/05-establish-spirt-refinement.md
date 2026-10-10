# S5. Establish the SPIR-T realization relation

**Deliverable:** a qualified adapter from the portable relations to one pinned SPIR-T/Vulkan profile. Keep this realization separate from the neutral specification so a future backend adds another implementation of the same contract.

## Construct and validate the first artifact

Follow [M13](../04-spirt-and-gpu-lowering/01-build-and-qualify-the-worker.md) and [M14](../04-spirt-and-gpu-lowering/02-run-the-integer-vertical-slice.md). Pin SPIR-T, Rust, SPIRV-Headers/Tools and every local patch. A session owns a private `Rc<Context>`, module, type/constant interning tables, logical-resource mapping, value correspondence and requirement accumulator. Publish only a complete package after all diagnostics, target validation and reflection/evidence checks pass.

The baseline selects Vulkan 1.2, SPIR-V 1.5, GLCompute, logical addressing and the Vulkan memory model. Use literal local sizes and materialized static specialization. ID-bearing annotation support is an explicit investigated restriction; do not assume a printed annotation is preserved by all visitors. Use typed logical storage-buffer accesses first. QPtr is an optional eligible subset with pointer-merge/call/range/operand restrictions.

Define descriptor and scalar-parameter layouts before emitting accesses. `Lowering.layout_valid` expresses a simple containing-binding profile: the logical start is relative to that binding's base, strides equal cell size, the binding offset is aligned and its length covers the logical extent. A different rebasing/layout policy needs an explicit relation. Check offset/extent multiplication, device limits, storage Boolean encoding and tail-lane behavior. Preserve barrier participation even for data-inactive lanes.

## Prove the four loop-value channels

Map initial state to `Loop.initial_inputs`, current state to body `RegionInput`, next-iteration state to body `outputs`, and final positive-iteration state to dominating body-defined values. The pin explicitly does not expose Loop results as `Value::NodeOutput`. Wrap source while loops in an outer selection so zero iterations return initial values and positive iterations return the final updated values through Select outputs.

`while_exit` and its two lemmas specify that final selection. `valid_carried` and `typed_values_arity` specify carried arity/type consistency. These are necessary pieces of O4/O5, not the full loop equivalence theorem. Establish condition evaluation order, source/target state correspondence across every iteration, correct last values, effect ordering and supported early exits. Keep unresolved upstream issue #31 distinct from this confirmed representation rule.

## Preserve memory, convergence and numerical relations

Relate target event traces to `Memory.admissible`, not merely to a validator-accepted instruction list. Identify the full Vulkan memory model features required by the chosen relation. `Lowering.sufficient` requires availability/visibility chain support when chains are used; the base memory-model feature and DeviceScope do not imply it. Check both support and enablement.

Each barrier and atomic lowering carries its execution/memory scope, storage classes, order and participation. CAS emulation needs its value, ordering and progress argument. Each floating/matrix lowering implements the selected numerical relation and exact format/shape constraints. If a library transformation changes any of those premises, invalidate and re-establish the corresponding evidence before emission.

The complete procedure and negative fixtures are in [M15](../04-spirt-and-gpu-lowering/03-preserve-control-memory-and-participation.md) and [M16](../04-spirt-and-gpu-lowering/04-complete-numerics-and-target-emission.md). Reject unsupported pointer/control/numeric cases before executable publication. Direct construction and SPIR-V validation do not bypass the semantic obligations.

## Acceptance evidence

Qualify one frontend-free integer fixture and one real Kuiper export, then views, loops, dependent launches, reductions and matrix workloads. Validate, independently reflect and execute the exact candidate. Record 0/1/multiple loop results; missing feature rejection; scalar, layout, failure, convergence and exceptional numeric cases. O4–O7 remain implementation work until these mappings have the required proof/checker evidence; GPU tests supplement that argument.
