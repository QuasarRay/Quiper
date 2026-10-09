# Compiler protocol

## 3. Compiler boundary

Use a versioned request/response protocol between the orchestrator and compiler workers. Requests carry KIR bytes/digests, the requested target profile, optimization policy, evidence policy, specialization values, and immutable dependency identities. Responses carry artifacts, complete requirements, diagnostics, provenance, and checked evidence results.

Each compiler worker privately owns its SPIR-T context and links a pinned `compiler/spirt` adapter SDK. Its vendor emitter can operate directly on SPIR-T within that worker. **SPIR-T objects do not cross the stable process boundary.** This permits independently built workers to use different compatible SPIR-T versions without recompiling the core.

Shared passes can be delivered as version-pinned SDK packages inside a worker. An independently installed pass that crosses process boundaries must use a specified KIR form or another explicitly versioned, validated representation. Do not invent a stable SPIR-T binary format by serializing its current Rust internals.

The protocol needs cancellation, bounded messages, streaming of large artifacts, deterministic request IDs, progress, and structured errors. A worker crash is a compilation failure; it cannot leave a successful partial package in the cache. Process isolation also matches SPIR-T's current context ownership constraints. [S2](../10-sources/README.md)
