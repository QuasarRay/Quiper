# Architecture

## 2. Architecture decision

Use a small, versioned **Kuiper Interchange Representation (KIR)** for the extraction boundary, with separate kernel bodies, host orchestration, resource layouts, requirements, and evidence. Use SPIR-T as the internal device transformation representation behind an adapter. Do not expose SPIR-T's Rust data structures as the public interchange or plugin ABI.

The first CUDA-independent execution path is KIR → SPIR-T → Vulkan-compatible SPIR-V → Vulkan compute. This is a concrete engineering choice based on SPIR-T's current shader-oriented implementation. It does not make SPIR-V mandatory for every backend. A direct SPIR-T → Mesa NIR path is a separate, measured development track, with its own driver integration and correctness work. See [the lowering plan](../04-spirt-and-gpu-lowering/README.md).

Retain the old CUDA path during migration as a comparison implementation. It becomes an optional package. CUDA removal from the default product occurs only when the declared replacement scope passes the release gates; unsupported legacy features remain visibly unsupported or supported only by an explicitly selected legacy package.
