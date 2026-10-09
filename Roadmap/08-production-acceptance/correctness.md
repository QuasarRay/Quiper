# Correctness

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
