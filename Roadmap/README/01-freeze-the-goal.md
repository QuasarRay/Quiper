# Freeze the decoupling and replacement claim

**Milestone M01.** A release-scope decision separating portable qualification, full replacement and assurance.

## Required inputs and specification

Start from the stated user requirements and the pinned implementation baseline. Use the exit-gate dependencies in [the implementation plan](../milestones.json); proof and investigation work may begin earlier. Read [the declarative specification](../Specification/README.md) before choosing representation details. This milestone implements `Extension.decoupled_addition`, `Qualification.full_replacement`.

## Required result

1. Remove CUDA as a required representation, compiler toolchain, runtime API, and installation dependency for the supported production profiles.
2. Preserve the semantics of each supported Kuiper program, including its memory ownership, synchronization, numerical contract, and host/device interaction. A backend must reject requirements it cannot satisfy.
3. After the extension interfaces are frozen, adding a compatible GPU backend must require **zero edits to existing source, existing build manifests, central registries, frontends, kernels, or other backends**. Addition of a separately built package and deployment configuration must be sufficient.
4. Make both ends of extraction independent: new input languages can produce the common contract; new host languages can consume compiled kernels and host execution plans. Neither endpoint must pass through CUDA C++, Rust source, or F* syntax.
5. Make production support a claim about a named profile, release, device, driver, and evidence set. Passing one example cannot qualify an entire backend.
6. Keep the implementation modular without replacing useful existing infrastructure. Reuse Kuiper's specifications and proofs, SPIR-T's IR and transformations, and existing drivers and validators. Keep custom machinery concentrated at the boundaries they do not supply.

The no-edit requirement is testable **within the semantics and protocol of a supported contract version**. No design can honestly guarantee that every future hardware feature or incompatible memory model fits an interface designed today. New expressible features use additive extension packages; incompatible semantics require a separately versioned contract. Existing supported modules must continue to work with their original contract. See [the extension rules](../03-backend-extension-contract/README.md).

## Architecture decision

Use a small, versioned **Kuiper Interchange Representation (KIR)** for the extraction boundary, with separate kernel bodies, host orchestration, resource layouts, requirements, and evidence. Use SPIR-T as the internal device transformation representation behind an adapter. Do not expose SPIR-T's Rust data structures as the public interchange or plugin ABI.

The first CUDA-independent execution path is KIR → SPIR-T → Vulkan-compatible SPIR-V → Vulkan compute. This is a concrete engineering choice based on SPIR-T's current shader-oriented implementation. It does not make SPIR-V mandatory for every backend. A direct SPIR-T → Mesa NIR path is a separate, measured development track, with its own driver integration and correctness work. See [the lowering plan](../04-spirt-and-gpu-lowering/README.md).

Retain the old CUDA path during migration as a comparison implementation. It becomes an optional package. CUDA removal from the default product occurs only when the declared replacement scope passes the release gates; unsupported legacy features remain visibly unsupported or supported only by an explicitly selected legacy package.

## Definition of completion

A declared portable release is qualified when the following conditions hold. Full CUDA replacement additionally requires G-REPLACEMENT over every mandatory row in `legacy-scope.json`; deferred rows cannot satisfy that gate:

- Every migrated primitive has a semantic specification, implementation, target requirement, test, and evidence classification.
- Supported kernels and their host plans run without the CUDA toolkit, CUDA runtime, NVCC, or Karamel in the new extraction path.
- At least two materially different real GPU environments pass the portable profile; the second runtime backend is independently installed and passes the no-edit test.
- A second non-F* input adapter and two host-language bindings use the same artifact contract. Their verification claims remain accurate.
- Required proofs and validation checks pass against the exact shipped artifacts; remaining trusted assumptions are explicit.
- The clean-install, failure-recovery, performance, compatibility, packaging, and rollback gates pass.

Vulkan on two vendors establishes useful hardware portability. It does **not** by itself demonstrate that a genuinely different compiler/runtime backend can be added without changing the core. Both tests are required.

## Evidence required to close this milestone

Close **P0** only with the implementation artifacts, positive and negative cases, and source-to-result identities required above. Link the implementation relation to the named F* symbols and the policy's applicable O1–O10 obligations. The specification's proved lemmas are reusable model facts; they do not discharge this implementation correspondence. Record unresolved cases as blockers or explicitly outside the claim. No backend gate is marked passed by this roadmap revision.
