# 08. Production acceptance

## 1. Qualify explicit release profiles

Use a per-feature/per-backend support matrix with statuses: unimplemented, experimental, tested, qualified. Record assurance separately as structural, source-verified, translation-validated, and the applicable trusted assumptions. Do not collapse these dimensions into one green status.

The initial production scope should include a useful portable compute corpus and host execution. Advanced matrices, physical pointers, vendor instructions, external memory, and multi-device execution have separate profiles. All legacy feature rows must still receive a disposition, including an explanation for deferral.

| Release class | Minimum evidence |
|---|---|
| Experimental | Clear unsupported scope and structural validation; no production claim |
| Beta | End-to-end corpus on named devices, explicit errors, known limitations |
| Production | Complete profile matrix, real-device qualification, operational/performance/install/rollback gates |
| Verified profile | The above where claimed, plus the evidence policy in document 06; trusted boundaries remain explicit |

## 2. Hardware and platform matrix

For portable GPU qualification, require at least two materially different Vulkan implementations/vendors, preferably NVIDIA and AMD or Intel. For the additive architecture gate, require a genuinely different runtime/API such as the scoped Metal backend on Apple hardware. Neither a second Vulkan vendor nor a CPU interpreter substitutes for that second test.

Record device model, architecture, driver version, OS, kernel, API/extension versions, compiler/backend digests, subgroup properties, limits, and numerical modes. Test at least the declared minimum and a current supported driver configuration for each claimed platform. A driver upgrade changes qualification inputs and must trigger the relevant regression set.

Use the developer's CachyOS environment for local validation where applicable. Reproducible build environments and CI runners can differ, but support claims must name the environments actually tested. Do not require a local distro replacement to work on the project.

Software implementations are useful for CI, compiler validation, deterministic debugging, and fault simulation. Label that evidence as software execution. Tensor/matrix instructions, weak-memory behavior, hardware limits, watchdog behavior, and performance require relevant hardware evidence.

## 3. Correctness suites

| Suite | Required cases | Failure meaning |
|---|---|---|
| Contract/serialization | Round trips, canonicalization, version mismatch, duplicate keys, malformed lengths/IDs, unknown required extensions | Invalid package accepted or valid package corrupted |
| Frontend extraction | Erasure, specialization, generated instantiations, host branches/loops, unresolved primitives, diagnostics | Semantics lost or language-specific assumptions leaked |
| Compiler | Direct construction, structurization, import resolution, pass metadata invalidation, pointer/legalization boundaries | Incorrect transformation or false evidence retention |
| Memory/layout | Mixed types, vectors, slices, aliasing, alignment, offset overflow, shared regions, edge/tail sizes | Invalid access or proof-visible layout change |
| Concurrency | Workgroup/subgroup barriers, divergent participation rejection, atomics, visibility, cross-queue dependencies | Forbidden outcome, hang, or premature completion |
| Numeric | Exact modes, permitted approximations, NaNs, infinities, signed zero, subnormals, casts, reduction nondeterminism | Contract violation, even if ordinary inputs look correct |
| Runtime | Resource lifetime, repeated use, out-of-memory, cancellation, teardown, device loss | Use-after-free, leak, false success, corrupted ownership |
| ABI/bindings | Identical artifact loaded by C/Rust, asynchronous borrows, error cleanup, struct/scalar layouts | Host-language-dependent behavior or ABI mismatch |
| Extensibility | Installed immutable core, out-of-tree backend/frontends/extensions, old/new version combinations | Existing source/build/registry edit required |
| Distribution | Clean/offline install, dependency inspection, package rollback, cache corruption, upgrade | Undeclared dependency or unrecoverable deployment |

For property testing and fuzzing, mutate packages, layouts, primitive combinations, pass sequences within legal preconditions, and API call sequences. Minimize failures into checked-in regression cases. Bound parser/compiler resources so malformed input produces diagnostics rather than uncontrolled resource use.

Sanitizers, host memory/race tools, driver validation, and device diagnostics are complementary. None independently establishes full program correctness. Run them where they observe a real risk, rather than treating a long list of tools as evidence by itself.

## 4. Workloads that determine release usefulness

Migrate the existing example and Klas families, then test application-shaped compositions:

- Dense GEMM with tiling, transposition, varying dimensions, strides, and fused epilogues.
- Sparse matrix multiplication using varied sparsity distributions and boundary cases.
- Row softmax, log-softmax, reductions, and chained tensor transforms with declared numerical policies.
- Vectorized array views and slices, including tails and offset copies.
- Multiple dependent launches that retain GPU-resident intermediates and synchronize only when the host needs results.
- Repeated model-serving style batches: load once, prepare pipelines once, vary inputs, overlap allowed transfers/work, handle shutdown and failure.

Use source specifications/reference results, legacy CUDA comparisons where applicable, and metamorphic properties where valid. Do not use handwritten CUDA output as the sole oracle. Never regenerate expected output automatically to make a discrepancy disappear.

Track exact input corpora, generators, seeds, sizes, error metrics, and expected legal nondeterminism. Separate instruction-specific kernels from semantically portable algorithms so the coverage report is not inflated by silently replacing one with the other.

## 5. Performance gates

Measure GPU kernel time, host submission overhead, cold/warm compile and pipeline time, transfer time, end-to-end latency/throughput, peak device/host memory, and package size. Record warmup, repetitions, confidence intervals, clock/power/thermal conditions, queue depth, synchronization, and background load.

Compare against:

1. The legacy CUDA implementation on the same NVIDIA hardware for matching semantics and workloads.
2. An appropriate native baseline on other hardware.
3. The previous qualified release for regression detection.

Do not compare different FP modes, tensor paths, algorithms, or input shapes and attribute the difference solely to SPIR-T. Optimizations that increase compile time or memory need those costs reported alongside kernel speed.

**Proposed initial budgets to ratify in P0:** no more than 10% geometric-mean end-to-end regression over the agreed portable corpus; no individual critical workload more than 20% slower; no more than 10% regression in peak memory or warm host submission overhead relative to its selected baseline. Report cold compilation separately with an agreed absolute service/startup budget. These are planning targets, not measured results or universal guarantees.

Use confidence intervals and an agreed noise threshold before declaring regression. A geometric mean must not hide a severe tail-latency or correctness failure. Any exception must identify workload, cause, user-visible consequence, owner, and expiration/review milestone. Set budgets before examining release results; do not quietly move the target afterward.

Keep direct Mesa promotion separate: it needs a measured benefit over the same-driver SPIR-V route that justifies its maintenance cost. No assumed percentage gain appears in the release plan.

## 6. CI and evidence gates

Create separate required jobs for contract conformance, strict source verification, extraction, compiler validation, runtime/binding tests, backend qualification, no-edit installation, packaging, and release provenance. Generic backend discovery produces the relevant matrix from manifests; runner inventory is configuration.

Every skipped feature test must state the missing capability and be matched against the claimed support matrix. Skipped tests cannot count as evidence for a supported feature. Hardware failures, unavailable runners, and compiler errors remain distinct results.

Keep CPU-only checks fast enough for ordinary changes. Run GPU regression jobs on affected profile/backend combinations, then the complete claimed release matrix for a release candidate. Isolate resource-intensive fuzzing/soak tests from deterministic per-change checks. Cached proof/build outputs require complete identity checks.

Before release, run at least a 24-hour sustained workload/queue/resource-lifecycle soak on representative qualified devices, with no unexplained correctness failures, hangs, or growing resource leakage. This is a minimum proposed qualification exercise, not proof of indefinite reliability.

## 7. Release, canary, and rollback

1. Build immutable versioned packages with exact dependencies, notices, provenance, and support/assurance manifests.
2. Reproduce portable artifacts in a clean environment. Record any target/driver artifacts whose bytes are not reproducible and why.
3. Test an installation that has no CUDA toolkit/runtime, NVCC, or Karamel requirement for the new path. Inspect build and runtime dependency graphs, not only command names.
4. Run a canary on representative workloads with explicit backend selection, numerical policy, and monitoring.
5. Promote the new path only after all release gates pass. Preserve independent rollback of core, compiler worker, and runtime package within compatibility rules.
6. Rehearse disabling a bad backend, invalidating its cache, selecting the previous qualified package, and verifying application results.
7. Publish release notes containing qualified scope, assurance status, incompatibilities, deferred legacy features, regressions/waivers, and migration steps.

Retaining an optional CUDA reference package is compatible with CUDA-free default operation. A release that still needs CUDA for a feature must say so for that feature; it cannot claim complete CUDA replacement.

## 8. Final acceptance checklist

- [ ] G-BASELINE, G-CONTRACT, G-EXTRACT, G-VERTICAL, G-CONCURRENCY, and G-COVERAGE pass.
- [ ] O1–O10 have the required evidence/disposition for each assurance profile; G-TRUST passes.
- [ ] G-ADD-COMPILER, G-ADD-RUNTIME, G-ADD-LANGUAGE, and G-ADD-OP pass against the frozen core.
- [ ] Real-device/profile matrix and meaningful workload corpus pass with complete skip accounting.
- [ ] Performance budgets and soak/failure/cleanup tests pass or have explicit scoped exceptions.
- [ ] Clean installation, old artifact compatibility, upgrade, rollback, and cache integrity pass.
- [ ] G-PRODUCTION and G-CUDA-FREE pass; public support claims exactly match the evidence.
