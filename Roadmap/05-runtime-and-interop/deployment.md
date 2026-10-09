# Deployment

## 6. Compilation, cache, and package lifecycle

Cache keys include normalized KIR, evidence policy, contract and semantic digests, backend/compiler versions, pass sequence, target/device features, numeric policy, specialization, ABI layouts, and relevant driver compatibility identity. Driver-native caches need stricter device/driver compatibility than portable SPIR-V.

Use bounded cache storage, atomic writes, corruption checks, and concurrent-reader/writer handling. Never reuse evidence after a source, operation definition, numeric flag, or backend lowering changes. Distinguish reproducibility of frontend artifacts from implementation-dependent driver pipeline caches.

Ship the portable package separately from the compiler and runtime packages. Applications that only load precompiled target packages should not need F*, SPIR-T, Rust tooling, OCaml, Karamel, or a shader compiler installed at runtime unless a selected backend explicitly requires compilation there.

Provide clean offline installation from pinned artifacts, dependency/license inventory, diagnostic inspection without a GPU, and uninstall/rollback without modifying unrelated packages.
## 7. Failure and operational tests

Inject allocation exhaustion, pipeline compile errors, malformed packages, unsupported features, queue submission errors, worker crashes, host cancellation, timeouts, device loss, corrupted cache entries, and shutdown during in-flight work. Some device-loss behavior needs dedicated hardware or a controlled harness; report simulated and observed evidence separately.

Expose structured stage timings, cache hits, chosen device/profile, fallback reasons, queue waits, memory use, and driver error details. Do not log full proprietary kernels or input buffers by default. Debug bundles should be opt-in and include enough hashes/configuration to reproduce failures without unnecessary application data.

For a service deployment, validate input/package sizes and resource budgets before expensive compilation or allocation. Trust boundaries must include native plugins, compiler workers, driver/kernel interfaces, and application callbacks. Memory-safe core code does not make arbitrary device execution safe by itself.
