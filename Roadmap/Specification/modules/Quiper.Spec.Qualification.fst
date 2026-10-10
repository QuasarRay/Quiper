module Quiper.Spec.Qualification
open Quiper.Spec.Foundation

type decision = | Pass | Fail | Inconclusive
// Rational bounds share a positive denominator. Confidence coverage is a
// separate statistical obligation; an interval is not sound merely by type.
type interval = { lower:nat; upper:nat; denominator:pos }
let valid_interval (i:interval) : bool = i.lower <= i.upper
let classify (i:interval) (limit_n:nat) (limit_d:pos) : decision =
  if not (valid_interval i) then Inconclusive
  else if i.upper * limit_d <= limit_n * i.denominator then Pass
  else if limit_n * i.denominator < i.lower * limit_d then Fail
  else Inconclusive
let uncertain_cannot_pass (i:interval) (n:nat) (d:pos)
  : Lemma (requires n * i.denominator < i.upper * d) (ensures classify i n d <> Pass) = ()

type confidence = | NoFiniteBound | Finite:interval -> confidence
let decide (data:confidence) (n:nat) (d:pos) : decision =
  match data with
  | NoFiniteBound -> Inconclusive
  | Finite bounds -> classify bounds n d
let missing_tail_bound_inconclusive (n:nat) (d:pos)
  : Lemma (decide NoFiniteBound n d = Inconclusive) = ()

type evidence_status = | Missing | Stale | FailedEvidence | InconclusiveEvidence | Passed
noeq type release = {
  mandatory:nat -> bool; evidence:nat -> evidence_status;
  current:nat -> bool; in_scope:nat -> bool; replaced:nat -> bool
}
let qualified (r:release) : prop =
  forall gate. r.mandatory gate ==> r.evidence gate = Passed /\ r.current gate
let full_replacement (r:release) : prop =
  qualified r /\ (forall row. r.in_scope row ==> r.replaced row)
let missing_blocks_release (r:release) (g:nat)
  : Lemma (requires r.mandatory g /\ r.evidence g = Missing) (ensures not (qualified r)) = ()
let deferred_blocks_replacement (r:release) (row:nat)
  : Lemma (requires r.in_scope row /\ not (r.replaced row)) (ensures not (full_replacement r)) = ()
