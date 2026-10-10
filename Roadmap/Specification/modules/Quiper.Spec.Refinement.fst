module Quiper.Spec.Refinement
open Quiper.Spec.Foundation
open Quiper.Spec.Kernel

type outcome = | Success:content -> outcome | Rejection:identity -> outcome
  | Failure:identity -> outcome | Pending | Divergence
type observation = { history:trace; result:outcome }
type semantics = content -> observation -> prop

let refines (pre:content -> prop) (source target:semantics) : prop =
  forall input obs. pre input /\ target input obs ==> source input obs
let inhabited (pre:content -> prop) (s:semantics) : prop =
  forall input. pre input ==> (exists obs. s input obs)
let refinement_refl (p:content -> prop) (s:semantics)
  : Lemma (refines p s s) = ()
let refinement_transitive (p:content -> prop) (a b c:semantics)
  : Lemma (requires refines p a b /\ refines p b c) (ensures refines p a c) = ()

// Relation obligations are propositions, not claims supplied by a plugin.
// The execution relation and its source/KIR/target meanings must be defined
// independently before these types can certify a concrete implementation.
noeq type obligations = {
  extraction:prop; erasure:prop; kir_soundness:prop; representation:prop;
  passes:prop; concurrency:prop; numerics:prop; runtime:prop;
  bindings:prop; identity:prop; prefix_safety:prop; progress:prop
}
let complete (o:obligations) : prop =
  o.extraction /\ o.erasure /\ o.kir_soundness /\ o.representation /\
  o.passes /\ o.concurrency /\ o.numerics /\ o.runtime /\ o.bindings /\
  o.identity /\ o.prefix_safety /\ o.progress

noeq type contract = { pre:content -> prop; behavior:semantics }
let implements (c:contract) (implementation:semantics) : prop =
  inhabited c.pre implementation /\ refines c.pre c.behavior implementation

// Host calls share these semantics, including exceptional cleanup and resource
// observations. Native host compilation is subject to this same refinement.
type host_effect = | Allocate | MakeView | Copy | Prepare | Submit | Wait | ReleaseResource
  | ScalarCalculation | Branch | Loop | Import:identity -> host_effect
type caller_obligation = | StaticEvidence | DynamicCheck | CallerEvidence | TrustedCaller
noeq type host_import = {
  contract_id:identity; permitted:host_effect -> bool;
  retained:resource -> bool; may_reenter:bool; specification:contract
}
let valid_call (c:contract) (retained_input actual_input:content) : prop =
  retained_input == actual_input /\ c.pre retained_input
let retained_call_preserves_pre (c:contract) (saved actual:content)
  : Lemma (requires valid_call c saved actual) (ensures c.pre actual) = ()
