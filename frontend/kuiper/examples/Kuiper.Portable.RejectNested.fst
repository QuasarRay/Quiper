module Kuiper.Portable.RejectNested

#lang-pulse
open Pulse.Lib.Pervasives
open Kuiper.Base
open Kuiper.Ref
module U32 = FStar.UInt32

// The closed encoder cannot represent this lambda. Full pure inspection must
// still find the deliberate proof bypass inside it before any erasure.
fn nested_bypass (r:gpu_ref U32.t)
  preserves r |-> 'x
{
  let hidden = (fun (_:unit) -> let witness = Prims.magic #unit () in witness);
  ();
}
