# Implement the migration in this order

The result must be a reusable compiler/runtime system. Adding a supported backend after v1 freezes must add a package and deployment configuration while leaving the released core, existing frontends, kernels, bindings, root build manifests, and central CI dispatch unchanged. The first refactor can change those existing files to establish the boundary. Subsequent compatible additions cannot.

## 1. Commit to the result before implementing it

1. Use [replacement scope](../00-current-state-and-gaps/implementation.md) and `legacy-scope.json` to retain the existing extraction surface as mandatory migration work. A useful portable release is an intermediate deliverable. G-REPLACEMENT remains blocked while mandatory behavior requires CUDA.
2. Use [evidence policies](../06-verification-and-trust/evidence-policy.md) to decide exactly which claim is being built. A source proof, a tested backend, and a checked compilation relation are distinct fields.
3. Implement the [dependency boundaries](../01-target-architecture/implementation.md), [package protocol](../03-backend-extension-contract/implementation.md), and [KIR contract](../02-language-independent-extraction/implementation.md) before vendor code becomes a core dependency.
4. Complete the typed extraction spike and the SPIR-T feasibility probes before freezing any v1 representation. A failed probe blocks that feature; it does not justify silently weakening its semantics.

## 2. Build the first complete route

Build F*/Pulse → KIR → a private SPIR-T worker → Vulkan SPIR-V → the Vulkan runtime. Use the [direct-construction procedure](../04-spirt-and-gpu-lowering/direct-construction.md), [control/pointer procedure](../04-spirt-and-gpu-lowering/control-and-pointers.md), [emission procedure](../04-spirt-and-gpu-lowering/emission-and-validation.md), and [runtime procedure](../05-runtime-and-interop/vulkan-adapter.md). Run unchanged portable artifacts from C and Rust bindings.

SPIR-T provides a compiler representation and transformations. Its official API does not provide Kuiper's package protocol, host-plan engine, or a ready-made Vulkan runtime. Those components are specified here as new work. See the [claim-to-source map](../10-sources/claim-evidence.md) for the source of each dependency fact and the proof still required for the new integration.

## 3. Establish addition-only extension

Freeze an installed core and its full dependency/contract closure. Add an independently built compiler, an independently built runtime for a shared artifact contract, a distinct API backend, a new frontend/binding, and a semantic extension. Follow the [substitution tests](../03-backend-extension-contract/addition-only-tests.md). Package admission and trusted checker selection are explicit configuration; installing an extension never grants itself trust.

Passing the tests establishes extensibility for the declared contract and tested semantic coverage. Incompatible future semantics require a new contract version. Preserve old readers and behaviors; never reinterpret an old package under a new meaning.

## 4. Qualify the actual release

Evaluate candidate-bound evidence, real-device results, failure ownership, performance, clean installation, and rollback. Safety failures cannot be waived. Mark all implementation gates `not-run` until their recorded evidence exists. Official references justify the design constraints; they are not proofs that the proposed implementation is correct.

Start reading at [the roadmap index](README.md). All instructions use the repository's existing Kuiper name. Proposed paths and APIs are specifications to implement, unless a command or symbol is expressly identified as already present.
