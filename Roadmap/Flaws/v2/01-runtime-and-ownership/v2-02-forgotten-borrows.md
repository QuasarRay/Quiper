# V2-02: a completion-token lifetime is not a leak-safe Rust ownership design

**Severity:** High. **Status:** Roadmap corrected in v3; implementation pending. **Evidence class:** binding-contract gap established against Rust's documented safety rules; no binding implementation exists to test. **Owner:** Rust binding, runtime and verification owners. **Resolve by:** G-CONTRACT and W14, before exposing safe borrowed asynchronous transfers.

V3 correction: [implementation and evidence record](../resolutions/01-close-runtime-contracts.md). The original audited evidence below is preserved against v2.

## 1. Affected instructions

The [caller procedure](https://github.com/QuasarRay/Quiper/blob/9dceaf274b46f295f7fc312fb3396d5729d7d97d/Roadmap/02-language-independent-extraction/caller-and-host-contracts.md) tells implementers to retain a borrow and preserve asynchronous lifetimes through a completion token. [Memory and bindings](https://github.com/QuasarRay/Quiper/blob/9dceaf274b46f295f7fc312fb3396d5729d7d97d/Roadmap/05-runtime-and-interop/memory-and-bindings.md) requires borrowed buffers to remain alive and proposes safe Rust ownership wrappers. Neither procedure specifies what happens when safe application code forgets that token or abandons a future whose GPU work has already begun.

See the [frozen caller procedure](https://github.com/QuasarRay/Quiper/blob/9dceaf274b46f295f7fc312fb3396d5729d7d97d/Roadmap/02-language-independent-extraction/caller-and-host-contracts.md). The problem is the missing leak-safety rule, not a claim that all token APIs are unsound.

## 2. Evidence and failure case

Rust's official [`mem::forget` documentation](https://doc.rust-lang.org/std/mem/fn.forget.html) states that safe code may suppress a destructor and that unsafe implementations must accommodate this. The [Rustonomicon's scoped-thread example](https://doc.rust-lang.org/nomicon/leaking.html) documents the corresponding failure of a borrowed completion guard.

A hypothetical API with this shape is insufficient if it starts a device write into caller-owned memory and relies on the returned token's destructor to wait:

```rust
let mut host = vec![0_u32; 1024];
let pending = download_into(&mut host); // Hypothetical: device write starts now.
std::mem::forget(pending);
drop(host);                            // The allocation can be freed.
```

Giving `pending` a lifetime parameter does not make its destructor mandatory. The caller can end the borrow by consuming the token, while the foreign operation continues. `Pin<&mut [T]>` alone does not transfer ownership of the allocation. Keeping provider code alive also does not keep this caller allocation alive.

The example is an API counterexample, not a compiled test of a Kuiper function. Rust tooling was unavailable in this audit. The official language documentation establishes the relevant destructor rule independently of that limitation.

## 3. Required correction

Specify leak safety for every safe asynchronous API before selecting its signature:

1. Prefer runtime-owned storage or transfer ownership of an owning allocation into the operation. Forgetting a token may leak that owned allocation; it must not expose a dangling borrowed pointer.
2. For borrowed storage, use a scope that cannot return while registered accesses remain, even if individual handles are forgotten. The official [`thread::scope` contract](https://doc.rust-lang.org/std/thread/fn.scope.html) is a useful ownership precedent, not a ready-made GPU implementation.
3. Copy when that is the selected contract, or make an unenforceable lifetime obligation explicitly unsafe. Do not put it behind a safe wrapper with only a documentation requirement to call `wait`.
4. Specify future cancellation, panic, timeout, unknown submission outcome and session failure. Stopping a wait does not stop DMA. A scope must not return borrowed storage merely because its drain timed out.

Keep result notification separate from the owner that retains memory and provider resources. Include leak behavior in O8/O9 and the public binding contract. Do not promise leak freedom where the language permits resource leaks; require memory safety despite them.

## 4. Closure evidence

Review the actual safe signatures and unsafe implementation together. Exercise `mem::forget`, dropped futures, panic during a scope, timeout and lost IPC replies after acceptance. Use controlled host-access instrumentation before real transfer tests.

The decisive result is that safe code cannot free or mutate storage while a permitted foreign access remains. A test that merely calls `wait` and observes correct output does not close this finding. F12's caller-obligation classification needs this additional language-specific retention case.
