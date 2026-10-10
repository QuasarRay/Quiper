# S4. Implement additive composition and evidence admission

**Deliverable:** independently installable packages that compose through frozen meanings and protocols, with protected evidence admission and no modifications to existing core/backend files.

## Freeze what addition must preserve

`Extension.additive` preserves every existing semantic lookup, while permitting new identities. `unchanged` preserves every protected installed file. `decoupled_addition` requires both. The protected closure includes installed core binaries, public readers, built-in validators, existing frontends/bindings/backends, root workspace/lockfiles and central workflow/registry source. Deployment may add an external package root and explicit admission/selection configuration. Cache outputs belong outside the immutable installation.

Implement generic role protocols for frontend, compiler, runtime, binding, checker, semantic extension and optional host compiler. An endpoint composes only when its produced contract identity matches its consumer's input contract; exact identity includes the meaning/version, not only a format name. Artifact reflection, numerical policy, target environment, ABI and evidence identities remain part of the match. A runtime package does not need to know a source language or a SPIR-T context.

Capabilities are full operation/type/storage/scope/order/numeric/shape/participation tuples. `supports` quantifies over the actual artifact requirements. A compiler's advertised tuple must agree with emitted code and an admitted relation. Unsupported required extensions fail before execution. The core can preserve opaque payloads only through a specified envelope; it cannot optimize unknown effects away.

## Implement the compiler boundary once

Use KIR as persistent input, private worker IR during compilation, and a versioned target package as output. Independent workers may pin different SPIR-T versions. Do not serialize `Rc<Context>`, entity handles, Rust structs or driver objects. Compile a bounded pass DAG with fixed-point composites carrying explicit budgets and evidence relations. Any pass failure invalidates partial executable output; fallback must preserve the original semantic and evidence policy.

Prove stage composition with `Refinement.refinement_transitive` after supplying each stage relation. The lemma establishes transitivity, not the truth of the individual relations. Preserve defined errors, guard failures, numerical nondeterminism and progress requirements in the observation projection. A source-precondition failure and an implementation crash cannot be silently identified.

## Admit policies through data and protected authority

The [v2 result schema](../schemas/gate-result.schema.json) represents assurance as a namespaced identity and definition digest. [Proposed policy definitions](policies/registry.json) fix the three initial meanings and their file digests; their status is proposed, not operational admission. The deployment's protected registry selects admitted definitions/checkers. Unknown names, wrong digests or unadmitted checkers fail closed even when the schema accepts the shape.

`admitted_policy` binds a claim to that protected registry. `acceptable_evidence` binds exact input/output content, checker, policy, assumptions and the complete required propositions. A concrete evidence checker must justify those propositions and its own soundness. Parsing JSON, matching hashes or trusting a producer's `passed` field is insufficient. Never allow an extension to replace the mandatory relation with a new ad hoc trust entry under the old identity.

Version-1 records keep the [version-1 schema](../schemas/gate-result.v1.schema.json) and decoder. Migration resolves their fixed aliases to the original definitions and writes a new version-2 record/digest with provenance. It does not rewrite old evidence or infer qualification. Adding a fourth policy requires a new definition/checker plus authorized configuration; it must not require a core enum edit.

## Acceptance evidence

Run compiler-only and shared-artifact runtime-only substitution, then a distinct real GPU API, an independent frontend/binding and an effectful semantic extension. Compare protected hashes before/after and rerun on the final release candidate. Include a fourth-policy schema/admission test. Source or root-build edits mean the addition guarantee has failed; revise provisional contracts and repeat the freeze before claiming success.
