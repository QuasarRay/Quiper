# Replay checked source and integer semantics on physical Vulkan

Run [the hardware workflow](../../.github/workflows/portable-hardware.yml) on the repository's existing `self-hosted, linux, X64, cuda` runner. It uses an Ubuntu container with graphics/utility driver injection and installs no CUDA toolkit. Build the backend from its independently locked packages.

## Admit one hardware driver

The selector probes explicit Vulkan driver manifests and accepts only a pool advertised as integrated or discrete GPUs. It sets one `VK_DRIVER_FILES` manifest for the entire replay. CPU, virtual, unknown and mixed pools fail. The harness repeats this classification before installing or invoking workers, measures the selected manifest before and after replay, and records the devices returned by execution. A vendor name is insufficient admission evidence.

[Khronos documents the driver override](https://github.com/KhronosGroup/Vulkan-Loader/blob/main/docs/LoaderDriverInterface.md#overriding-the-default-driver-discovery) and the [physical-device type enumeration](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceType.html). The type is informational metadata. Runtime feature, limit, reflection and target-validation admission remain required.

## Bind replay to the successful source check

Wait for the latest source workflow for the exact requested Git head to complete successfully. Download its existing `source-cpu-vulkan-evidence` artifact through the authenticated GitHub artifact API. A failed latest run must stop replay; an older successful run cannot replace it.

The source report carries every admitted KIR package, export manifest, execution invocation and literal expected output. It includes guards for short views and binds each vector to its completed CPU result. Admission requires the actual checkout SHA and measured source bytes to match the hardware checkout. Reject duplicate JSON fields, unsupported stronger assurance claims, changed package or invocation digests, changed expected outputs, missing executions, duplicate cases and source paths outside the checkout.

The producer verifies the source with the pinned compiler before creating these packages. Hardware replay executes those exact packages; it does not rerun F* on the GPU runner or establish source refinement.

## Run both matrices and preserve their evidence

Replay the 75-case integer matrix through installed workers and the independent reference evaluator. Require every core case and the matrix's explicit C-interface cases. Run the eight Vulkan execution suites. Then replay all 38 measured source vectors through the same interfaces, including guard rejections. The source hardware report contains 39 cases with additive installation; the CPU source report contains 45 cases because it also checks six invalid source admissions.

Require the Khronos validation layer and zero validation diagnostics. Measure the core executable, C library, C driver and worker binaries. Reject changed host binaries or driver manifest bytes before publishing a report. Emit `integer-hardware.json` or `source-hardware.json` only after its complete matrix passes. Workflow success also requires the execution suites to pass.

## Keep qualification claims scoped to the run

This is one physical device smoke test. It does not cover every adapter, establish implementation refinement, replay every source proof dependency, qualify noncoherent/fault/cancellation paths, or close the workload, driver, soak, performance, installation and rollback matrix. Production gates remain false.

Serialize hardware replay jobs and do not cancel an older submitted job to start a newer candidate. A CI timeout is an external limit; it does not prove GPU quiescence or safe cancellation. An unresponsive driver must block production qualification until the containment protocol is implemented.

A workflow definition is insufficient evidence of a successful physical run. Record the actual job result, candidate and report separately from earlier CPU reports.
