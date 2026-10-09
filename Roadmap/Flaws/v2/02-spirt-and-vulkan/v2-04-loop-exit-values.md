# V2-04: the loop mapping omits the pinned SPIR-T exit-value convention

**Severity:** Medium. **Status:** Open. **Evidence class:** confirmed dependency constraint missing from the detailed mapping. **Owner:** SPIR-T adapter and extraction owners. **Resolve by:** the P0 construction probe and W08, before accepting general loop results.

## 1. Affected instructions

V2 correctly identifies tail-controlled loops and the need to preserve zero iterations in [control and pointers](../../../04-spirt-and-gpu-lowering/control-and-pointers.md). The [operation mapping](../../../04-spirt-and-gpu-lowering/operation-mapping.md) names `NodeKind::Loop`, region outputs and carried values. It does not distinguish backedge values from values exported after the loop at the selected SPIR-T revision.

See the [frozen mapping](https://github.com/QuasarRay/Quiper/blob/9dceaf274b46f295f7fc312fb3396d5729d7d97d/Roadmap/04-spirt-and-gpu-lowering/operation-mapping.md). This finding does not reopen the corrected tail-control explanation or assert that issue #31 has been reproduced.

## 2. Evidence and failure case

The pinned [region and node definitions](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/lib.rs#L779-L852) specify a nonuniform result convention:

- A selection case's region outputs provide the selection node's results.
- A loop body's region outputs provide the next iteration's inputs. They are not available as ordinary `Value::NodeOutput` results of the loop.
- Body-defined values may escape the loop under the pin's dominance rules. The source explicitly describes this as a representation that may change.

The official [RegionDef API documentation](https://rust-gpu.github.io/spirt/spirt/struct.RegionDef.html) is supporting context; the pinned source is the API authority for this roadmap.

For `i = 0; while i < n { i = i + 1; }`, the final value must be zero when the body never runs and the final updated value otherwise. A generic adapter that gives all structured nodes the same result accessor can construct an unsupported loop result. An adapter that uses the final iteration's input instead of its updated value can return `n - 1` for positive `n`.

These are construction mistakes the present instructions do not rule out concretely. No SPIR-T compilation or miscompilation was reproduced here.

## 3. Required correction

Add a pinned mapping for four distinct channels: initial loop inputs, backedge outputs, repeat-condition value and post-loop values. Explain which body-defined values can legally be referenced after the loop at this pin. Do not map KIR loop results mechanically to `Value::NodeOutput` on a `Loop`.

For a zero-iteration source loop, use the outer selection's results to merge initial values with the qualified executed-loop result. Preserve the source condition's evaluation point and effects. Cover outputs that are not themselves carried on the backedge, multiple carried values, nested loops and early exits.

If a later private SPIR-T revision changes this representation, implement a separate versioned adapter mapping and rerun the correspondence tests. A proposed upstream change does not alter the baseline API retroactively.

## 4. Closure evidence

Compile the actual adapter against `e8757adba8d14068a7bf1b3bc9f24cac982f4bd3`. Use fixtures with zero, one and several iterations whose final values distinguish the input, update and exit result. Add a negative adapter invariant rejecting unsupported loop-node output references before lifting.

Check the emitted result against the independent KIR relation, not only IR printing or `spirv-val`. Keep this probe separate from F08's unresolved upstream lifting report so that passing one does not falsely close the other.
