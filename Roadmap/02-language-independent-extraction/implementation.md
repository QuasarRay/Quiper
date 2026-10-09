# Implement KIR and the extraction boundary

## 1. Specify the portable data model

KIR v1 is a new contract to implement. It does not exist in upstream F*, Pulse, Kuiper, or SPIR-T. Publish the grammar, semantics, canonical vectors, and generated reader tests together. Do not serialize a source AST or a SPIR-T Rust struct and call it language-independent.

Use separate sections for types, constants, resource declarations, functions/regions, host plans, requirements, and evidence. IDs are package-local unsigned integers with declared bounds; external symbols are namespaced names plus semantic-version/digest identity. Resolve imports by full signature and effects before final emission. Distinct definitions with the same external identity are a link error unless identical under the specified canonical identity.

The initial kernel form is typed SSA with structured regions. Every operation has explicit result types, operands, region arguments/results where applicable, effects, semantic policy, and source/evidence correspondence. Values dominate their uses. Region exits have fixed arity/types. Loops explicitly declare carried values, condition, body/continue behavior, and exits. A later unstructured importer must legalize into this form or declare a separately supported dialect.

Types include fixed-width integers with signedness, explicit binary float formats, Boolean values, vectors/aggregates with specified layouts, and logical resource views. A view carries allocation identity, byte offset, extent, element layout, permissions, and lifetime generation. There is no implicit `usize`, raw host pointer, F* term, or host-language object in portable bytes.

## 2. Give operations precise contracts

| Operation family | Required meaning | Reject when |
|---|---|---|
| Integer arithmetic | Width and wrapping/checked/saturating/preconditioned behavior; shift and division edge rules | A source operation is mapped to a different overflow relation |
| Floating operations | Format, rounding/error relation, contraction, signed-zero/NaN/denormal policy | Requested behavior lacks an approved target implementation |
| Resource access | Address space, checked offset/size calculation, alignment, initialization and access permission | Bounds, provenance, or required caller obligations are not established |
| Calls/control | Evaluation order, argument/result/effect signatures, loop/recursion policy | Unsupported recursion, escaping resources, or unmatched effects remain |
| Collective operations | Execution/memory scope, participant set, ordering and storage effects | Uniformity/convergence obligations are unavailable |
| Assertions/guards | Proved predicate versus runtime condition with an exceptional exit | Unchecked assertions are promoted to proof facts or failure can violate collective participation |

Separate storage representation from arithmetic representation. For example, encode portable Boolean values canonically and marshal them to a chosen device representation; never infer storage size from a host compiler's Boolean type. Host and device layouts need an explicit equality witness or conversion plan. Padding, stride, alignment, and array extent are semantic inputs to resource reasoning.

Use bounded canonical JSON for control manifests initially: sorted object keys, duplicate-key rejection, UTF-8, normalized identifiers, exact integer strings and fixed-width hexadecimal float bit patterns where lossless numbers matter. Do not canonicalize NaNs or signed zero away. Define domain-separated content hashes over canonical payloads, excluding only explicitly nonsemantic fields. A later binary body encoding needs equivalent normative vectors and version negotiation; optimization does not authorize reinterpretation of old bytes.

## 3. Implement independent validators

The decoder checks lengths, counts, UTF-8, numeric encodings, IDs and required versions before allocation. The structural/type checker checks references, dominance, signatures, region shapes, resource declarations, effects, and extension requirements. The semantic/evidence checker establishes the selected policy's obligations. Passing the first two does not establish memory safety of arbitrary unchecked device code.

Bound parser nesting, region count, imports, specialization variants, instruction count, proof-checker time, and output bytes. Return structured errors with a stable code, stage, source span, and operation ID. Build two independent decoder implementations from the written contract and shared vectors; shared generated code is useful but not independent evidence against a shared generator error.

## 4. Prove that neither endpoint depends on a language

Implement the F*/Pulse exporter and a restricted typed C frontend. The C adapter must specify its accepted semantics and diagnose UB-prone constructs rather than borrowing a CUDA/C interpretation accidentally. Give it elementwise, reduction, and tiled matrix examples over the accepted subset; keep its proof status accurate.

Implement one neutral host-plan interpreter and C/Rust bindings that load identical target-package bytes. The first release requires this binding comparison, including asynchronous resource use and failures. Native host-plan compilation is optional and must pass G-HOST-CODEGEN before selection. In P7 add a third independent frontend or a specified SPIR-V importer and a separate binding with no existing source changes.

The official [Pulse extraction guide](https://fstar-lang.org/tutorial/book/pulse/pulse_extraction.html) documents existing OCaml/C/Rust routes and an extension AST path. This supports investigating reuse; it does not establish that that AST retains the Kuiper facts KIR needs. Follow the next [typed capture procedure](typed-capture-procedure.md).
