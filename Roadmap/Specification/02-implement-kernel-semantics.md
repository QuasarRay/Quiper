# S2. Implement kernels and operation definitions

**Deliverable:** a typed neutral program reader and independently specified evaluator whose accepted behaviors match the declarative kernel and operation relations. Then connect the F*/Pulse exporter to that boundary.

## Implement the defined scalar and control fragment first

`Kernel.expression` defines literals, variables, U32 addition and comparison. `eval` returns `None` for unavailable variables or ill-typed operation inputs. The surrounding KIR checker must reject malformed literals and all uninitialized/ill-typed uses; `Literal` intentionally represents raw syntax and is not intrinsically typed. `Kernel.executes` specifies successful `Assign`, `Load`, `Store`, sequence, selection, while and explicit extension invocation.

For a load/store, establish permission, source value/type and logical location before applying the constructor. The authority relation includes allocation extent, generation and aliasing rules from S1. `write_frame` gives the unchanged-memory property outside one cell. Provide a typing environment and effect system for the concrete SSA KIR, then prove their checked invariants imply these semantic premises. Do not confuse representing an operation with proving it safe.

Map SSA bindings, block/region arguments and source variables to the abstract environment. Keep a correspondence for every definition and use. Prove the entry-state relation, each instruction relation, branch selection, and exit-state relation. Source locations aid diagnostics but do not establish correspondence. Selective inlining, specialization or phi/region conversion must update the relation and its evidence identities.

The while relation tests before its first body, and tests again after each body. `no_iteration` constructs the zero-iteration derivation. Positive derivations compose body and remaining-loop traces. It is a finite successful-execution relation: it does not prove loop termination or classify every rejected/stuck/divergent prefix. The complete implementation contract must add those observations and discharge `prefix_safety` and `progress`; do not use absence of a successful derivation as permission to crash.

## Define every operation family

| Family | Declarative contract and implementation requirement |
|---|---|
| Integers and casts | `well_typed`, modular add/subtract/multiply, `signed_division` and `shift_pre`; define bitwise, comparison and conversion cases under identified operation definitions, including overflow, zero division and oversized shifts |
| Vectors and views | Lane-wise relations, lengths, alignment and `subview`; vector packing cannot enlarge permissions or change scalar semantics |
| Private/global/shared memory | Initialized values, typed loads/stores, frame properties, storage identity and global reservation compatibility; shared storage is per workgroup |
| Atomics | `compare_exchange`/`atomic_add` value relation plus `Memory.admissible` with the exact type/storage/scope/order tuple; weak CAS/spurious failure needs its own relation |
| Barriers | Every member reaches the same dynamic collective instance, plus the required storage-class ordering; a memory fence is not a control barrier |
| Subgroups | `shuffle_relation`, actual subgroup size, source-lane membership and convergence; do not retain an implicit 32-lane assumption |
| Floating and approximations | A complete numerical policy with a total legal result relation; preserve NaNs, signed zero, infinities, subnormal/rounding controls and error bounds as specified |
| Matrix/tensor operations | `exact_gemm` for mathematical integer behavior, or an identified `matrix_policy` for mixed precision/rounding; dimensions, layouts, input/product/accumulator types and participation are independent requirements |
| Assertions and guards | `assert_pre` already holds before erasure; `guard_relation` supplies success only when its predicate holds; failure suppresses invalid effects and dependent execution |
| Host and foreign services | S3 and `Refinement.host_import`; effect, retained-resource, error and reentrancy behavior is part of the contract |
| User extensions | `operation_definition`, a nonempty result witness on legal inputs, typed inputs/results, an explicit frame/event relation and an admitted checker; names alone supply none of these |

For each operation definition, prove legal input/output typing, definedness on its precondition, permitted effects, frame preservation, numerical meaning, participation and progress. Keep pointer-heavy or special hardware families as explicit profiles when the initial logical-buffer subset cannot represent them. A qualified subset is useful; it is not full replacement.

## Export verified source without losing facts

Capture source typing, effects, layout, bounds, caller obligations and proof correspondence before ghost erasure. Follow [M08](../02-language-independent-extraction/02-export-a-checked-kuiper-program.md) for the pinned typed Pulse extraction hook and its feasibility spike. Classify static specialization, executable data and proof-only data independently. Bind the executable KIR and proof sidecar to the same semantic package; source names or an F* `.checked` filename are insufficient.

Use `Refinement.refines` to state the exporter relation and `refinement_transitive` to compose justified stages. `implements` additionally rules out an empty implementation relation on legal input. An implementation that only rejects valid supported calls must fail the source behavior relation; the source contract must not allow arbitrary rejection merely to make the theorem easy.

## Acceptance evidence

Close G-EXTRACT only for the recorded source subset. Include positive/negative type and effect fixtures, 0/1/multiple loop iterations, exceptional numerical inputs, uninitialized state, invalid views and unknown operations. Compare an actual Kuiper export and a frontend-free package with the independent reference. Source verification alone does not discharge O1/O2 or the later SPIR-T bridge.
