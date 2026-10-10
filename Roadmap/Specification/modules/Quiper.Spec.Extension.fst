module Quiper.Spec.Extension
open Quiper.Spec.Foundation
open Quiper.Spec.Refinement

type definition = { semantic_identity:identity; bytes:content }
type registry = identity -> option definition
let additive (old next:registry) : prop =
  forall k d. old k == Some d ==> next k == Some d
let addition_preserves_lookup (old next:registry) (k:identity) (d:definition)
  : Lemma (requires additive old next /\ old k == Some d) (ensures next k == Some d) = ()
let additions_compose (a b c:registry)
  : Lemma (requires additive a b /\ additive b c) (ensures additive a c) = ()

type role = | Frontend | Compiler | Runtime | Binding | Checker | SemanticExtension | HostCompiler
type endpoint = { role:role; consumes:identity; produces:identity; package:identity }
let composable (producer consumer:endpoint) : bool = producer.produces = consumer.consumes
type capability = {
  operation:identity; data_type:identity; storage:identity; scope:identity;
  ordering:identity; numeric:identity; shape:content; participation:identity
}
let supports (required:list capability) (enabled:capability -> bool) : prop =
  forall r. FStar.List.Tot.mem r required ==> enabled r

noeq type policy = { id:identity; required:obligations; permits_assumption:identity -> bool }
type policy_registry = identity -> option policy
let admitted_policy (trusted:policy_registry) (claimed:identity) (p:policy) : prop =
  trusted claimed == Some p /\ p.id = claimed
let unknown_policy_rejected (trusted:policy_registry) (id:identity) (p:policy)
  : Lemma (requires trusted id == None) (ensures not (admitted_policy trusted id p)) = ()

type evidence = {
  subject:content; output:content; checker:identity; policy:identity;
  assumptions:list identity; proof_payload:content
}
let evidence_bound (e:evidence) (input output:content) (checker policy:identity) : bool =
  e.subject = input && e.output = output && e.checker = checker && e.policy = policy
let acceptable_evidence (e:evidence) (input output:content) (p:policy)
  (trusted_checker:identity -> content -> prop) : prop =
  evidence_bound e input output e.checker p.id /\
  (forall a. FStar.List.Tot.mem a e.assumptions ==> p.permits_assumption a) /\
  trusted_checker e.checker e.proof_payload /\ complete p.required

// Files in the protected installation are mathematical byte sequences.
// Deployment adds external roots/configuration. It cannot edit these files.
type installation = string -> option content
let unchanged (old next:installation) (protected:string -> bool) : prop =
  forall path. protected path ==> old path == next path
let decoupled_addition (old next:installation) (protected:string -> bool)
  (old_registry next_registry:registry) : prop =
  unchanged old next protected /\ additive old_registry next_registry
