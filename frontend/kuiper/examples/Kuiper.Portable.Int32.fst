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
