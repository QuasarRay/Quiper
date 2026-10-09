# Pipeline

## 1. First production path

Implement a Vulkan compute backend first. Use the proposed Vulkan 1.2 / SPIR-V 1.5 profile in the direct-construction procedure; ratify its exact optional features and device matrix in P0, record its digest, and validate against it. Do not target an unspecified “latest SPIR-V.” Optional features and extensions must be individually queried and enabled. [E3](../10-sources/README.md)

The backend worker reads KIR, constructs SPIR-T directly through a pinned adapter, applies a reviewed pipeline, emits SPIR-V, validates it, prepares reflection/layout metadata, and packages it for the Vulkan runtime. The current upstream `spv::lower` name means SPIR-V → SPIR-T; `spv::lift` is the reverse. Keep that terminology unambiguous in implementation. [S2–S3](../10-sources/README.md)

A serialized SPIR-V detour on input is useful for differential/round-trip tests but is not required for direct construction. A particular backend may emit SPIR-V on output. Neither statement implies SPIR-T is itself a hardware execution API.
## 2. Required lowering sequence

1. Validate KIR and accepted evidence; resolve imports and required semantic extensions.
2. Fix the program's semantic profile and target constraints before optimizations.
3. Specialize kernel parameters, layouts, workgroup shapes, and supported numeric operations. Retain a checked relation to the unspecialized contract.
4. Construct SPIR-T values, regions, globals, types, and exports using the adapter's explicit mapping. Preserve operation IDs and source/evidence correspondence.
5. Normalize control flow and calls. Structurize only when the transformation preserves exits, side effects, convergence, and barriers; reject unsupported control flow.
6. Legalize memory and pointers. Use `QPtr` where its analyzed subset fits; do not make successful legalization a substitute for provenance/bounds validation.
7. Apply approved optimizations with preconditions and evidence bookkeeping. Keep FP-sensitive rewrites behind the requested numeric mode.
8. Lower target operations and remove all unresolved custom operations. Finalize entrypoints, interfaces, bindings, memory model, capabilities, extensions, and execution modes.
9. Emit and validate SPIR-V; cross-check reflection against KIR layouts and runtime dispatch metadata.
10. Record all tool/pass versions, flags, requirements, specialization choices, and output hashes. Publish the cache entry atomically only after all mandatory checks succeed.

Pass ordering is a contract. Do not blindly copy an example's `QPtr` sequence or layout constants: the inspected example is exploratory and even leaves its final write disabled. [S4](../10-sources/README.md)
