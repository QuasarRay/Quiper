# Declarative F* specification and implementation milestones

This directory contains actual F* source for the portable semantic contract, together with implementation instructions and a reproducible verification record. The model uses logical values, resources, relations and observations. It does not choose Rust object layouts, wire encodings, device pointers, a source language or a GPU vendor. `Lowering` is a separate realization contract for the investigated SPIR-T/Vulkan route.

The specification covers kernel evaluation and structured execution, operation families, logical permissions and physical footprints, concurrent-event constraints, host resources, asynchronous operation states, backend composition, additive installation, evidence admission and release decisions. Its extensible parts take explicit semantic relations as parameters. A concrete profile must define those relations and prove their adequacy; a plugin cannot fill them with an arbitrary predicate and inherit a verified claim.

## Implement the specification in this order

1. [S1: Freeze the semantic domain and profile](01-freeze-the-semantic-domain.md).
2. [S2: Implement kernels and operation definitions](02-implement-kernel-semantics.md).
3. [S3: Implement resource and runtime transitions](03-implement-runtime-semantics.md).
4. [S4: Implement additive composition and evidence admission](04-implement-additive-composition.md).
5. [S5: Establish the SPIR-T realization relation](05-establish-spirt-refinement.md).
6. [S6: Replay proofs and check correctness evidence](06-replay-proofs-and-evidence.md).

The [implementation milestone inventory](../implementation-milestones.json) connects these relations to all 31 active roadmap deliverables. [V2 resolutions](../Flaws/v2/resolutions/README.md) connect the eight documented flaws to corrected instructions, model facts and remaining implementation tests.

## Source modules

| Module | Declarative meaning |
|---|---|
| [Foundation](modules/Quiper.Spec.Foundation.fst) | Logical generations, values, views, permissions, bytes and frame properties |
| [Kernel](modules/Quiper.Spec.Kernel.fst) | Expression evaluation and inductive successful execution, including while loops |
| [Memory](modules/Quiper.Spec.Memory.fst) | Event ordering, races, collective participation, numerical-policy obligations and physical footprints |
| [Operations](modules/Quiper.Spec.Operations.fst) | Complete family taxonomy, operation contracts, integer/atomic/guard/subgroup/matrix relations and catalog coverage |
| [Host](modules/Quiper.Spec.Host.fst) | Allocation, reservation, quiescent resolution, free, snapshot copy and scoped retention |
| [Runtime](modules/Quiper.Spec.Runtime.fst) | Acceptance window, success-dependent submission, failure, observation, redemption, retirement and host ownership |
| [Refinement](modules/Quiper.Spec.Refinement.fst) | Observable behaviors, trace inclusion, nonempty semantics, O1–O10, prefix safety, progress and host import contracts |
| [Extension](modules/Quiper.Spec.Extension.fst) | Role compatibility, capability tuples, immutable semantic registration, policy/evidence admission and protected installation |
| [Lowering](modules/Quiper.Spec.Lowering.fst) | SPIR-T loop-value channels, target layout and Vulkan feature obligations |
| [Qualification](modules/Quiper.Spec.Qualification.fst) | Conservative interval decisions, exact-candidate gates and full replacement coverage |
| [All](modules/Quiper.Spec.All.fst) | Dependency root and specification version |

## What correctness evidence exists

The [verification record](evidence/fstar-verification.json) records strict F* verification separately for every module, with file and compiler hashes. The [source lock](sources.json) records the official compiler archive and solver identity. No development admits, assumed declarations, lax checking or unsafe coercions are used in these modules. The runner checks each module explicitly; checking only the empty dependency root would not establish that every dependency's proof obligations ran.

The proved lemmas establish the stated model facts. They do not prove that this model captures every intended Kuiper program, that an unspecified semantic extension is sound, or that a future implementation refines it. Full IEEE floating semantics, a selected GPU axiomatic memory relation, typed-source extraction, complete loop lowering, prefix safety and progress must be instantiated and discharged for the claimed profile. These are explicit implementation obligations, not hidden axioms or completed proofs. No GPU, compiler-backend or production gate has been run by this revision.

The official F*, Pulse, Kuiper and SPIR-T references and their exact evidentiary limits are in [S6](06-replay-proofs-and-evidence.md). References justify the modeling choices; the verification log supplies evidence for the checked lemmas. Neither replaces implementation correspondence.
