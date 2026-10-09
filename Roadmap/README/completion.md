# Completion

## 4. Definition of completion

A declared portable release is qualified when the following conditions hold. Full CUDA replacement additionally requires G-REPLACEMENT over every mandatory row in `legacy-scope.json`; deferred rows cannot satisfy that gate:

- Every migrated primitive has a semantic specification, implementation, target requirement, test, and evidence classification.
- Supported kernels and their host plans run without the CUDA toolkit, CUDA runtime, NVCC, or Karamel in the new extraction path.
- At least two materially different real GPU environments pass the portable profile; the second runtime backend is independently installed and passes the no-edit test.
- A second non-F* input adapter and two host-language bindings use the same artifact contract. Their verification claims remain accurate.
- Required proofs and validation checks pass against the exact shipped artifacts; remaining trusted assumptions are explicit.
- The clean-install, failure-recovery, performance, compatibility, packaging, and rollback gates pass.

Vulkan on two vendors establishes useful hardware portability. It does **not** by itself demonstrate that a genuinely different compiler/runtime backend can be added without changing the core. Both tests are required.
