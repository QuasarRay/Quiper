module Kuiper.Portable.Control

#lang-pulse
open Pulse.Lib.Pervasives
open Kuiper.Base
open Kuiper.Ref
module U32 = FStar.UInt32

// Replaying the Pure call exposes the documented UInt32 projection contract
// when its result is used in a Pulse ghost arithmetic proof.
let add_mod_projection (a b:U32.t)
  : Lemma (U32.v (U32.add_mod a b) == (U32.v a + U32.v b) % 4294967296)
  = let result = U32.add_mod a b in
    FStar.UInt.pow2_values 32;
    ()

let zero_increment_math (original first last:U32.t)
  : Lemma
    (requires (U32.eq original 0ul /\
               first == U32.add_mod original 1ul /\
               last == U32.add_mod first 1ul))
    (ensures (last == 2ul))
  = FStar.UInt.pow2_values 32;
    assert (U32.v original == 0);
    U32.v_inj original 0ul;
    add_mod_projection original 1ul;
    assert (U32.v first == 1);
    add_mod_projection first 1ul;
    FStar.Math.Lemmas.modulo_lemma 2 4294967296;
    assert (U32.v last == 2);
    U32.v_inj last 2ul

let increment_three_math (original first last:U32.t)
  : Lemma
    (requires (first == U32.add_mod original 1ul /\
               last == U32.add_mod first 2ul))
    (ensures (last == U32.add_mod original 3ul))
  = FStar.UInt.pow2_values 32;
    add_mod_projection original 1ul;
    add_mod_projection first 2ul;
    add_mod_projection original 3ul;
    FStar.Math.Lemmas.lemma_mod_add_distr 2 (U32.v original + 1) 4294967296;
    assert (U32.v last == U32.v (U32.add_mod original 3ul));
    U32.v_inj last (U32.add_mod original 3ul)

ghost fn prove_zero_increment (original:erased U32.t) (first last:U32.t)
  requires pure (U32.eq (FStar.Ghost.reveal original) 0ul /\
                 first == U32.add_mod (FStar.Ghost.reveal original) 1ul /\
                 last == U32.add_mod first 1ul)
  ensures pure (last == 2ul)
{
  zero_increment_math (FStar.Ghost.reveal original) first last;
}

ghost fn prove_increment_three (original:erased U32.t) (first last:U32.t)
  requires pure (first == U32.add_mod (FStar.Ghost.reveal original) 1ul /\
                 last == U32.add_mod first 2ul)
  ensures pure (last == U32.add_mod (FStar.Ghost.reveal original) 3ul)
{
  increment_three_math (FStar.Ghost.reveal original) first last;
}

inline_for_extraction noextract
fn read_increment_is_zero (r:gpu_ref U32.t)
  requires r |-> 'x
  returns was_zero:bool
  ensures r |-> U32.add_mod 'x 1ul ** pure (was_zero == U32.eq 'x 0ul)
{
  let x = Kuiper.Ref.read r;
  let y = U32.add_mod x 1ul;
  Kuiper.Ref.write r y;
  U32.eq x 0ul;
}

// The condition's update executes once, before the selected branch's update.
fn condition_once (r:gpu_ref U32.t)
  requires r |-> 'x
  ensures r |-> (if U32.eq 'x 0ul then 2ul else U32.add_mod 'x 3ul)
{
  if (read_increment_is_zero r) {
    let x = Kuiper.Ref.read r;
    let y = U32.add_mod x 1ul;
    prove_zero_increment 'x x y;
    Kuiper.Ref.write r y;
  } else {
    let x = Kuiper.Ref.read r;
    let y = U32.add_mod x 2ul;
    prove_increment_three 'x x y;
    Kuiper.Ref.write r y;
  };
}

inline_for_extraction noextract
fn choose_return (x:U32.t)
  returns value:U32.t
  ensures pure (value == (if U32.eq x 0ul then 7ul else 9ul))
{
  if (U32.eq x 0ul) {
    return 7ul;
  } else {
    return 9ul;
  };
}

// Both helper branches jump to the same return label, yielding a joined U32.
fn matching_returns (r:gpu_ref U32.t)
  requires r |-> 'x
  ensures r |-> (if U32.eq 'x 0ul then 7ul else 9ul)
{
  let x = Kuiper.Ref.read r;
  let y = choose_return x;
  Kuiper.Ref.write r y;
}

// The early return skips the write after the conditional. Only a nonzero
// selector executes the continuation and adds five to the original value.
fn asymmetric_return (r:gpu_ref U32.t) (selector:U32.t)
  requires r |-> 'x
  ensures r |-> (if U32.eq selector 0ul then FStar.Ghost.reveal 'x else U32.add_mod 'x 5ul)
{
  if (U32.eq selector 0ul) {
    return ();
  } else {
    ();
  };
  let x = Kuiper.Ref.read r;
  let y = U32.add_mod x 5ul;
  Kuiper.Ref.write r y;
}
