module Kuiper.Portable.Int32

#lang-pulse
open Pulse.Lib.Pervasives
open Kuiper.Base
open Kuiper.Ref
module U32 = FStar.UInt32

inline_for_extraction noextract
fn add_one (x:U32.t)
  returns y:U32.t
  ensures pure (y == U32.add_mod x 1ul)
{
  U32.add_mod x 1ul;
}

fn increment (r:gpu_ref U32.t)
  requires r |-> 'x
  ensures r |-> U32.add_mod 'x 1ul
{
  let x = Kuiper.Ref.read r;
  let y = add_one x;
  Kuiper.Ref.write r y;
}

inline_for_extraction noextract
fn add_delta (x delta:U32.t)
  returns y:U32.t
  ensures pure (y == U32.add_mod x delta)
{
  U32.add_mod x delta;
}

// Both owned refs are disjoint, and the input's owned value is preserved.
// This is a scalar ref contract. Lifting it to one ref per GPU lane is separate.
fn copy_add (input output:gpu_ref U32.t) (delta:U32.t)
  preserves input |-> 'x
  requires output |-> 'old
  ensures output |-> U32.add_mod 'x delta
{
  let x = Kuiper.Ref.read input;
  let y = add_delta x delta;
  Kuiper.Ref.write output y;
}

inline_for_extraction noextract
fn early_add_three (x:U32.t)
  returns y:U32.t
  ensures pure (y == U32.add_mod x 3ul)
{
  let y = U32.add_mod x 3ul;
  return y;
}

// Explicit returns exercise the checked forward-jump labels in both functions.
fn early_increment (r:gpu_ref U32.t)
  requires r |-> 'x
  ensures r |-> U32.add_mod 'x 3ul
{
  let x = Kuiper.Ref.read r;
  let y = early_add_three x;
  Kuiper.Ref.write r y;
  return ();
}

inline_for_extraction noextract
fn max_u32 (left right:U32.t)
  returns value:U32.t
  ensures pure (value == (if U32.lt left right then right else left))
{
  if (U32.lt left right) {
    right;
  } else {
    left;
  };
}

// The store consumes the U32 result joined from the helper's two branches.
fn max_assign (r:gpu_ref U32.t) (floor:U32.t)
  requires r |-> 'x
  ensures r |-> (if U32.lt 'x floor then floor else FStar.Ghost.reveal 'x)
{
  let x = Kuiper.Ref.read r;
  let y = max_u32 x floor;
  Kuiper.Ref.write r y;
}

// Only the selected owned ref is read or written; the other is preserved.
fn conditional_increment (left right:gpu_ref U32.t) (selector:U32.t)
  requires left |-> 'x ** right |-> 'y
  ensures
    left |-> (if U32.eq selector 0ul then U32.add_mod 'x 1ul else FStar.Ghost.reveal 'x) **
    right |-> (if U32.eq selector 0ul then FStar.Ghost.reveal 'y else U32.add_mod 'y 2ul)
{
  if (U32.eq selector 0ul) {
    let x = Kuiper.Ref.read left;
    let y = U32.add_mod x 1ul;
    Kuiper.Ref.write left y;
  } else {
    let x = Kuiper.Ref.read right;
    let y = U32.add_mod x 2ul;
    Kuiper.Ref.write right y;
  };
}

// Both thresholds use unsigned comparison; all three increments wrap in U32.
fn nested_increment (r:gpu_ref U32.t)
  requires r |-> 'x
  ensures r |->
    (if U32.lt 'x 10ul then U32.add_mod 'x 1ul
     else if U32.lt 'x 20ul then U32.add_mod 'x 2ul
     else U32.add_mod 'x 3ul)
{
  let x = Kuiper.Ref.read r;
  if (U32.lt x 10ul) {
    let y = U32.add_mod x 1ul;
    Kuiper.Ref.write r y;
  } else {
    if (U32.lt x 20ul) {
      let y = U32.add_mod x 2ul;
      Kuiper.Ref.write r y;
    } else {
      let y = U32.add_mod x 3ul;
      Kuiper.Ref.write r y;
    };
  };
}
