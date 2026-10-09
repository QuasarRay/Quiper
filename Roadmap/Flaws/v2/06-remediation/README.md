# Remediation order and closure requirements

Correct the affected contracts before treating v1 interfaces as frozen. Keep the existing replacement scope, proof requirements and independent compiler/runtime tests. The new findings extend those requirements where the v2 instructions remain incomplete.

## 1. Implementation order

| Order | Work | Findings | Existing owners/work packages | Required evidence |
|---|---|---|---|---|
| 1 | Define failure-dependent execution, leak-safe host memory ownership and operation retirement | V2-01, V2-02, V2-03 | W03–W04, W10–W14; runtime/binding/verification | State transitions, actual safe API shapes, negative lifetime/replay cases |
| 2 | Resolve the exact SPIR-T loop mapping and Vulkan semantic feature dependency | V2-04, V2-05 | W08–W09, W12, W18; compiler/concurrency | Compile-checked mapping, final-value fixtures, chain-dependent rejection and O6 relation |
| 3 | Fix Vulkan host-object concurrency before enabling parallel service calls | V2-06 | W10/W14; runtime/bindings | Queue/pool ownership table, enforced thread rules, concurrent call tests |
| 4 | Make admitted policy identities representable without core/schema edits | V2-07 | W03–W04, W18–W20; contracts/assurance | Frozen-reader additive policy exercise plus unadmitted-policy rejection |
| 5 | Ratify a statistically specified performance decision procedure | V2-08 | W02/W22; performance/release | Tail model/adequacy rules, interval method, multiplicity resolution and calibration fixtures |
| 6 | Requalify affected final-candidate claims | All applicable findings | W21–W22; release | Candidate-bound gate results with actual implementation/proof/hardware evidence |

Resolve policy choices during P0/P1 even when their implementation gate is later. P7's no-edit exercise must include any newly supported policy path. P8 must use the corrected runtime, compiler, binding and measurement identities when selecting evidence.

## 2. Small reviewable changes

1. Add the runtime success/failure dependency relation, with a producer-fails/consumer-would-access case.
2. Select owned, scoped or copied asynchronous host memory APIs and document forgotten-handle behavior.
3. Add terminal-operation retirement, acknowledgement and stale-ID semantics with bounded-state tests.
4. Add the exact pinned loop value mapping and compile the adapter probe.
5. Add the chain-feature requirement/exclusion rule and the host synchronization table to the Vulkan profile.
6. Correct the result-policy representation and test installation against frozen readers.
7. Specify and exercise the confidence procedure, including the diagnostic counterexample.

Each change should preserve the current constraints it builds on. For example, preventing a consumer from running does not authorize freeing resources while other accepted work remains; accepting a new policy identifier does not admit its checker automatically.

## 3. Required closure record

For each finding, record its ID, affected profile, accountable owner, chosen correction, revised specification links, implementation commits, input/tool/profile digests, tests/proofs, observed results, excluded scope and reviewer. Distinguish **specified**, **implemented** and **qualified**.

A source-level limitation can be closed by a proved/enforced exclusion for a scoped profile, while remaining open for full replacement. A textual clarification can resolve a contradiction, but it does not prove the runtime or pass correct. Keep mandatory legacy rows visible through G-REPLACEMENT.

Return to the [finding index](../README.md) for the individual closure cases.
