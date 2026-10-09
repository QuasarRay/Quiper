# Current roadmap and machine-file coverage

The input set is the 85 current Markdown pages at `9dceaf274b46f295f7fc312fb3396d5729d7d97d`. Instruction pages received a semantic/consistency review within their topic and across dependent contracts. Topic indexes received navigation/scope checks. The table reports where findings apply; a row with no additional finding is not a correctness proof.

| Page | Review scope | Disposition |
|---|---|---|
| [00-current-state-and-gaps/README.md](../../../00-current-state-and-gaps/README.md) | Navigation, scope and linked procedure coverage | No additional finding after the applicable v1 corrections |
| [00-current-state-and-gaps/baseline.md](../../../00-current-state-and-gaps/baseline.md) | Baseline | No additional finding after the applicable v1 corrections |
| [00-current-state-and-gaps/coupling.md](../../../00-current-state-and-gaps/coupling.md) | Coupling | No additional finding after the applicable v1 corrections |
| [00-current-state-and-gaps/implementation.md](../../../00-current-state-and-gaps/implementation.md) | Freeze and expand the migration inventory | No additional finding after the applicable v1 corrections |
| [00-current-state-and-gaps/reuse.md](../../../00-current-state-and-gaps/reuse.md) | Reuse | No additional finding after the applicable v1 corrections |
| [00-current-state-and-gaps/semantic-risks.md](../../../00-current-state-and-gaps/semantic-risks.md) | Semantic risks | No additional finding after the applicable v1 corrections |
| [01-target-architecture/README.md](../../../01-target-architecture/README.md) | Navigation, scope and linked procedure coverage | No additional finding after the applicable v1 corrections |
| [01-target-architecture/artifacts.md](../../../01-target-architecture/artifacts.md) | Artifacts | No additional finding after the applicable v1 corrections |
| [01-target-architecture/boundaries.md](../../../01-target-architecture/boundaries.md) | Boundaries | No additional finding after the applicable v1 corrections |
| [01-target-architecture/implementation.md](../../../01-target-architecture/implementation.md) | Implement the dependency boundaries | No additional finding after the applicable v1 corrections |
| [01-target-architecture/planning-and-compatibility.md](../../../01-target-architecture/planning-and-compatibility.md) | Planning and compatibility | No additional finding after the applicable v1 corrections |
| [02-language-independent-extraction/README.md](../../../02-language-independent-extraction/README.md) | Navigation, scope and linked procedure coverage | No additional finding after the applicable v1 corrections |
| [02-language-independent-extraction/caller-and-host-contracts.md](../../../02-language-independent-extraction/caller-and-host-contracts.md) | Implement caller obligations and host plans | V2-01, V2-02 |
| [02-language-independent-extraction/capture.md](../../../02-language-independent-extraction/capture.md) | Capture | No additional finding after the applicable v1 corrections |
| [02-language-independent-extraction/host-and-languages.md](../../../02-language-independent-extraction/host-and-languages.md) | Host and languages | No additional finding after the applicable v1 corrections |
| [02-language-independent-extraction/implementation.md](../../../02-language-independent-extraction/implementation.md) | Implement KIR and the extraction boundary | No additional finding after the applicable v1 corrections |
| [02-language-independent-extraction/kir-contract.md](../../../02-language-independent-extraction/kir-contract.md) | Kir contract | No additional finding after the applicable v1 corrections |
| [02-language-independent-extraction/typed-capture-procedure.md](../../../02-language-independent-extraction/typed-capture-procedure.md) | Implement typed capture and erasure correspondence | No additional finding after the applicable v1 corrections |
| [03-backend-extension-contract/README.md](../../../03-backend-extension-contract/README.md) | Navigation, scope and linked procedure coverage | No additional finding after the applicable v1 corrections |
| [03-backend-extension-contract/addition-only-tests.md](../../../03-backend-extension-contract/addition-only-tests.md) | Prove backend addition without existing source changes | V2-07 |
| [03-backend-extension-contract/capabilities.md](../../../03-backend-extension-contract/capabilities.md) | Capabilities | V2-05, V2-07 |
| [03-backend-extension-contract/compiler-protocol.md](../../../03-backend-extension-contract/compiler-protocol.md) | Compiler protocol | No additional finding after the applicable v1 corrections |
| [03-backend-extension-contract/conformance.md](../../../03-backend-extension-contract/conformance.md) | Conformance | No additional finding after the applicable v1 corrections |
| [03-backend-extension-contract/discovery.md](../../../03-backend-extension-contract/discovery.md) | Discovery | No additional finding after the applicable v1 corrections |
| [03-backend-extension-contract/implementation.md](../../../03-backend-extension-contract/implementation.md) | Implement discoverable packages and service protocols | V2-03, V2-07 |
| [03-backend-extension-contract/runtime-abi.md](../../../03-backend-extension-contract/runtime-abi.md) | Runtime abi | V2-06 |
| [04-spirt-and-gpu-lowering/README.md](../../../04-spirt-and-gpu-lowering/README.md) | Navigation, scope and linked procedure coverage | No additional finding after the applicable v1 corrections |
| [04-spirt-and-gpu-lowering/control-and-pointers.md](../../../04-spirt-and-gpu-lowering/control-and-pointers.md) | Implement control-flow and pointer legalization safely | V2-04, V2-05 |
| [04-spirt-and-gpu-lowering/direct-construction.md](../../../04-spirt-and-gpu-lowering/direct-construction.md) | Implement the SPIR-T worker and direct construction | V2-04, V2-05 |
| [04-spirt-and-gpu-lowering/emission-and-validation.md](../../../04-spirt-and-gpu-lowering/emission-and-validation.md) | Emit, validate, and package the target artifact | V2-05 |
| [04-spirt-and-gpu-lowering/numeric-and-matrix.md](../../../04-spirt-and-gpu-lowering/numeric-and-matrix.md) | Numeric and matrix | No additional finding after the applicable v1 corrections |
| [04-spirt-and-gpu-lowering/operation-mapping.md](../../../04-spirt-and-gpu-lowering/operation-mapping.md) | Implement the SPIR-T representation mapping | V2-04, V2-05 |
| [04-spirt-and-gpu-lowering/other-backends.md](../../../04-spirt-and-gpu-lowering/other-backends.md) | Other backends | No additional finding after the applicable v1 corrections |
| [04-spirt-and-gpu-lowering/pipeline.md](../../../04-spirt-and-gpu-lowering/pipeline.md) | Pipeline | No additional finding after the applicable v1 corrections |
| [04-spirt-and-gpu-lowering/primitive-lowering.md](../../../04-spirt-and-gpu-lowering/primitive-lowering.md) | Primitive lowering | No additional finding after the applicable v1 corrections |
| [04-spirt-and-gpu-lowering/qualification-and-upstream.md](../../../04-spirt-and-gpu-lowering/qualification-and-upstream.md) | Qualify the worker and manage upstream changes | No additional finding after the applicable v1 corrections |
| [05-runtime-and-interop/README.md](../../../05-runtime-and-interop/README.md) | Navigation, scope and linked procedure coverage | No additional finding after the applicable v1 corrections |
| [05-runtime-and-interop/deployment.md](../../../05-runtime-and-interop/deployment.md) | Deployment | No additional finding after the applicable v1 corrections |
| [05-runtime-and-interop/interop.md](../../../05-runtime-and-interop/interop.md) | Interop | No additional finding after the applicable v1 corrections |
| [05-runtime-and-interop/memory-and-bindings.md](../../../05-runtime-and-interop/memory-and-bindings.md) | Memory and bindings | V2-02, V2-06 |
| [05-runtime-and-interop/memory-and-failures.md](../../../05-runtime-and-interop/memory-and-failures.md) | Implement physical memory footprints and guard failures | V2-01 |
| [05-runtime-and-interop/state-and-epochs.md](../../../05-runtime-and-interop/state-and-epochs.md) | State and epochs | V2-01, V2-03 |
| [05-runtime-and-interop/submission-and-lifecycle.md](../../../05-runtime-and-interop/submission-and-lifecycle.md) | Implement submission outcomes and package lifetimes | V2-01, V2-03 |
| [05-runtime-and-interop/vulkan-adapter.md](../../../05-runtime-and-interop/vulkan-adapter.md) | Implement the first Vulkan runtime package | V2-01, V2-05, V2-06 |
| [06-verification-and-trust/README.md](../../../06-verification-and-trust/README.md) | Navigation, scope and linked procedure coverage | No additional finding after the applicable v1 corrections |
| [06-verification-and-trust/evidence-policy.md](../../../06-verification-and-trust/evidence-policy.md) | Implement fixed evidence policies | V2-01, V2-02, V2-05, V2-07 |
| [06-verification-and-trust/obligations.md](../../../06-verification-and-trust/obligations.md) | Obligations | V2-01, V2-02, V2-05 |
| [06-verification-and-trust/source-and-extraction.md](../../../06-verification-and-trust/source-and-extraction.md) | Source and extraction | No additional finding after the applicable v1 corrections |
| [06-verification-and-trust/trust-and-reports.md](../../../06-verification-and-trust/trust-and-reports.md) | Trust and reports | No additional finding after the applicable v1 corrections |
| [06-verification-and-trust/validators.md](../../../06-verification-and-trust/validators.md) | Validators | No additional finding after the applicable v1 corrections |
| [07-implementation-phases/README.md](../../../07-implementation-phases/README.md) | Navigation, scope and linked procedure coverage | No additional finding after the applicable v1 corrections |
| [07-implementation-phases/coverage-and-proof.md](../../../07-implementation-phases/coverage-and-proof.md) | Coverage and proof | No additional finding after the applicable v1 corrections |
| [07-implementation-phases/extension-and-release.md](../../../07-implementation-phases/extension-and-release.md) | Extension and release | No additional finding after the applicable v1 corrections |
| [07-implementation-phases/foundations.md](../../../07-implementation-phases/foundations.md) | Foundations | No additional finding after the applicable v1 corrections |
| [07-implementation-phases/implementation.md](../../../07-implementation-phases/implementation.md) | Turn the phases into reviewable implementation changes | No additional finding after the applicable v1 corrections |
| [07-implementation-phases/research-and-effort.md](../../../07-implementation-phases/research-and-effort.md) | Research and effort | No additional finding after the applicable v1 corrections |
| [07-implementation-phases/vertical-and-concurrency.md](../../../07-implementation-phases/vertical-and-concurrency.md) | Vertical and concurrency | No additional finding after the applicable v1 corrections |
| [08-production-acceptance/README.md](../../../08-production-acceptance/README.md) | Navigation, scope and linked procedure coverage | No additional finding after the applicable v1 corrections |
| [08-production-acceptance/ci-admission.md](../../../08-production-acceptance/ci-admission.md) | Implement generic CI without granting package authority | No additional finding after the applicable v1 corrections |
| [08-production-acceptance/ci.md](../../../08-production-acceptance/ci.md) | Ci | V2-03 |
| [08-production-acceptance/correctness.md](../../../08-production-acceptance/correctness.md) | Correctness | Add the eight findings' closure cases |
| [08-production-acceptance/gate-records.md](../../../08-production-acceptance/gate-records.md) | Implement candidate-bound gate evaluation | V2-07 |
| [08-production-acceptance/performance-procedure.md](../../../08-production-acceptance/performance-procedure.md) | Implement an executable performance policy | V2-08 |
| [08-production-acceptance/performance.md](../../../08-production-acceptance/performance.md) | Performance | V2-08 |
| [08-production-acceptance/profiles-and-hardware.md](../../../08-production-acceptance/profiles-and-hardware.md) | Profiles and hardware | No additional finding after the applicable v1 corrections |
| [08-production-acceptance/rollout.md](../../../08-production-acceptance/rollout.md) | Rollout | No additional finding after the applicable v1 corrections |
| [09-work-packages-and-decisions/README.md](../../../09-work-packages-and-decisions/README.md) | Navigation, scope and linked procedure coverage | No additional finding after the applicable v1 corrections |
| [09-work-packages-and-decisions/corrections.md](../../../09-work-packages-and-decisions/corrections.md) | Assign the audit corrections to implementation owners | Owners mapped in this audit's remediation page |
| [09-work-packages-and-decisions/decisions.md](../../../09-work-packages-and-decisions/decisions.md) | Decisions | No additional finding after the applicable v1 corrections |
| [09-work-packages-and-decisions/risks.md](../../../09-work-packages-and-decisions/risks.md) | Risks | No additional finding after the applicable v1 corrections |
| [09-work-packages-and-decisions/traceability.md](../../../09-work-packages-and-decisions/traceability.md) | Traceability | No additional finding after the applicable v1 corrections |
| [09-work-packages-and-decisions/work-packages.md](../../../09-work-packages-and-decisions/work-packages.md) | Work packages | Owners mapped in this audit's remediation page |
| [10-sources/README.md](../../../10-sources/README.md) | Navigation, scope and linked procedure coverage | No additional finding after the applicable v1 corrections |
| [10-sources/audit-method.md](../../../10-sources/audit-method.md) | Audit method | No additional finding after the applicable v1 corrections |
| [10-sources/claim-evidence.md](../../../10-sources/claim-evidence.md) | Match each instruction to primary evidence | No additional finding after the applicable v1 corrections |
| [10-sources/external.md](../../../10-sources/external.md) | External | No additional finding after the applicable v1 corrections |
| [10-sources/kuiper.md](../../../10-sources/kuiper.md) | Kuiper | No additional finding after the applicable v1 corrections |
| [10-sources/spirt.md](../../../10-sources/spirt.md) | Spirt | No additional finding after the applicable v1 corrections |
| [10-sources/upstream-decisions.md](../../../10-sources/upstream-decisions.md) | Record SPIR-T reuse and dependency decisions | Dated inventory correctly labeled; metadata refreshed |
| [README/README.md](../../../README/README.md) | Navigation, scope and linked procedure coverage | No additional finding after the applicable v1 corrections |
| [README/architecture.md](../../../README/architecture.md) | Architecture | No additional finding after the applicable v1 corrections |
| [README/completion.md](../../../README/completion.md) | Completion | No additional finding after the applicable v1 corrections |
| [README/implementation-sequence.md](../../../README/implementation-sequence.md) | Implement the migration in this order | No additional finding after the applicable v1 corrections |
| [README/reading-order.md](../../../README/reading-order.md) | Reading order | No additional finding after the applicable v1 corrections |
| [README/requirements.md](../../../README/requirements.md) | Requirements | No additional finding after the applicable v1 corrections |

## Machine-readable and checker files

| File | Check and limitation |
|---|---|
| [document-map.json](../../../document-map.json) | Parsed; all 20 old paths mapped to matching folders; destinations exist. This is a migration map, not a compiler contract. |
| [milestones.json](../../../milestones.json) | Parsed; phase DAG, gate references, separate replacement claim and conditional host-codegen gate reviewed. No implementation result is claimed. |
| [resolutions.json](../../../resolutions.json) | All 21 v1 rows traced to instructions. Historical corrected/pending states are preserved; residual cases are recorded separately here. |
| [schemas/gate-result.schema.json](../../../schemas/gate-result.schema.json) | Read as a structural envelope; V2-07 enum witness executed. No full schema-engine/metaschema validation claimed. |
| [tools/check_roadmap.py](../../../tools/check_roadmap.py) | Read and executed; checked links, reachability, mappings, planning consistency and available pinned source paths. It does not evaluate semantic correctness or the new v2 finding count. |

See [historical coverage](coverage-history.md), [executed diagnostics](diagnostics.md) and [rejected suspicions](not-findings.md) for the bounds of these checks.
