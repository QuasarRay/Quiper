module Kuiper.Portable.Reject

#lang-pulse
open Pulse.Lib.Pervasives
open Kuiper.Base
open Kuiper.Ref
module U32 = FStar.UInt32
module U64 = FStar.UInt64

// Verified U64 code must be rejected by the U32-only source adapter.
fn increment_u64 (r:gpu_ref U64.t)
  requires r |-> 'x
  ensures r |-> U64.add_mod 'x 1uL
{
  let x = Kuiper.Ref.read r;
  let y = U64.add_mod x 1uL;
  Kuiper.Ref.write r y;
}

// The implicit permission binder needs specialization outside this profile.
fn fractional_reads (input output:gpu_ref U32.t) (delta:U32.t)
  preserves input |-> Frac 'f 'x
  requires output |-> 'old
  ensures output |-> U32.add_mod 'x delta
{
  let x = Kuiper.Ref.read input;
  let y = U32.add_mod x delta;
  Kuiper.Ref.write output y;
}

// This deliberate proof bypass is a negative fixture, never proof evidence.
fn admitted (r:gpu_ref U32.t)
  requires r |-> 'x
  ensures r |-> U32.add_mod 'x 1ul
{
  admit();
}
