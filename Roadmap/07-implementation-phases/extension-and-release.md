# Extension and release

## P7. Prove the architecture is additive and language-independent

**Depends on:** P4 and the P2 language/host boundary; can overlap P5/P6.

1. Freeze/hash a release-candidate core binary and read-only source checkout.
2. Build an out-of-tree second compiler/runtime package. Prefer a narrowly scoped Metal backend using qualified SPIR-V → MSL translation and a native runtime adapter; select it during P0 feasibility work.
3. Run the same portable corpus and host bindings through that package. Add a separate frontend/host adapter and a checked semantic extension.
4. Verify no core/frontends/existing kernels/registries/root build manifests changed. Verify generic build/test discovery works.
5. If the exercise requires an interface change, update the provisional contract, freeze again, and repeat with an independent package. Do not waive the requirement because the change is small.

**G-ADD-COMPILER, G-ADD-RUNTIME, G-ADD-LANGUAGE, G-ADD-OP:** all pass with candidate-bound hashes, dependency closures, and diffs. Include compiler-only and runtime-only substitutions for a shared target-artifact contract in addition to the distinct-API package. A simulator or another Vulkan device is useful evidence but does not replace the distinct runtime/API gate.
## P8. Qualify, package, cut over, and support

**Depends on:** P5, P6, P7.

1. Complete the production matrix, soak tests, performance budgets, error-path exercises, clean installation, and compatibility corpus.
2. Ship separate portable/compiler/runtime/binding packages with reproducible provenance and dependency/license inventories.
3. Publish exact supported devices/drivers/profiles and assurance levels. Freeze v1 and document upgrade/rollback policy.
4. Run canaries with real workloads. Make the new path the default only after the gates pass.
5. Keep CUDA as an optional legacy/reference package; remove it from default discovery/build/install requirements. Retire compatibility code only under a separate, evidence-backed deprecation decision.

**G-PRODUCTION, G-CUDA-FREE:** release matrix and rollback rehearsal pass for the exact candidate; unsupported scope is explicit; no default path accidentally resolves CUDA components. **G-REPLACEMENT** additionally requires CUDA-independent coverage of every mandatory legacy-scope row. A scoped portable release cannot satisfy that gate by deferring rows.
