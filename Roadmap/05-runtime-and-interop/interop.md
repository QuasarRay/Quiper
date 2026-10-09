# Interop

## 5. External interoperability

Start with standalone buffers and explicit host transfers. Add external memory and synchronization only when a real integration needs them. Each import/export contract states resource ownership, device compatibility, allowed handle types, mapping restrictions, synchronization handoff, and destruction rules.

Tensor-framework interoperability should preserve shape, stride, element type, device identity, and producer/consumer synchronization. A zero-copy claim requires measurements and an actual shared allocation; it is not established by avoiding a copy in one wrapper.

CUDA interoperability may exist in an optional compatibility package. It must not introduce a CUDA dependency into ordinary Vulkan or Metal use. GPUDirect/RDMA, GPU-initiated I/O, or a GPU-native operating system are independent projects, not consequences of changing the compiler IR.
