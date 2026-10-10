module Quiper.Spec.Memory
open Quiper.Spec.Foundation

type scope = | Invocation | Subgroup | Workgroup | Device | System
type order = | Relaxed | Acquire | Release | AcquireRelease | Sequential
type memory_event = {
  actor:nat; workgroup:nat; subgroup:nat;
  object:resource; first:nat; count:nat; mode:access;
  observed:option scalar; written:option scalar;
  atomic_operation:option identity; synchronization_scope:scope; ordering:order
}
noeq type execution = {
  present:nat -> bool;
  events:nat -> memory_event;
  happens_before:nat -> nat -> prop;
  reads_from:nat -> nat -> prop;
  modification_before:nat -> nat -> prop
}
let overlaps (a b:memory_event) : bool =
  a.count > 0 && b.count > 0 && a.object = b.object &&
  a.first < b.first + b.count && b.first < a.first + a.count
let conflicts (a b:memory_event) : bool =
  overlaps a b && (a.mode <> Read || b.mode <> Read)

let race_free (x:execution) : prop =
  forall i j. x.present i /\ x.present j /\ i <> j /\
    conflicts (x.events i) (x.events j) ==>
    ((x.events i).mode = Atomic /\ (x.events j).mode = Atomic) \/
    x.happens_before i j \/ x.happens_before j i

let ordered (x:execution) : prop =
  (forall i. not (x.happens_before i i)) /\
  (forall i j k. x.happens_before i j /\ x.happens_before j k ==> x.happens_before i k)

// Atomic compatibility is not enough to prove legal outcomes. Each profile
// supplies read-from/coherence, visibility, scope and numerical axioms as a
// concrete relation, with an adequacy argument against its official model.
let admissible (profile:execution -> prop) (x:execution) : prop =
  ordered x /\ race_free x /\ profile x

noeq type collective = { members:nat -> bool; arrived:nat -> bool; same_instance:nat -> bool }
let convergent (c:collective) : prop =
  forall lane. c.members lane ==> c.arrived lane /\ c.same_instance lane

noeq type float_policy = {
  identity:identity;
  permits:nat -> nat -> nat -> prop;
  // Inputs/results are IEEE bit patterns. A policy must define signed zero,
  // NaN, infinity, subnormals, rounding, contraction and approximation bounds.
  defined: x:nat{x < word_modulus} -> y:nat{y < word_modulus} ->
    GTot (r:nat{r < word_modulus /\ permits x y r})
}

type physical_range = { first_byte:nat; length:nat }
let covers (physical logical:physical_range) : bool =
  physical.first_byte <= logical.first_byte &&
  logical.first_byte + logical.length <= physical.first_byte + physical.length
let isolated (a b:physical_range) : bool =
  a.first_byte + a.length <= b.first_byte || b.first_byte + b.length <= a.first_byte

let isolated_covers_do_not_overlap (a b x y:physical_range)
  : Lemma (requires covers a x /\ covers b y /\ isolated a b)
          (ensures isolated x y) = ()
