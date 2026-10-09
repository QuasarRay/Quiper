# 7. Documentation validation

**Historical audit record.** This describes the audit snapshot, not the current correction status. See [current resolutions](../README/resolutions.md).

## 7. Documentation validation

The collection was checked for unique finding IDs, agreement between the index and detailed severities, required location/correction/closure fields, balanced Markdown fences, local link targets and heading anchors, and final newlines/whitespace. Pinned Quiper and SPIR-T source paths and referenced line ranges were checked against the corresponding git objects. Cited F* paths were checked against the retrieved pinned tree and file contents.

The result is eight Markdown files containing 21 findings, with 13 high and 8 medium severities. The change is confined to additions under `Roadmap/Flaws`. This validation checks the audit documents; none of the proposed implementation closure tests is reported as executed.
