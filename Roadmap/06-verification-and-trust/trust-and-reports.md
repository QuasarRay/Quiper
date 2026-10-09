# Trust and reports

## 7. Trusted computing base record

Publish a per-profile inventory covering the source checker/logic/solver, extraction adapter, unproved compiler passes or validators, serializer, SPIR-T bridge, target compiler/driver, runtime FFI, host binding, OS, firmware, and hardware behavior used in the claim.

Do not describe SPIR-T, Mesa, NVCC, a Vulkan driver, or all of F* as formally verified merely because a Kuiper kernel was checked. Specify what is proved, what is validated per compilation, what is tested, and what is assumed.

Full machine-code correctness is a separate goal unless the project actually implements a proof path through target code generation and the relevant driver/hardware semantics. This roadmap makes that boundary visible rather than using “verified” as a property of the entire software stack.
## 8. Verification release report

For every supported entrypoint/profile pair, publish source-proof status, admitted/axiomatic dependency closure, extraction relation status, enabled pass evidence, runtime/ABI evidence, unsupported features, hardware test identities, numerical policy, and known limitations. Missing evidence must be machine-readable and block a stronger assurance label.

The first release may expose several assurance levels, but the user-selected verified profile must enforce its stated evidence policy at compile/load time. A build flag must never silently downgrade that policy.
