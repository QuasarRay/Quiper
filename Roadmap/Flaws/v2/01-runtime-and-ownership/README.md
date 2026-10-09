# Runtime and ownership findings

Specify success, lifetime and retirement together. Completion alone cannot establish a successful producer postcondition, and a language-level borrow alone cannot retain memory after an asynchronous token is forgotten.

| Finding | Contract to settle | Decisive case |
|---|---|---|
| [V2-01: dependent failure](v2-01-dependent-failure.md) | Execution permission after a producer fails | A queued consumer must not use invalid output |
| [V2-02: forgotten borrows](v2-02-forgotten-borrows.md) | Safe Rust retention independent of token destruction | Forget the token while foreign memory access remains |
| [V2-03: operation retirement](v2-03-operation-retirement.md) | Bounded retry history and one-time ownership transfer | Replay an old ID after terminal-state retirement |

Return to the [v2 index](../README.md). Use the [remediation order](../06-remediation/README.md) before freezing the runtime contract.
