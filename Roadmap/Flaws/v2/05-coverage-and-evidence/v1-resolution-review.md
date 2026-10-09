# Review of the 21 v1 corrections

V2's [resolution matrix](../../README/resolutions.md) records documentation corrections and leaves implementation evidence pending. This audit preserves that history. It does not relabel an unexecuted closure test as passed or silently rewrite the original findings.

| V1 finding | Review of the v2 correction | Additional v2 result |
|---|---|---|
| F01: replacement scope | Mandatory legacy coverage and a separate completion gate are specified | Correction holds; no replacement implementation was tested |
| F02: assurance minimum | Three fixed policies and O1–O10 requirements now bound claims | [V2-07](../03-contracts-and-evidence/v2-07-policy-extensibility.md) exposes the new policy/closed-schema interaction |
| F03: safety exceptions | Only bounded performance waivers remain permitted | Correction holds |
| F04: evidence identity | Exact candidate/input closure, provenance and rerun rules are specified | Correction holds; future evaluator remains unexecuted |
| F05: second API timing | Selection and hardware feasibility are P0; implementation is P7 | Correction holds |
| F06: performance decisions | Ratios, thresholds, pairing, intervals and limited remeasurement are specified | [V2-08](../04-release-measurement/v2-08-tail-confidence.md) adds tail-adequacy and resampling-resolution obligations |
| F07: ID annotations | Literal baseline and separate patched-profile qualification are explicit | Correction holds as a restriction; no patch is qualified |
| F08: loop report | Affected shapes remain blocked pending reproduction, fix or justified exclusion | Correction holds as a triage rule; [V2-04](../02-spirt-and-vulkan/v2-04-loop-exit-values.md) is a separate pinned representation gap |
| F09: QPtr subset | Calls, merges, memory operands and 32-bit ranges receive explicit eligibility checks | Correction holds; no legalization corpus was run |
| F10: upstream reuse | Baseline choice, candidate decisions, pins and recheck requirements are recorded | Correction holds; refreshed metadata is not adoption evidence |
| F11: erasure hook | Actual fork function and pre-erasure trace/correspondence work are identified | Correction holds as a feasibility procedure; build/proof still pending |
| F12: foreign callers | Static, dynamic, caller-evidence and trusted obligations are separated | [V2-02](../01-runtime-and-ownership/v2-02-forgotten-borrows.md) adds forgotten-token retention to the safe Rust contract |
| F13: host imports | Contract identity, effects and permitted assurance are required | Correction holds; arbitrary callbacks do not inherit the strong policy |
| F14: host code generation | Interpreter/bindings are mandatory; native compilation is conditional | Correction holds |
| F15: independent roles | Compiler and runtime substitutions are separate from the distinct-API test | Correction holds for those axes; [V2-07](../03-contracts-and-evidence/v2-07-policy-extensibility.md) adds a policy-extension case |
| F16: abort boundary | Recoverable errors/unwinds and application-fatal abort are distinguished | Correction holds |
| F17: provider lifetime | Exact instances remain pinned by resources, callbacks and destructors | Correction holds for provider lifetime; it does not establish caller allocation retention in V2-02 |
| F18: CI admission | Reviewed identities and protected runner/signing authority are separate | Correction holds |
| F19: unknown acceptance | Session IDs, operation digests, deduplication, queries and fail-stop restart are specified | [V2-03](../01-runtime-and-ownership/v2-03-operation-retirement.md) adds bounded retirement; [V2-01](../01-runtime-and-ownership/v2-01-dependent-failure.md) adds failure-dependent execution |
| F20: noncoherent overlap | Physical footprints and atom-separated/ordered alternatives are specified | Correction holds; hardware qualification remains pending |
| F21: assertions and guards | Logical distinction, collective-safe failure and output poisoning are specified | [V2-01](../01-runtime-and-ownership/v2-01-dependent-failure.md) adds already submitted consumers of a failed result |

V2-05 and V2-06 add target-feature and host-object synchronization detail beyond the earlier findings. The eight new records identify their precise scope; they do not assert that every v1 correction failed.

When the roadmap is revised again, link each correction to both its specification change and its implementation evidence. Do not mark a residual case qualified merely because its predecessor's documentation status says corrected.
