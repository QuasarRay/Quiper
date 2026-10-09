# Artifacts

## 3. Three artifacts with different purposes

**Portable KIR package:** canonical kernel/host semantics, resource layouts, symbols, requirements, source mappings, proof/evidence references, and dependency digests. This is the stable extraction boundary.

**Compiler working representation:** process-local SPIR-T objects and analysis state. Pin their version inside the compiler package. Their pretty-printed form is a diagnostic artifact; it is not the persistence or interchange format.

**Target package:** device code or backend-specific IR, exact target environment, host ABI layout, entrypoint reflection, specialization values, numerical policy, proof/checker results, and compilation provenance. A target package is loadable only by compatible runtime packages.

Never cache a raw pointer, process-local entity ID, driver handle, or native Rust struct as a portable artifact.
## 4. Preserve information deliberately

The frontend must export facts before erasure loses them: element types, layout relations, alignment, aliasing permissions, subgroup assumptions, matrix dimensions, uniformity, effects, and numeric modes. Keep ghost derivations in evidence/side metadata, bound to executable operations by stable IDs and digests. Debug information is separately classified; dropping debug information must not drop a semantic requirement.

Do not preserve every compiler AST node forever. Preserve the information required to check semantics or guide a justified optimization. Each transformation declares facts it consumes, preserves, establishes, or invalidates. After a rewrite, stale evidence is rejected or regenerated.

An opaque extension cannot be treated as pure by default. Until a trusted/checkable operation definition is available, it is an optimization barrier and cannot enter a verified executable profile.
