module Kuiper.Portable.Operations

#lang-pulse
open Pulse.Lib.Pervasives
open Kuiper.Base
open Kuiper.Ref
module U32 = FStar.UInt32

fn divide_three (r:gpu_ref U32.t)
  requires r |-> 'x
  ensures r |-> U32.div 'x 3ul
{
  let x = Kuiper.Ref.read r;
  let y = U32.div x 3ul;
  Kuiper.Ref.write r y;
}

fn remainder_three (r:gpu_ref U32.t)
  requires r |-> 'x
  ensures r |-> U32.rem 'x 3ul
{
  let x = Kuiper.Ref.read r;
  let y = U32.rem x 3ul;
  Kuiper.Ref.write r y;
}

// 16711935 = 0x00ff00ff.
fn and_mask (r:gpu_ref U32.t)
  requires r |-> 'x
  ensures r |-> U32.logand 'x 16711935ul
{
  let x = Kuiper.Ref.read r;
  let y = U32.logand x 16711935ul;
  Kuiper.Ref.write r y;
}

// 2147549184 = 0x80010000.
fn or_mask (r:gpu_ref U32.t)
  requires r |-> 'x
  ensures r |-> U32.logor 'x 2147549184ul
{
  let x = Kuiper.Ref.read r;
  let y = U32.logor x 2147549184ul;
  Kuiper.Ref.write r y;
}

// 2863311530 = 0xaaaaaaaa.
fn xor_mask (r:gpu_ref U32.t)
  requires r |-> 'x
  ensures r |-> U32.logxor 'x 2863311530ul
{
  let x = Kuiper.Ref.read r;
  let y = U32.logxor x 2863311530ul;
  Kuiper.Ref.write r y;
}

fn not_bits (r:gpu_ref U32.t)
  requires r |-> 'x
  ensures r |-> U32.lognot 'x
{
  let x = Kuiper.Ref.read r;
  let y = U32.lognot x;
  Kuiper.Ref.write r y;
}

fn shift_left_one (r:gpu_ref U32.t)
  requires r |-> 'x
  ensures r |-> U32.shift_left 'x 1ul
{
  let x = Kuiper.Ref.read r;
  let y = U32.shift_left x 1ul;
  Kuiper.Ref.write r y;
}

fn shift_right_31 (r:gpu_ref U32.t)
  requires r |-> 'x
  ensures r |-> U32.shift_right 'x 31ul
{
  let x = Kuiper.Ref.read r;
  let y = U32.shift_right x 31ul;
  Kuiper.Ref.write r y;
}
