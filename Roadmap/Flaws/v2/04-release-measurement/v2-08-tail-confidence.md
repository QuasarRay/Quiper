# V2-08: the confidence recipe lacks tail-adequacy and bootstrap-resolution rules

**Severity:** Medium. **Status:** Open. **Evidence class:** measurement-policy gap with an executed mathematical counterexample. **Owner:** performance and release owners. **Resolve by:** P0 performance ratification and the metric evaluator, before G-PRODUCTION uses the result.

## 1. Affected instructions

The [performance procedure](../../../08-production-acceptance/performance-procedure.md) supplies ratio direction, paired blocks, at least 30 independent blocks, a seeded 10,000-replicate bootstrap, multiple-decision/repeated-look adjustment and pass/fail/inconclusive rules. These correct much of F06.

The [frozen recipe](https://github.com/QuasarRay/Quiper/blob/9dceaf274b46f295f7fc312fb3396d5729d7d97d/Roadmap/08-production-acceptance/performance-procedure.md) does not fix the interval construction, quantify independent-block adequacy for p99, or impose a resolution/error rule when multiplicity pushes bootstrap tail probabilities below the available resampling precision. Calling for useful precision and pilot measurements does not supply those decision rules.

## 2. A permitted implementation can make a false tail decision

Consider a deliberately simple population, with equal request counts per independent block. Baseline latency is always 1. A candidate block has latency 1 for all its requests with probability 0.98, and latency 2 for all its requests with probability 0.02. Within-block correlation is retained, as the roadmap requires. The population p99 ratio is 2, beyond the proposed 1.20 limit.

With 30 independent blocks, the probability of observing no slow block is `0.98 ** 30 = 0.5454843193824369`. On that event every observed ratio is 1. A percentile block bootstrap produces `[1, 1]` for any resample count and passes the tail limit. Bonferroni adjustment does not create the missing tail observations. More requests inside the same blocks do not solve this example.

This is a counterexample to treating the named minimum/recipe as sufficient for the stated confidence claim. It is not a measurement of Kuiper, and it does not show that every bootstrap implementation fails. A degenerate-data rule returning inconclusive can prevent this particular false pass; v2 needs to require such a rule or another justified procedure.

## 3. Multiplicity also needs a numerical resolution budget

For 200 decisions and three looks, one equal Bonferroni allocation for a two-sided 95% procedure puts `0.05 / (2 * 200 * 3)` in each tail. Ten thousand replicates yield only about `0.4167` expected resampled statistics in such a tail. A fixed seed makes this repeatable; it does not establish the accuracy of the extreme quantile estimate.

The official [SciPy bootstrap documentation](https://docs.scipy.org/doc/scipy/reference/generated/scipy.stats.bootstrap.html) distinguishes interval methods and describes degenerate-distribution limitations. It does not establish coverage for this project's block statistics. The numerical examples here are independently calculated in the [diagnostic record](../05-coverage-and-evidence/diagnostics.md).

## 4. Required correction and closure

Specify the estimand, quantile convention, interval method, independence/stationarity assumptions and treatment of degenerate or unresolved tails. Choose independent-block counts and observation windows for the claimed tail population. Do not infer adequacy solely from the number of requests or zero observed slow events.

Scale resample count and numerical error control to the actual multiplicity/look budget, or select a justified alternative confidence procedure. Where the method cannot support the required decision, return inconclusive. Preserve the existing limit on optional remeasurement and ratify choices before seeing candidate results.

Extend evaluator fixtures with rare slow blocks, all-identical observations, heavy tails, high multiplicity, correlated requests and repeated looks. Calibrate coverage/error behavior on declared model families and document the limits of that calibration. Retain ordinary ratio-orientation tests as separate checks.

For the specific 2% example, 149 independent blocks make the probability of seeing no slow block at most 5%. That arithmetic is only a detection illustration. It is not a universal sample-size prescription or enough by itself to qualify p99 accurately.
