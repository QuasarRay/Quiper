module Kuiper.Portable.CheckedViews
open Quiper.Spec.Foundation
open Quiper.Spec.Operations

let maximum_words : nat = 1048576
let checked (start count extent index:nat) : bool =
  start + count <= extent && extent <= maximum_words && index < count
let address (start count extent index:nat) : option nat =
  if checked start count extent index then Some (start + index) else None

let checked_address_contained (r:resource) (start count extent index:nat)
  : Lemma (requires checked start count extent index)
          (ensures contains {backing=r;start=start;count=count;cell_bytes=4}
                            {object=r;cell=start+index}) = ()
let checked_address_no_word_wrap (start count extent index:nat)
  : Lemma (requires checked start count extent index)
          (ensures start + index < word_modulus) = ()
let rejected_address_is_none (start count extent index:nat)
  : Lemma (requires not (checked start count extent index))
          (ensures address start count extent index == None) = ()
let different_global_indices (start a b:nat)
  : Lemma (requires a <> b) (ensures start+a <> start+b) = ()
let checked_store_frame (h:store) (p q:location) (v:scalar)
  : Lemma (requires p <> q) (ensures write_cell h p v q == h q) =
  write_frame h p q v

let bool_bits (b:bool) : nat = if b then 1 else 0
let bool_roundtrip (b:bool) : Lemma ((bool_bits b <> 0) = b) = ()
let signed_bits (x:int{-2147483648 <= x && x < 2147483648})
  : n:nat{n < word_modulus} = if x < 0 then x + word_modulus else x
let signed_roundtrip (x:int{-2147483648 <= x && x < 2147483648})
  : Lemma (signed32 (signed_bits x) == x) = ()
let bits_roundtrip (n:nat{n < word_modulus})
  : Lemma (signed_bits (signed32 n) == n) = ()
