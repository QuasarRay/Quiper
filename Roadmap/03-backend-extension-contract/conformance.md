# Conformance

## 7. Mandatory no-edit tests

**G-ADD-COMPILER:** freeze/hash the released core and adapter contracts. Build an independent compiler backend package against the public SDK. Install it; compile the existing portable corpus; compare hashes before/after. Only new package files and explicit deployment configuration may differ.

**G-ADD-RUNTIME:** repeat with a materially different runtime/API, not just a second Vulkan vendor. A candidate is a Metal path using an independently qualified translation/runtime package; an OpenCL candidate must solve SPIR-T's current `Kernel` limitation or use a separately specified lowering. P0 selects and records the second implementation after a feasibility spike; P7 implements and qualifies that selected route.

Also require the [compiler-only and compatible-runtime substitutions](addition-only-tests.md). The distinct-API exercise is cumulative with those tests; a paired backend with an undocumented private artifact convention cannot satisfy independence.

**G-ADD-LANGUAGE:** add a frontend and a host binding independently; reuse unchanged runtime and compiler packages.

**G-ADD-OP:** add a semantic extension and implementation with its evidence/checker support. Ensure an old core routes it correctly while an installation lacking that support rejects it before execution.

For each test, prohibit central enum edits, manual registration patches, root dependency edits, hidden environment rewrites, frontend conditionals, and updates to portable kernel source. Run against an installed core binary as well as a read-only source checkout. Record exact hashes and the filesystem diff.
## 8. Compatibility and withdrawal

Negotiate protocol major/minor versions and required extensions explicitly. Additive optional fields may be ignored only when they carry no required semantics; unknown required fields/versions must fail closed. Keep a corpus of old valid packages and intentionally incompatible packages.

Support disabling a broken backend package and rolling back its version independently. Cache keys include package and semantic digests. Removing a package must not break artifact inspection or falsely report that its cached binaries remain executable.
