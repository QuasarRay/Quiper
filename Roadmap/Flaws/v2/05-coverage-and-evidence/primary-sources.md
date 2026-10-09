# Primary evidence used by the v2 audit

Official documents establish language/API rules. Pinned source establishes the selected implementation's behavior. Neither establishes correctness of the proposed KIR compiler/runtime merely by being cited.

## 1. F*, Pulse and Kuiper

| Primary source | Audit use | Limit |
|---|---|---|
| [F*: erasure and ghost effect](https://fstar-lang.org/tutorial/book/part4/part4_ghost.html) | Rechecked the distinction between executable values and erased proof content | Does not supply the proposed exporter's preservation proof |
| [Pulse extraction](https://fstar-lang.org/tutorial/book/pulse/pulse_extraction.html) | Rechecked existing extraction routes and extension context | Does not make the new neutral KIR contract an existing API |
| [Pulse basics](https://fstar-lang.org/tutorial/book/pulse/pulse_ch1.html) | Kept partial correctness distinct from guaranteed progress | No new termination claim is inferred |
| [Pulse atomic operations and invariants](https://fstar-lang.org/tutorial/book/pulse/pulse_atomics_and_invariants.html) | Source atomicity and invariant reasoning depend on justified target behavior | Does not prove the Vulkan memory-model mapping |
| [Project fork's Pulse.Extract.Main](https://github.com/QuasarRay/FStar/blob/0eef57bef411aac090354a75c21e00b674bd420c/pulse/src/checker/Pulse.Extract.Main.fst#L582-L592) | Re-read erasure followed by simplification, goto elimination and export | Fork-source evidence, not official upstream API equivalence |
| [Kuiper kernel launch contract](https://github.com/FStarLang/kuiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/src/lib/kuiper/Kuiper.Kernel.Base.fsti#L18-L38) and [epoch interface](https://github.com/FStarLang/kuiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/src/lib/kuiper/Kuiper.Epoch.fsti) | V2-01: conditional resource facts pass through dependent launches | Recoverable-failure propagation is new work |
| [Kuiper guard interface](https://github.com/FStarLang/kuiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/src/lib/kuiper/Kuiper.Assert.fsti) and [async chain](https://github.com/FStarLang/kuiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/src/examples/Kuiper.Example.Async.Chain.fst) | Checked the source distinction and existing chain use behind V2-01 | No device guard failure was executed |

## 2. SPIR-T, Vulkan and Rust

| Primary source | Audit use | Limit |
|---|---|---|
| [Pinned SPIR-T region/node definitions](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/lib.rs#L779-L920) and [RegionDef API](https://rust-gpu.github.io/spirt/spirt/struct.RegionDef.html) | V2-04: loop backedge outputs differ from selection/node results | Use the pin for implementation; live generated docs can change |
| [Pinned SPIR-V lifting](https://github.com/Rust-GPU/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/spv/lift.rs) | Checked how values are resolved during emission | Source inspection does not reproduce a miscompilation |
| [Vulkan 1.2 features](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVulkan12Features.html) and [memory model](https://docs.vulkan.org/spec/latest/appendices/memorymodel.html) | V2-05: chain support is a distinct feature; map actual semantic use | Does not establish which Kuiper lowering needs it without analysis |
| [Vulkan fundamentals](https://docs.vulkan.org/spec/latest/chapters/fundamentals.html) and [vkCmdDispatch](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatch.html) | V2-06: host accesses can require synchronization of implicit shared objects | The selected profile is Vulkan 1.2, not every extension in the live manual |
| [Vulkan synchronization](https://docs.vulkan.org/spec/latest/chapters/synchronization.html) | V2-01/V2-06: distinguish device dependencies from application success and host locking | API ordering does not prove a source postcondition |
| [Rust mem::forget](https://doc.rust-lang.org/std/mem/fn.forget.html), [Rustonomicon leaking](https://doc.rust-lang.org/nomicon/leaking.html), [thread::scope](https://doc.rust-lang.org/std/thread/fn.scope.html) | V2-02: destructor suppression and a scope-based ownership precedent | These are language/standard-library rules, not a tested GPU binding |

## 3. Evidence encoding and measurement

| Primary source | Audit use | Limit |
|---|---|---|
| [JSON Schema 2020-12 validation](https://json-schema.org/draft/2020-12/json-schema-validation) | V2-07: an enum admits only its listed values; considered structural versus semantic validation separately | No complete schema-engine or metaschema run is claimed |
| [SciPy bootstrap documentation](https://docs.scipy.org/doc/scipy/reference/generated/scipy.stats.bootstrap.html) | V2-08: interval methods and degenerate distributions require explicit handling | The audit's population example and multiplicity arithmetic are independent calculations |

The official live manuals were accessed during this audit; they are not frozen release inputs. P0 must select their applicable versions/snapshots together with tool and target profiles, as v2 already requires. The issue is not that a live reference exists, but whether its stated rule supports the specific instruction being implemented.
