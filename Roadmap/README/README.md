# Kuiper roadmap v3: implement the declarative specification

Implement these milestones in order. Each page names a concrete deliverable, its declarative F* relations, the implementation procedure and the evidence required to close it. The tested partial implementation is linked below; the full production implementation remains unfinished.

The target is a fully decoupled Kuiper with language-independent kernel/host contracts and independently installable compiler, runtime, binding and semantic packages. Compatible backend additions must preserve the frozen installed core and existing packages. SPIR-T is the first detailed compiler realization.

**V3 status:** all eight v2 documentation flaws are corrected. The executable F* model and its verification record are in [Specification](../Specification/README.md). Production implementation, refinement of concrete code and GPU qualification remain outstanding.

[The experimental implementation](../../portable/README.md) now executes checked linear U32/ref source through SPIR-T and software Vulkan. [Implementation status](../../validation/implementation-status.json) and [implementation findings](../Flaws/implementation/README.md) record its measured scope and unresolved gates separately from this declarative roadmap.

## Begin here

- [Specification and proof scope](../Specification/README.md)
- [V2 correction record](../Flaws/v2/resolutions/README.md)
- [Implementation milestone inventory](../implementation-milestones.json)

- [M01: Freeze the decoupling and replacement claim](01-freeze-the-goal.md)
- [M02: Sequence implementation by evidence dependencies](02-sequence-the-implementation.md)

## Implementation areas

- [00 current state and gaps](../00-current-state-and-gaps/README.md)
- [01 target architecture](../01-target-architecture/README.md)
- [02 language independent extraction](../02-language-independent-extraction/README.md)
- [03 backend extension contract](../03-backend-extension-contract/README.md)
- [04 spirt and gpu lowering](../04-spirt-and-gpu-lowering/README.md)
- [05 runtime and interop](../05-runtime-and-interop/README.md)
- [06 verification and trust](../06-verification-and-trust/README.md)
- [07 implementation phases](../07-implementation-phases/README.md)
- [08 production acceptance](../08-production-acceptance/README.md)
- [09 work packages and decisions](../09-work-packages-and-decisions/README.md)
- [10 sources](../10-sources/README.md)

## Evidence and history

- [Audit collection](../Flaws/README/README.md)
- [V2 audit, preserved against its reviewed revision](../Flaws/v2/README.md)
- [Phase and gate plan](../milestones.json)
- [Document migration map](../document-map.json)

Run `python3 Roadmap/tools/check_roadmap.py` for links, milestone/specification coverage and proof-record identity. Run `python3 Roadmap/tools/verify_spec.py --fstar /absolute/path/to/fstar.exe --report /tmp/quiper-spec-verification.json` for strict F* verification. Use the locked toolchain described in the specification.
