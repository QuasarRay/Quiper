# Close tail-measurement qualification

The roadmap correction is complete. Concrete backend implementation and qualification remain pending. The F* lemmas below concern the specified model; each implementation must establish the correspondence and closure cases.

## V2-08

Tail estimands, independent sampling units, exact order-statistic bounds, multiplicity/looks and inadequate-data rejection are specified.

Implement [the milestone](../../../08-production-acceptance/03-qualify-performance-and-cutover.md) against `Qualification.uncertain_cannot_pass` in [the specification](../../../Specification/README.md). Preserve [the original finding](../04-release-measurement/v2-08-tail-confidence.md) as the reason for the change.

**Required implementation evidence:** Reject the 30-block fast-only sample as inadequate; validate binomial rank bounds, ties, zero denominators, confidence allocation and cumulative looks.
