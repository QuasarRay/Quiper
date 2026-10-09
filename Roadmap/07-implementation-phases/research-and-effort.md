# Research and effort

## PX. Direct Mesa research

**Depends on:** the stable P4 baseline; never blocks P8.

Deliver one-driver prototype, semantics comparison, generated-code/performance evidence, maintenance estimate, and a promote/defer decision. Native ISA generation and direct backend-IR routes receive separate work packages rather than being hidden inside the portable migration.

**G-MESA-DECISION:** measured evidence supports the decision. A negative result is a valid completed experiment.
## Effort and sequencing assumptions

These are planning ranges, not measured implementation estimates or delivery promises. They assume engineers already familiar with compiler construction, GPU memory models, F*/Pulse, and driver APIs, plus access to required hardware.

| Phase | Initial engineering effort range |
|---|---:|
| P0 | 2–4 person-weeks |
| P1 | 4–8 person-weeks |
| P2 | 4–8 person-weeks |
| P3 | 6–12 person-weeks |
| P4 | 8–16 person-weeks |
| P5 | 8–20 person-weeks |
| P6 | 10–24 person-weeks |
| P7 | 6–14 person-weeks |
| P8 | 8–16 person-weeks |

The sum is 56–122 person-weeks before the optional Mesa track. Concurrent work can shorten calendar time but not remove proof and hardware dependencies. Full legacy parity, new proof foundations, upstream compiler changes, or unavailable driver features can exceed this range substantially. Re-estimate after P0 and after the P3 end-to-end path; proof research should not be treated as fixed-duration routine implementation.

Keep review units small and evidence-complete. Parallel teams can own semantics, extraction/compiler, runtime/bindings, and hardware qualification, with one owner for each interface. Do not merge cross-cutting implementation before the owning contracts are reviewable.
