# Close lowering and policy-admission contracts

The roadmap correction is complete. Concrete backend implementation and qualification remain pending. The F* lemmas below concern the specified model; each implementation must establish the correspondence and closure cases.

## V2-04

Initial inputs, body inputs, backedge outputs and final body-defined values have separate mappings; outer Select supplies source while results.

Implement [the milestone](../../../04-spirt-and-gpu-lowering/03-preserve-control-memory-and-participation.md) against `Lowering.zero_iteration_values; Lowering.positive_iteration_values; Lowering.typed_values_arity` in [the specification](../../../Specification/README.md). Preserve [the original finding](../02-spirt-and-vulkan/v2-04-loop-exit-values.md) as the reason for the change.

**Required implementation evidence:** Compile and execute 0/1/multiple/nested/early-exit accumulator fixtures and reject fictitious Loop node-output references.

## V2-05

Chain-dependent semantics require supported and enabled availability/visibility chains, or a separately proved/enforced chain-free restriction.

Implement [the milestone](../../../04-spirt-and-gpu-lowering/03-preserve-control-memory-and-participation.md) against `Lowering.chains_require_feature` in [the specification](../../../Specification/README.md). Preserve [the original finding](../02-spirt-and-vulkan/v2-05-memory-model-chains.md) as the reason for the change.

**Required implementation evidence:** Reject missing-chain-feature capability fixtures before dispatch and qualify direct/multi-hop publication litmus cases.

## V2-07

Result schema v2 carries namespaced policy identity plus definition digest; protected admission retains immutable old meanings.

Implement [the milestone](../../../06-verification-and-trust/02-admit-immutable-evidence-policies.md) against `Extension.unknown_policy_rejected` in [the specification](../../../Specification/README.md). Preserve [the original finding](../03-contracts-and-evidence/v2-07-policy-extensibility.md) as the reason for the change.

**Required implementation evidence:** An unchanged reader accepts an admitted fourth policy and rejects unadmitted identities, wrong digests and self-admission; preserve v1 decoding.
