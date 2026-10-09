# Planning and compatibility

## 5. Compiler control without backend coupling

Provide a deterministic pipeline planner. Pass packages declare input/output dialect versions, preconditions, effects, analysis invalidation, target requirements, and evidence obligations. The planner builds an explicit acyclic pass graph and rejects ambiguous or cyclic lowering plans.

Users can inspect the selected plan, choose approved passes, disable an optimization, pin specialization choices, inspect each IR stage, and compare costs. Performance tuning may suggest candidate schedules; acceptance is controlled by semantic checks and measured budgets. The trusted path does not depend on a tuning model being correct.

Separate mandatory legalization from optional optimization. Disabling an optimization must not accidentally disable an essential memory-model or ABI legalization. Do not silently reroute a failed compilation through a weaker numerical profile.
## 6. Limits of portability

The portable baseline is a useful, explicitly restricted compute profile. Extensions expose hardware features instead of flattening everything to the least capable GPU. Programs state their requirements; backends advertise implementations and constraints.

Two backends may realize the same numerical relation with different instructions. They may not substitute a weaker relation without the program selecting that relation. A kernel requiring NVIDIA-specific WGMMA semantics can remain tied to an extension even when the surrounding application and extraction system are portable.

Compilation format and runtime API are independent choices. SPIR-V emission does not implement Vulkan; generating an ISA binary does not implement allocation, relocation, command submission, or driver compatibility. The selected compiler/runtime combination must account for both sides. A package can implement either role independently or both as separately addressable services.
## 7. Compatibility policy

Maintain portable source APIs wherever existing semantics permit. During P1–P4, some core and proof interfaces must change to remove hardcoded assumptions; the no-edit backend guarantee begins at the explicit contract-freeze milestone.

Existing CUDA headers, `cudaStream_t` values, CUDA launch syntax, and device pointers are not automatically a portable ABI. Supply a documented legacy compatibility package or a deliberate client migration. Never describe those clients as unchanged when their binary interface has changed.

Keep old contract readers available through supported adapter packages. Major versions may coexist. Migration tools produce new artifacts with new digests and fresh evidence; they do not relabel an old artifact as compliant.
