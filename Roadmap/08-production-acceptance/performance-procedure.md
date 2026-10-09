# Implement an executable performance policy

## 1. Ratify the inputs before measuring

In P0, create a versioned performance manifest with exact corpus/generator/input digests, comparable baseline per case, hardware/driver and numerical modes, workload weights, critical flags, queue depths, warmups, trial counts, measured statistics, thresholds, absolute latency/startup/memory limits, and confidence procedure. Blank limits block G-PRODUCTION. The following is the initial policy to ratify, not a measured performance claim.

Use equal weights for end-to-end workload cases unless P0 publishes fixed application-derived weights before collecting candidate results. Weights sum to one. Keep different hardware/native-baseline groups separate. Compare legacy CUDA only on the same NVIDIA hardware, identical algorithms/input/precision and equivalent semantics. On other hardware use a specified native baseline or the prior qualified release; report that group's meaning separately.

## 2. Normalize regression direction

All ratios make larger values worse:

- Latency, compile time, memory and overhead: `new / baseline`.
- Throughput: `baseline / new`.
- Aggregate end-to-end score: `exp(sum(weight[i] * log(ratio[i])))` over the predefined comparable cases.

Reject zero/negative/invalid denominators and incomparable measurements. Never combine latency and its inverse or let a throughput gain conceal a correctness failure. Report individual cases alongside the aggregate.

Ratify these proposed limits: aggregate end-to-end ratio ≤ 1.10; every critical case ≤ 1.20; peak memory and warm host submission ratio ≤ 1.10; every latency-sensitive case's p99 ratio ≤ 1.20 and its predefined absolute service limit. Cold compile/pipeline time must meet its predefined absolute startup limit. A missing tail/startup budget means the gate cannot pass, rather than treating the metric as informational.

## 3. Make uncertainty part of the decision

Use randomized paired baseline/candidate blocks on the same device. Retain raw samples and block identities. Use at least 30 independent blocks; measure enough requests per block to estimate the specified tail with useful precision. P0 fixes warmup and sample count using pilot measurements, before candidate qualification.

For an upper-bounded metric, use a simultaneous 95% confidence procedure over all required decisions. One concrete initial implementation is a seeded 10,000-replicate block bootstrap with a prespecified Bonferroni adjustment for the number of decisions. Resample independent blocks, retain pairing, and recompute each full statistic, including the aggregate. Do not bootstrap correlated requests as if they were independent trials. Store the procedure and RNG/seed/version with the result.

Classify each limit as pass if the interval's upper bound is at or below the limit, fail if its lower bound exceeds the limit, otherwise inconclusive. Overall pass requires every required limit to pass, except an admitted performance-only waiver. Allow at most two additional prespecified batches for inconclusive results, combine all observations under the same rule, and then report inconclusive if unresolved. Account for repeated looks in the confidence procedure, for example by allocating the error budget across all three looks. Repeatedly sampling until a convenient pass is prohibited.

Separate controlled noise failures, thermal throttling, timeouts and real regressions. Fix environmental faults and repeat the declared experiment; retain discarded-run reasons. A faster erroneous kernel never enters performance qualification.

## 4. Validate the metric evaluator

Use synthetic fixtures with a 30% throughput regression, a single critical 25% latency regression, a p99-only regression, invalid denominators and an interval crossing the limit. Fix expected pass/fail/inconclusive decisions before implementation. Verify that all ratio orientations and weights give the intended result. Include a case whose geometric mean passes while a critical or tail limit fails.

Benchmark the IPC and in-process paths separately where both are supported, including batching, cancellation and borrowed-buffer retention. Tune only after recording a correct baseline. Changing an algorithm or numerical mode creates a different comparison rather than an unexplained compiler speedup.
