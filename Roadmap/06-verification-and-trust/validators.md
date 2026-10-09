# Validators

## 5. Translation validation strategy

Use a small, independently specified reference evaluator for the supported deterministic KIR subset. It is valuable for tests and debugging; ordinary execution comparison is not a proof for all inputs.

For transformations with tractable relations, implement translation validators that check each produced artifact. Their soundness is part of the trusted argument. Begin with scalar operations, layouts, index arithmetic, simple control flow, and resource bindings. Extend deliberately to loops, aliasing, and concurrent operations.

For concurrent behavior, evaluate allowed outcome sets and happens-before/visibility relations. A scalar interpreter cannot validate a subgroup barrier or a weak-memory atomic lowering. Use explicit semantics, proof obligations, and bounded model exploration where appropriate; report bounded scope honestly.

Retain the legacy CUDA route for differential tests, and also compare against independent specifications/reference calculations. Agreement between two routes sharing the same extraction error is not independent evidence.

SPIRV-Tools validation is a required format/environment gate, but upstream documents that the validator is incomplete. It cannot establish that emitted code computes the intended function. Vulkan validation layers similarly support API diagnosis, not application functional proofs. [E5](../10-sources/README.md)
## 6. Evidence integrity

Bind evidence to the canonical source/IR digest, semantic profile, operation definitions, compiler pipeline, numerical flags, target requirements, and output artifact. A changed pass sequence or approximation mode invalidates the previous relation unless a new checked relation covers it.

Every pass reports preserved/invalidated facts. Reject an output whose necessary evidence disappeared. Do not attach evidence only to function names or line numbers; rewrites and renaming make those insufficient.

Cache verification results only under the complete proof/checker/toolchain identity. Replaying a previous `.checked` file from a different source or compiler build is not a release verification run. Package seeding may speed local development; release evidence needs validated provenance and a reproducible replay policy.

Unknown checker identities, unsupported proof formats, mismatched digests, unavailable assumptions, or incomplete verification cause verified-profile rejection. Users may explicitly choose an unchecked mode, but it must be a distinct artifact/run policy.
