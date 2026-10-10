# Qualify performance uncertainty and complete cutover

**Milestone M27.** Prespecified performance decisions with adequate tail evidence, bounded waivers, tested rollout and rollback.

## Required inputs and specification

Start from [M25](01-qualify-the-correctness-matrix.md), [M26](02-implement-candidate-bound-admission.md). Use the exit-gate dependencies in [the implementation plan](../milestones.json); proof and investigation work may begin earlier. Read [the declarative specification](../Specification/README.md) before choosing representation details. This milestone implements `Qualification.classify`, `Qualification.full_replacement`.

## Ratify the inputs before measuring

In P0, create a versioned performance manifest with exact corpus/generator/input digests, comparable baseline per case, hardware/driver and numerical modes, workload weights, critical flags, queue depths, warmups, trial counts, measured statistics, thresholds, absolute latency/startup/memory limits, and confidence procedure. Blank limits block G-PRODUCTION. The following is the initial policy to ratify, not a measured performance claim.

Use equal weights for end-to-end workload cases unless P0 publishes fixed application-derived weights before collecting candidate results. Weights sum to one. Keep different hardware/native-baseline groups separate. Compare legacy CUDA only on the same NVIDIA hardware, identical algorithms/input/precision and equivalent semantics. On other hardware use a specified native baseline or the prior qualified release; report that group's meaning separately.

## Normalize regression direction

All ratios make larger values worse:

- Latency, compile time, memory and overhead: `new / baseline`.
- Throughput: `baseline / new`.
- Aggregate end-to-end score: `exp(sum(weight[i] * log(ratio[i])))` over the predefined comparable cases.

Reject zero/negative/invalid denominators and incomparable measurements. Never combine latency and its inverse or let a throughput gain conceal a correctness failure. Report individual cases alongside the aggregate.

Ratify these proposed limits: aggregate end-to-end ratio ≤ 1.10; every critical case ≤ 1.20; peak memory and warm host submission ratio ≤ 1.10; every latency-sensitive case's p99 ratio ≤ 1.20 and its predefined absolute service limit. Cold compile/pipeline time must meet its predefined absolute startup limit. A missing tail/startup budget means the gate cannot pass, rather than treating the metric as informational.

## Fix the estimand and confidence procedure before collecting results

Implement V2-08 by replacing the underspecified bootstrap gate. For each latency-sensitive workload/device/profile/queue-depth stratum, define the population as requests drawn from the frozen input and arrival distribution after the declared warmup. The quantile is `inf{x: F(x) >= 0.99}`. The target ratio is candidate population p99 divided by baseline population p99, not p99 of paired ratios and not the mean of per-block p99 values. Define cold-start and sustained-load strata separately.

The initial tail gate uses independent measurement windows and one randomly selected, prespecified request per window for each candidate/baseline condition. Keep pairing where possible and randomize condition order. Windows must be separated/controlled enough to justify independence; record that argument from pilot autocorrelation/drift evidence. Extra correlated requests are diagnostic data, not extra independent samples. If the deployment distribution or stationarity/independence assumption is unsupported, report inconclusive for that claim.

Use distribution-free order-statistic confidence bounds obtained by exact binomial tail inversion, with outward/conservative arithmetic. For n independent observations, p=0.99 and ordered values X(1)..X(n), choose the greatest lower index l with `Pr[Bin(n,p) < l] <= alpha_tail` and the least upper index u with `Pr[Bin(n,p) >= u] <= alpha_tail`. Use lower bound zero for nonnegative latency when l=0; if no finite u exists, use positive infinity and classify the gate inconclusive. Discrete distributions/ties require the conservative quantile convention above; do not jitter or discard ties. All-equal observations can still yield a valid exact bound when the independence and sample-size conditions hold; the degenerate-bootstrap warning does not invalidate such an exact bound. This procedure has no bootstrap Monte Carlo tail error.

Freeze the total number M of inferential decisions and maximum L=3 planned looks. Allocate family error 0.05 across all decisions and looks, then divide each decision's allocation among its required one-sided bounds. A p99 ratio requires four bounds: candidate lower/upper and baseline lower/upper. Use `alpha_tail = 0.05 / (4*M*L)` for those; compute the conservative ratio interval `[C_lower/B_upper, C_upper/B_lower]`. A zero/nonfinite baseline lower bound blocks a finite upper ratio. Apply the absolute candidate p99 limit using the allocated candidate upper bound. Record the exact allocation so reused bounds are not accidentally counted as independent evidence.

For M=200 and L=3, `alpha_tail` is about 0.0000208333. Even a finite upper bound requires `0.99^n <= alpha_tail`, so n must be at least 1,073 independent observations per condition. That minimum only permits using the sample maximum as an upper bound; it does not guarantee enough precision to pass. Use a pilot, before candidate selection, to choose larger fixed batches and three cumulative look sizes. With insufficient n, a degenerate resample distribution, unresolved drift or a missing upper bound, return inconclusive. Thirty blocks and 10,000 bootstrap draws do not establish a valid p99 claim.

For aggregate and mean-based metrics, predeclare a separate confidence method and its distribution assumptions, coverage justification, minimum effective sample size and failure rules. The release evaluator must reject an unspecified method. If a bootstrap method is approved for those metrics, freeze interval construction (percentile/basic/BCa), block resampling unit, pairing, seeds, adjusted tail probabilities and a numerical Monte Carlo error bound that fits inside the confidence allocation. A fixed 10,000 draws is not an accuracy guarantee. Use an exact/conservative alternative or increase draws by the prespecified rule; never choose a method after seeing which passes.

Classify a limit as pass only when its upper confidence bound is at/below the fixed limit; fail when the lower bound exceeds it; otherwise inconclusive. Stop after the third predefined look if unresolved. Retain environmental faults, excluded runs and reasons; a rerun cannot erase a real regression. No performance result is qualified until correctness has passed.

See the official [SciPy bootstrap reference](https://docs.scipy.org/doc/scipy/reference/generated/scipy.stats.bootstrap.html) for interval methods and degenerate-data behavior, and [SciPy binomial distribution reference](https://docs.scipy.org/doc/scipy/reference/generated/scipy.stats.binom.html) for the distribution used in the exact calculation. The coverage argument here is a mathematical design obligation; these library references do not validate this project's evaluator.

## Validate the metric evaluator

Use synthetic fixtures with a 30% throughput regression, a single critical 25% latency regression, a p99-only regression, invalid denominators and an interval crossing the limit. Fix expected pass/fail/inconclusive decisions before implementation. Verify that all ratio orientations and weights give the intended result. Include a case whose geometric mean passes while a critical or tail limit fails.

Benchmark the IPC and in-process paths separately where both are supported, including batching, cancellation and borrowed-buffer retention. Tune only after recording a correct baseline. Changing an algorithm or numerical mode creates a different comparison rather than an unexplained compiler speedup.

## Reproduce the missed-tail counterexample

Use independent blocks with equal request counts: baseline latency is always 1, candidate latency is 1 with probability 0.98 and 2 with probability 0.02. Candidate population p99 is 2, while 30 blocks miss every slow block with probability `0.98^30`, about 0.545484. Resampling those observed fast-only blocks cannot discover the missing tail. The revised evaluator must return inconclusive from that inadequate tail sample, never pass a ratio limit of 1.20.

Test exact tail-index boundaries, all-equal data, ties, insufficient n, zero baseline bounds, four-bound ratio propagation, cumulative looks and the full M-decision error allocation. For 10,000 bootstrap draws, 200 decisions and three looks, the old two-sided adjusted tail contains only about 0.4167 draws on average; include that as a rejection fixture for an evaluator that still assumes a fixed resample count is adequate. `Qualification.classify` checks interval decisions only; proving statistical coverage and implementing binomial inversion remain separate obligations.

## Performance gates

Measure GPU kernel time, host submission overhead, cold/warm compile and pipeline time, transfer time, end-to-end latency/throughput, peak device/host memory, and package size. Record warmup, repetitions, confidence intervals, clock/power/thermal conditions, queue depth, synchronization, and background load.

Compare against:

1. The legacy CUDA implementation on the same NVIDIA hardware for matching semantics and workloads.
2. An appropriate native baseline on other hardware.
3. The previous qualified release for regression detection.

Do not compare different FP modes, tensor paths, algorithms, or input shapes and attribute the difference solely to SPIR-T. Optimizations that increase compile time or memory need those costs reported alongside kernel speed.

**Proposed initial budgets to ratify in P0:** no more than 10% geometric-mean end-to-end regression over the agreed portable corpus; no individual critical workload more than 20% slower; no more than 10% regression in peak memory or warm host submission overhead relative to its selected baseline. Report cold compilation separately with an agreed absolute service/startup budget. These are planning targets, not measured results or universal guarantees.

Use confidence intervals and an agreed noise threshold before declaring regression. A geometric mean must not hide a severe tail-latency or correctness failure. Any exception must identify workload, cause, user-visible consequence, owner, and expiration/review milestone. Set budgets before examining release results; do not quietly move the target afterward.

Keep direct Mesa promotion separate: it needs a measured benefit over the same-driver SPIR-V route that justifies its maintenance cost. No assumed percentage gain appears in the release plan.

Use [the executable measurement procedure](03-qualify-performance-and-cutover.md) for ratio orientation, weights, tail/startup limits, confidence, remeasurement and pass/fail/inconclusive decisions. The brief budgets above do not supersede that procedure.

## Release, canary, and rollback

1. Build immutable versioned packages with exact dependencies, notices, provenance, and support/assurance manifests.
2. Reproduce portable artifacts in a clean environment. Record any target/driver artifacts whose bytes are not reproducible and why.
3. Test an installation that has no CUDA toolkit/runtime, NVCC, or Karamel requirement for the new path. Inspect build and runtime dependency graphs, not only command names.
4. Run a canary on representative workloads with explicit backend selection, numerical policy, and monitoring.
5. Promote the new path only after all release gates pass. Preserve independent rollback of core, compiler worker, and runtime package within compatibility rules.
6. Rehearse disabling a bad backend, invalidating its cache, selecting the previous qualified package, and verifying application results.
7. Publish release notes containing qualified scope, assurance status, incompatibilities, deferred legacy features, regressions/waivers, and migration steps.

Retaining an optional CUDA reference package is compatible with CUDA-free default operation. A release that still needs CUDA for a feature must say so for that feature; it cannot claim complete CUDA replacement.
## Final acceptance checklist

- [ ] G-BASELINE, G-CONTRACT, G-EXTRACT, G-VERTICAL, G-CONCURRENCY, and G-COVERAGE pass.
- [ ] O1–O10 have the required evidence/disposition for each assurance profile; G-TRUST passes.
- [ ] G-ADD-COMPILER, G-ADD-RUNTIME, G-ADD-LANGUAGE, and G-ADD-OP pass against the exact final frozen core and contract digests selected for release.
- [ ] Real-device/profile matrix and meaningful workload corpus pass with complete skip accounting.
- [ ] Correctness, ownership, failure, cleanup, and applicable soak tests pass without safety waivers. Performance exceptions alone follow the bounded waiver policy.
- [ ] Clean installation, old artifact compatibility, upgrade, rollback, and cache integrity pass.
- [ ] G-PRODUCTION and G-CUDA-FREE pass; public support claims exactly match the evidence.

- [ ] Candidate-bound evidence is accepted under [the result policy](02-implement-candidate-bound-admission.md); performance-only waivers cannot waive safety or required soak checks.
- [ ] G-REPLACEMENT passes before complete CUDA replacement is claimed; mandatory deferred legacy rows keep it blocked.

## Evidence required to close this milestone

Close **G-PRODUCTION, G-REPLACEMENT** only with the implementation artifacts, positive and negative cases, and source-to-result identities required above. Link the implementation relation to the named F* symbols and the policy's applicable O1–O10 obligations. The specification's proved lemmas are reusable model facts; they do not discharge this implementation correspondence. Record unresolved cases as blockers or explicitly outside the claim. No backend gate is marked passed by this roadmap revision.
