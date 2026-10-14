# Preserve generated dead tails after a return

[Source CI for commit `3e97efd8c5507d8282ac2dad8ba40f3ff6894790`](https://github.com/QuasarRay/Quiper/actions/runs/38053365271) passed the complete Control module's strict verification and executed `condition_once` and `matching_returns`. It then rejected `asymmetric_return` with `capture contains unreachable`. No complete source report was published.

The previous arithmetic-context correction worked. This failure is in the frontend's classification of checked syntax: Pulse can insert `Tm_Unreachable` while elaborating the postcondition of a jump. Rejecting every occurrence rejects a valid return tail, even when lowering never reaches that tail.

## Check the distinction in the pinned compiler

[Pulse's Goto checker](https://github.com/FStarLang/FStar/blob/0eef57bef411aac090354a75c21e00b674bd420c/pulse/src/checker/Pulse.Checker.Goto.fst#L50-L68) gives a jump an unreachable postcondition. [The prover's unreachable elaborators](https://github.com/FStarLang/FStar/blob/0eef57bef411aac090354a75c21e00b674bd420c/pulse/src/checker/Pulse.Checker.Prover.fst#L535-L549) and [post-hint continuation](https://github.com/FStarLang/FStar/blob/0eef57bef411aac090354a75c21e00b674bd420c/pulse/src/checker/Pulse.Checker.Prover.fst#L1477-L1521) construct unreachable syntax from that context. These explain why the checked capture contains the node; they do not prove this adapter's control-flow correspondence.

## Admit only a structurally dead generated tail

Keep whole-tree inspection for admit, assume, magic, unsafe coercion and unsupported constructs. Permit an unreachable node during that inspection only when it is marked as generated and lies in a continuation dominated by a jump that cannot return there. A bind's head and two nonreturning conditional arms determine that structural property. A label consumes its own return jumps, so it must not be classified as an unconditional jump out of its enclosing continuation.

Inspect the unreachable node's computation in the native observer as well. Do not give the node executable semantics. If CPS lowering encounters it, reject it before ordinary ghost erasure. This prevents a reachable generated node from becoming successful execution.

Continue to reject explicit bypass symbols inside dead tails. Continue to reject source unreachable nodes and generated unreachable nodes outside jump-dominated tails. This rule does not accept arbitrary unreachability assertions or establish imported proof closure.

## Replay the correction

Require the three new boundary cases: a generated tail after a scoped return emits no instructions; reachable or source tails reject; and a dead tail cannot conceal an explicit bypass. The source boundary suite now contains fourteen tests.

Require the complete source matrix, including both asymmetric-return selectors and their short-view cases, before claiming source execution coverage. The Operations extension raises the matrix to 45 cases across 17 entries. Its hardware replay must bind all 38 executable/view vectors to the successful CPU source report and the exact checkout. No new passing report is claimed until those jobs complete.
