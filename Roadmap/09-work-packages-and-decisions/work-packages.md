# Work packages

## 1. Suggested review units

These are implementation work packages, not automatically created GitHub issues or commitments by named people. Each change should include its contract, implementation, focused tests, evidence impact, and rollout effect. Use stacked changes when dependencies are real; avoid one branch that rewrites the entire system before it can be exercised.

| ID | Work package | Existing touchpoints | Acceptance evidence |
|---|---|---|---|
| W01 | Primitive/host/evidence inventory | `extraction/`, `src/`, `include/`, generated instantiations | Every extraction/default/foreign case classified |
| W02 | Reproducible baseline | `verify.mk`, `nvcc.mk`, tests/benchmarks/packaging | Pinned logs and workload results |
| W03 | KIR grammar, canonical encoding, semantic profiles | New `contracts/`, semantic specifications | Independent reader agreement; invalid inputs rejected |
| W04 | Discovery and compiler/runtime protocols | Build entrypoints plus new generic orchestrator | Synthetic package installation without registration edits |
| W05 | Typed F* manifest capture | Pinned F* hook and `extraction/` | Static/ghost/runtime classification survives erasure |
| W06 | F* executable KIR export | Current primitive handlers | No CUDA names in the contract; baseline corpus extracts |
| W07 | KIR reference semantics and checker | New validation tools | Specified supported subset and independent test oracle |
| W08 | SPIR-T construction SDK | New `compiler/spirt/` | Typed/control/layout/evidence mapping tests |
| W09 | SPIR-V target emitter | SPIR-T adapter, target validator | Explicit environment, valid modules, reflection agreement |
| W10 | Vulkan runtime basics | New backend package | Real allocation/dispatch/result/cleanup path |
| W11 | Shared memory and layout | `Kuiper.SHMem`, array/views, current helpers | Alignment/layout refinement plus limit/overflow tests |
| W12 | Barriers, subgroups, atomics | Barrier/AtomicOps contracts, runtime | Participation and memory-model obligations resolved |
| W13 | Host plans, queues, epochs | `Kernel.Base`, streams/epochs, async examples | Ordered dependency chains and failure ownership |
| W14 | C/Rust host bindings | New binding packages | Same artifact bytes; lifetime/error/ABI tests |
| W15 | Second source adapter | Independent restricted typed frontend | KIR contract conformance; correct assurance labeling |
| W16 | Numeric policy and math | Float modules and extraction mappings | Exact/approximate tests and evidence per operation |
| W17 | Matrix/vendor extensions | TensorCore/WGMMA interfaces and headers | Qualified shapes/layouts/relations, explicit unsupported cases |
| W18 | Pass/evidence validation | All transformation boundaries | O1–O10 evidence and invalidation behavior |
| W19 | Distinct out-of-tree backend | Independent compiler/runtime package | Frozen-core hashes unchanged; real workload runs |
| W20 | Additive frontend/operation exercise | Public SDK and contracts only | No existing source or build edits |
| W21 | Packaging and CI generalization | Workflows, configure, package scripts | Discovered backend matrix, clean CUDA-free install |
| W22 | Qualification and cutover | Release tooling and support matrix | Performance, soak, canary, rollback, complete claim report |
| W23 | Direct Mesa experiment | Separate version-pinned package | Same-driver comparison and promote/defer decision |

W04's synthetic package exercises discovery only; it does not satisfy W19's real backend gate. W07's evaluator exercises sequential semantics only until concurrency is explicitly modeled. These distinctions must remain in progress reports.
