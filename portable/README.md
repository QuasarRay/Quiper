# Portable integer implementation

The new path accepts checked KIR, constructs SPIR-T directly, emits SPIR-V and runs it through an independent Vulkan worker. Compiler and runtime installation only adds files. The core has no F*, Pulse, SPIR-T, Vulkan or CUDA dependency.

This is an **experimental integer implementation**, with `kuiper.experimental-tested/1` evidence. It is not a production-ready replacement for Kuiper's CUDA backend. A Kuiper/Pulse source exporter, complete refinement proofs, broader GPU semantics and the hardware/operational qualification matrix remain unfinished.

Follow these milestones in order:

1. [Build, install and execute](implementation/01-install-and-execute.md).
2. [Check semantics and owned execution](implementation/02-check-semantics-and-ownership.md).
3. [Connect checked source and close refinement](implementation/03-connect-source-and-refinement.md).
4. [Qualify the exact release candidate](implementation/04-qualify-the-candidate.md).

The authoritative specification remains [Roadmap/Specification](../Roadmap/Specification/README.md). [Implementation status](../validation/implementation-status.json) keeps every roadmap release gate open until its full criterion has actual evidence. Model lemmas and CPU tests do not establish those gates by themselves.
