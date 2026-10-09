# Kuiper: replace CUDA dependence with SPIR-T

The goal is to make Kuiper a verified GPU programming system whose compiler, runtime, and extraction contracts are independent of CUDA and of any single source or output language. SPIR-T becomes the device compilation layer. GPU backends, source-language adapters, and host-language bindings become independently installable packages.

The target repository is **QuasarRay/Quiper**. The existing language and modules are named **Kuiper**; this roadmap preserves that naming. All components, APIs, profiles, commands, and directories proposed below are future work unless explicitly identified as existing.

**Status:** revised implementation instructions; proposed components remain unimplemented. Revised on 2026-10-10 Australia/Melbourne, using evidence inspected on 2026-10-09 UTC. Baseline: Quiper `413219948f91911ffaf0ac37a5ff941c5d1e55c7` and SPIR-T `e8757adba8d14068a7bf1b3bc9f24cac982f4bd3`.


## Categorized instructions

- [Requirements](requirements.md)
- [Architecture](architecture.md)
- [Reading order](reading-order.md)
- [Completion](completion.md)

## Detailed implementation and correction procedures

- [Implementation sequence](implementation-sequence.md)

## Start here

Read [the implementation sequence](implementation-sequence.md), then [the categorized topic order](reading-order.md). See [all 21 corrections](../Flaws/README/resolutions.md), [the full audit collection](../Flaws/README/README.md), [primary evidence](../10-sources/claim-evidence.md), and [the path migration map](../document-map.json).

The original 20 Markdown files, including the eight audit files, each now have a matching folder containing categorized pages. `README.md` files inside those folders are new navigation indexes. Run `python3 Roadmap/tools/check_roadmap.py` from the repository root to validate document links, finding coverage, milestone references and pinned source paths available in this checkout.
