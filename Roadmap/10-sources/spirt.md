# Spirt

## SPIR-T evidence

All S-links refer to SPIR-T commit `e8757adba8d14068a7bf1b3bc9f24cac982f4bd3`. Its package manifest reports version `0.4.0`; the commit identity is the audit anchor.

| ID | Source | What was used |
|---|---|---|
| S1 | [README](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/README.md) | Shader orientation, evolving scope, OpenCL/text-parser exclusions, available facilities |
| S2 | [IR definitions](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/lib.rs), [Cargo.toml](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/Cargo.toml) | Public construction, SPIR-V-oriented variants, context ownership, safe-Rust policy, dependency/version boundary |
| S3 | [SPIR-V module](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/spv/mod.rs), [legalization](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/passes/legalize.rs) | Conversion terminology, representation, structurization entrypoint |
| S4 | [QPtr model](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/qptr/mod.rs), [layout configuration](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/qptr/layout.rs), [QPtr example](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/examples/spv-lower-link-qptr-lift.rs) | Pointer/provenance assumptions, extent widths, configuration, illustrative pass sequence |
| S5 | [Repository tree](https://github.com/Rust-GPU/spirt/tree/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3), [.gitmodules](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/.gitmodules) | Inspected implementation scope and SPIRV-Headers dependency |

“No direct Mesa emitter found” describes this inspected tree. It is not a claim that no prototype exists anywhere in the ecosystem or in uninspected branches.
