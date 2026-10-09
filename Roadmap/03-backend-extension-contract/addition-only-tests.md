# Prove backend addition without existing source changes

## 1. Freeze the protected closure

Before P7, produce a manifest of the installed core binaries, contract readers, built-in validators, existing frontends/bindings, root build files/lockfiles, central workflow logic, and existing backend packages. Hash their content and source revisions. Make the checkout read-only and run the same exercise without a source checkout at all.

The permitted diff consists of a new package/install root and explicit deployment configuration selecting/admitting it. Cache and evidence outputs go to separate writable directories and do not replace installed components. Do not use a rewritten environment, a hidden regenerated registry, or a new core binary to pass the test.

## 2. Test each extension axis separately

| Exercise | New component | Components kept unchanged | Required outcome |
|---|---|---|---|
| G-ADD-COMPILER | Independent compiler worker producing the existing Vulkan artifact contract | Core, frontend, runtime, host bindings, portable corpus | Correct compilation/execution with checked requirements and evidence |
| G-ADD-RUNTIME substitution | Independent runtime consuming that same artifact contract | Core, compiler output bytes, frontend, host bindings | Same semantics and failure ownership on compatible devices |
| G-ADD-RUNTIME distinct API | Selected Metal or other real API package | Core, portable KIR/host contract, existing source and bindings | Same mandatory portable corpus through the second API |
| G-ADD-LANGUAGE | Third frontend and independently implemented host binding | Existing core/compiler/runtime packages | Correct KIR and shared artifact consumption with accurate assurance labels |
| G-ADD-OP | Semantic extension plus lowerer/checker support | Core, existing operation meanings and existing packages | Accepted with admitted support; rejected before execution without it |

The two runtime exercises are cumulative. Another Vulkan implementation tests substitution but does not replace the distinct-API requirement. A Metal runtime is not expected to load Vulkan binaries; format compatibility is checked where claimed.

## 3. Exercise more than the happy path

Use elementwise operations, views, bounded control flow, reductions/matrix kernels, and asynchronous chains in the declared common subset. Include malformed/unknown versions, insufficient capabilities, incompatible evidence policies, missing extensions, old package readers, duplicate package IDs, and conflicting registrations.

Install packages built by an independent implementer from the public specification. A renamed existing package or a paired compiler/runtime sharing undocumented assumptions is insufficient. Run compiler-only and runtime-only substitutions to expose that coupling.

## 4. Preserve feature expressiveness

Keep hardware-specific operations as namespaced extensions with explicit requirements. Add a test operation with side effects that the generic optimizer does not understand; it must preserve effects conservatively and route to the registered lowerer. If an old core cannot express required routing/effect/evidence behavior, fix the provisional contract and restart the freeze. After v1 release, use an explicitly incompatible contract version instead of changing old semantics.

## 5. Record final-candidate evidence

Store before/after protected hashes, filesystem diffs, package/tool identities, corpus digest, target/profile, environment, full pass plan, and outcomes using the [gate-result contract](../08-production-acceptance/gate-records.md). Any covered core/contract change invalidates the relevant exercise. P8 must use results for the actual release candidate, including changes made after an earlier P7 pass.
