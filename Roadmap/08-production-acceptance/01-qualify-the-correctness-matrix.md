# Qualify the declared hardware and correctness matrix

**Milestone M25.** A complete claimed feature/device matrix, adversarial failure suite and nonwaivable operational evidence.

## Required inputs and specification

Start from [M16](../04-spirt-and-gpu-lowering/04-complete-numerics-and-target-emission.md), [M19](../05-runtime-and-interop/03-qualify-the-vulkan-service.md). Use the exit-gate dependencies in [the implementation plan](../milestones.json); proof and investigation work may begin earlier. Read [the declarative specification](../Specification/README.md) before choosing representation details. This milestone implements `Operations.catalog_covers`, `Memory.admissible`, `Qualification.qualified`.

## Qualify explicit release profiles

Use a per-feature/per-backend support matrix with statuses: unimplemented, experimental, tested, qualified. Record assurance separately using the fixed `qualified-v1`, `source-verified-v1`, and `refinement-verified-v1` policies and their explicit assumptions; record individual translation-validation results as evidence, not as an unqualified end-to-end label. Do not collapse these dimensions into one green status.

The initial production scope should include a useful portable compute corpus and host execution. Advanced matrices, physical pointers, vendor instructions, external memory, and multi-device execution have separate profiles. All legacy feature rows must still receive a disposition, including an explanation for deferral.

| Release class | Minimum evidence |
|---|---|
| Experimental | Clear unsupported scope and structural validation; no production claim |
| Beta | End-to-end corpus on named devices, explicit errors, known limitations |
| Production | Complete profile matrix, real-device qualification, operational/performance/install/rollback gates |
| Verified profile | The above where claimed, plus the evidence policy in document 06; trusted boundaries remain explicit |
## Hardware and platform matrix

For portable GPU qualification, require at least two materially different Vulkan implementations/vendors, preferably NVIDIA and AMD or Intel. For the additive architecture gate, require a genuinely different runtime/API such as the scoped Metal backend on Apple hardware. Neither a second Vulkan vendor nor a CPU interpreter substitutes for that second test.

Record device model, architecture, driver version, OS, kernel, API/extension versions, compiler/backend digests, subgroup properties, limits, and numerical modes. Test at least the declared minimum and a current supported driver configuration for each claimed platform. A driver upgrade changes qualification inputs and must trigger the relevant regression set.

Use the developer's CachyOS environment for local validation where applicable. Reproducible build environments and CI runners can differ, but support claims must name the environments actually tested. Do not require a local distro replacement to work on the project.

Software implementations are useful for CI, compiler validation, deterministic debugging, and fault simulation. Label that evidence as software execution. Tensor/matrix instructions, weak-memory behavior, hardware limits, watchdog behavior, and performance require relevant hardware evidence.

## Correctness suites

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
## Workloads that determine release usefulness

Migrate the existing example and Klas families, then test application-shaped compositions:

- Dense GEMM with tiling, transposition, varying dimensions, strides, and fused epilogues.
- Sparse matrix multiplication using varied sparsity distributions and boundary cases.
- Row softmax, log-softmax, reductions, and chained tensor transforms with declared numerical policies.
- Vectorized array views and slices, including tails and offset copies.
- Multiple dependent launches that retain GPU-resident intermediates and synchronize only when the host needs results.
- Repeated model-serving style batches: load once, prepare pipelines once, vary inputs, overlap allowed transfers/work, handle shutdown and failure.

Use source specifications/reference results, legacy CUDA comparisons where applicable, and metamorphic properties where valid. Do not use handwritten CUDA output as the sole oracle. Never regenerate expected output automatically to make a discrepancy disappear.

Track exact input corpora, generators, seeds, sizes, error metrics, and expected legal nondeterminism. Separate instruction-specific kernels from semantically portable algorithms so the coverage report is not inflated by silently replacing one with the other.

## Evidence required to close this milestone

Close **G-COVERAGE, G-CONCURRENCY, G-PRODUCTION** only with the implementation artifacts, positive and negative cases, and source-to-result identities required above. Link the implementation relation to the named F* symbols and the policy's applicable O1–O10 obligations. The specification's proved lemmas are reusable model facts; they do not discharge this implementation correspondence. Record unresolved cases as blockers or explicitly outside the claim. No backend gate is marked passed by this roadmap revision.
