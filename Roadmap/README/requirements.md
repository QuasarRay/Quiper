# Requirements

## 1. Required result

1. Remove CUDA as a required representation, compiler toolchain, runtime API, and installation dependency for the supported production profiles.
2. Preserve the semantics of each supported Kuiper program, including its memory ownership, synchronization, numerical contract, and host/device interaction. A backend must reject requirements it cannot satisfy.
3. After the extension interfaces are frozen, adding a compatible GPU backend must require **zero edits to existing source, existing build manifests, central registries, frontends, kernels, or other backends**. Addition of a separately built package and deployment configuration must be sufficient.
4. Make both ends of extraction independent: new input languages can produce the common contract; new host languages can consume compiled kernels and host execution plans. Neither endpoint must pass through CUDA C++, Rust source, or F* syntax.
5. Make production support a claim about a named profile, release, device, driver, and evidence set. Passing one example cannot qualify an entire backend.
6. Keep the implementation modular without replacing useful existing infrastructure. Reuse Kuiper's specifications and proofs, SPIR-T's IR and transformations, and existing drivers and validators. Keep custom machinery concentrated at the boundaries they do not supply.

The no-edit requirement is testable **within the semantics and protocol of a supported contract version**. No design can honestly guarantee that every future hardware feature or incompatible memory model fits an interface designed today. New expressible features use additive extension packages; incompatible semantics require a separately versioned contract. Existing supported modules must continue to work with their original contract. See [the extension rules](../03-backend-extension-contract/README.md).
