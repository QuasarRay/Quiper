module Quiper.Spec.Runtime
open Quiper.Spec.Foundation

type phase = | Absent | Accepted | InFlight | CompletedUnchecked | Succeeded | Failed | Rejected
let terminal (p:phase) : bool = p = Succeeded || p = Failed || p = Rejected
type retention = | RuntimeOwned | ScopeOwned | TokenOnly
noeq type operation = {
  phase:phase; request:content; dependencies:nat -> bool;
  visibility:bool; guard_ok:bool; post_checked:bool;
  redeemed:bool; held:bool; retention:retention
}
noeq type session = {
  generation:nat; alive:bool; retired:nat; window:pos;
  operations:nat -> operation
}
let deps_succeeded (s:session) (o:operation) : prop =
  forall d. o.dependencies d ==> (s.operations d).phase = Succeeded
let safe_retention (o:operation) : bool =
  o.phase = Absent || terminal o.phase ||
  (o.held && (o.retention = RuntimeOwned || o.retention = ScopeOwned))
let fresh (s:session) (id:nat) : bool =
  s.alive && s.retired < id && id <= s.retired + s.window &&
  (s.operations id).phase = Absent
let runnable (s:session) (o:operation) : prop =
  s.alive /\ o.phase = Accepted /\ o.held /\ safe_retention o /\ deps_succeeded s o
let publishable (o:operation) : bool =
  o.phase = CompletedUnchecked && o.visibility && o.guard_ok && o.post_checked
let releaseable (o:operation) : bool =
  terminal o.phase && not o.redeemed

// These are transition obligations, independent of threads, queues or IPC.
// Actual submit must also reserve disjoint/conflict-ordered resource entitlements
// and validate the source call precondition against the retained input version.
type action = | Begin | FinishDevice | PublishSuccess | PublishFailure | Redeem | Observe
let operation_step (a:action) (before after:operation) : prop =
  match a with
  | Begin -> before.phase = Accepted /\ after == {before with phase=InFlight}
  | FinishDevice -> before.phase = InFlight /\ after == {before with phase=CompletedUnchecked}
  | PublishSuccess -> publishable before /\ after == {before with phase=Succeeded}
  | PublishFailure ->
    (before.phase = Accepted \/ before.phase = InFlight \/ before.phase = CompletedUnchecked) /\
    after == {before with phase=Failed}
  | Redeem -> releaseable before /\ after == {before with redeemed=true; held=false}
  | Observe -> after == before

// Completion of failed GPU work and its safe teardown are distinct. Failed
// status alone never authorizes releasing storage still touched by the device.
let can_dispose (o:operation) (device_quiescent:bool) : bool =
  terminal o.phase && device_quiescent
let safe_step (s:session) (a:action) (b c:operation) (quiescent:bool) : prop =
  operation_step a b c /\
  (a = Begin ==> runnable s b) /\
  (a = Redeem ==> can_dispose b quiescent)

let may_retire (s:session) (next:nat) : prop =
  s.retired <= next /\ next <= s.retired + s.window /\
  (forall id. s.retired < id /\ id <= next ==>
    terminal (s.operations id).phase /\ (s.operations id).redeemed) /\
  // Keep dependency-success facts until every remaining dependent is resolved.
  (forall id d. next < id /\ id <= s.retired + s.window /\
    not (terminal (s.operations id).phase) /\ (s.operations id).dependencies d ==> next < d)

let failed_dependency_blocks (s:session) (o:operation) (d:nat)
  : Lemma (requires o.dependencies d /\ (s.operations d).phase = Failed)
          (ensures not (runnable s o)) = ()
let completion_is_not_success (o:operation)
  : Lemma (requires o.phase = CompletedUnchecked) (ensures o.phase <> Succeeded) = ()
let forgotten_handle_retains (o:operation) (after:operation)
  : Lemma (requires safe_retention o /\ operation_step Observe o after)
          (ensures safe_retention after) = ()
let redeemed_once (b c:operation)
  : Lemma (requires operation_step Redeem b c) (ensures not (releaseable c)) = ()
let retired_never_fresh (s:session) (id:nat)
  : Lemma (requires id <= s.retired) (ensures not (fresh s id)) = ()
let timeout_cannot_release (o:operation)
  : Lemma (requires o.phase = InFlight) (ensures not (releaseable o)) = ()

type host_object = { owner:nat; alive:bool }
let permitted_host_call (o:host_object) (thread:nat) : bool = o.alive && o.owner = thread
let exclusive_owner (o:host_object) (a b:nat)
  : Lemma (requires permitted_host_call o a /\ permitted_host_call o b) (ensures a = b) = ()

let safe_step_preserves_retention (s:session) (a:action) (b c:operation) (quiet:bool)
  : Lemma (requires safe_retention b /\ safe_step s a b c quiet)
          (ensures safe_retention c) =
  match a with
  | Begin -> ()
  | FinishDevice -> ()
  | PublishSuccess -> ()
  | PublishFailure -> ()
  | Redeem -> ()
  | Observe -> ()

let success_requires_guard (b c:operation)
  : Lemma (requires operation_step PublishSuccess b c)
          (ensures c.guard_ok /\ c.visibility /\ c.post_checked) = ()
