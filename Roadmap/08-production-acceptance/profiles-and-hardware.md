# Profiles and hardware

## 1. Qualify explicit release profiles

Use a per-feature/per-backend support matrix with statuses: unimplemented, experimental, tested, qualified. Record assurance separately using the fixed `qualified-v1`, `source-verified-v1`, and `refinement-verified-v1` policies and their explicit assumptions; record individual translation-validation results as evidence, not as an unqualified end-to-end label. Do not collapse these dimensions into one green status.

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
