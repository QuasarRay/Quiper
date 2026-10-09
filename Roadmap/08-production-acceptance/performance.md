# Performance

## 5. Performance gates

Measure GPU kernel time, host submission overhead, cold/warm compile and pipeline time, transfer time, end-to-end latency/throughput, peak device/host memory, and package size. Record warmup, repetitions, confidence intervals, clock/power/thermal conditions, queue depth, synchronization, and background load.

Compare against:

1. The legacy CUDA implementation on the same NVIDIA hardware for matching semantics and workloads.
2. An appropriate native baseline on other hardware.
3. The previous qualified release for regression detection.

Do not compare different FP modes, tensor paths, algorithms, or input shapes and attribute the difference solely to SPIR-T. Optimizations that increase compile time or memory need those costs reported alongside kernel speed.

**Proposed initial budgets to ratify in P0:** no more than 10% geometric-mean end-to-end regression over the agreed portable corpus; no individual critical workload more than 20% slower; no more than 10% regression in peak memory or warm host submission overhead relative to its selected baseline. Report cold compilation separately with an agreed absolute service/startup budget. These are planning targets, not measured results or universal guarantees.

Use confidence intervals and an agreed noise threshold before declaring regression. A geometric mean must not hide a severe tail-latency or correctness failure. Any exception must identify workload, cause, user-visible consequence, owner, and expiration/review milestone. Set budgets before examining release results; do not quietly move the target afterward.

Keep direct Mesa promotion separate: it needs a measured benefit over the same-driver SPIR-V route that justifies its maintenance cost. No assumed percentage gain appears in the release plan.

Use [the executable measurement procedure](performance-procedure.md) for ratio orientation, weights, tail/startup limits, confidence, remeasurement and pass/fail/inconclusive decisions. The brief budgets above do not supersede that procedure.
