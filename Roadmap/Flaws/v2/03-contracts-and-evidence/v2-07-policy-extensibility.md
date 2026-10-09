# V2-07: the result schema blocks the promised additive assurance-policy extension

**Severity:** Medium. **Status:** Open. **Evidence class:** reproducible schema/prose contradiction. **Owner:** contracts, assurance and release-tooling owners. **Resolve by:** P1, before freezing the result reader used by extension qualification.

## 1. Affected instructions

[Capabilities](../../../03-backend-extension-contract/capabilities.md) allows a semantic extension requiring a new assumption to be deployed additively under a changed assurance claim. The [package contract](../../../03-backend-extension-contract/implementation.md) and [evidence policy](../../../06-verification-and-trust/evidence-policy.md) require a new policy identity. The [no-edit exercise](../../../03-backend-extension-contract/addition-only-tests.md) freezes contract readers and built-in validators.

The [gate-result schema](../../../schemas/gate-result.schema.json), at `/properties/profile/properties/assurance/enum`, permits exactly three strings. Its [pinned v2 version](https://github.com/QuasarRay/Quiper/blob/9dceaf274b46f295f7fc312fb3396d5729d7d97d/Roadmap/schemas/gate-result.schema.json) has no policy-reference or extension alternative.

## 2. Reproduction and consequence

Take an independently reviewed policy whose identity is `refinement-verified-v2`. The name is a witness, not a proposal to grant this policy trust. It is absent from the enum. Any conforming implementation of this enum assertion rejects a result that names it, even after the protected admission policy has approved its checker and semantics.

The [JSON Schema validation specification](https://json-schema.org/draft/2020-12/json-schema-validation#section-6.1.2) makes this a structural rejection. The [executed diagnostic](../05-coverage-and-evidence/diagnostics.md) checks the exact enum in the committed file; it does not claim to have run a complete JSON Schema validator.

Relabeling the result as `refinement-verified-v1` would misstate its assumptions. Editing the frozen enum/reader defeats the stated additive deployment path. Installing another schema might be a viable design, but v2 does not specify a discovery/negotiation route that makes the old result reader use it without changing the protected contract.

## 3. Required correction

Make the policy identity extensible while keeping trust admission closed. For example, use a namespaced policy ID and canonical definition digest, resolved by an independently admitted policy/checker package. Keep the three current policies as fixed definitions with immutable meanings. A display category must not replace the exact policy identity.

Alternatively, explicitly define versioned result-reader packages and how the frozen core routes a new envelope to them. Document which extension promise that supports. Do not leave schema replacement as an undocumented escape from the no-edit test.

Do not repair this by accepting arbitrary strings as trusted claims. Structural parsing, policy recognition, assumption comparison, evidence evaluation and release authorization remain separate checks. Bind the selected policy's digest and admitted checker identity into the result's required inputs and attestation relation.

## 4. Closure evidence

Freeze the core/result reader, install a reviewed new policy/checker package and admit it through configuration. Record and evaluate its result without editing existing contract files. Repeat with an unknown ID, wrong definition digest, unadmitted checker and a changed meaning under an old ID; reject each.

Keep old v1 records readable and preserve their original claims. This is a residual interaction between F02's fixed policies and F15's additive extension contract, not a reason to weaken either requirement.
