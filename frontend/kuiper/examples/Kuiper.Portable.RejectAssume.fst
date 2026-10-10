module Kuiper.Portable.RejectAssume

#lang-pulse
open Pulse.Lib.Pervasives
open Kuiper.Base
open Kuiper.Ref
module U32 = FStar.UInt32

// Deliberate ghost proof bypass. The adapter must reject it before erasure.
fn assumed (r:gpu_ref U32.t)
  preserves r |-> 'x
{
  Pulse.Lib.Core.assume_ (pure True);
}
