module Quiper.Spec.Lowering
open Quiper.Spec.Foundation

// Target-neutral semantic requirements are separate from this realization.
// This module specifies the investigated SPIR-T/Vulkan implementation profile.
type requirements = { uses_device_scope:bool; uses_visibility_chains:bool }
type features = { memory_model:bool; device_scope:bool; visibility_chains:bool }
let sufficient (r:requirements) (f:features) : bool =
  f.memory_model && (not r.uses_device_scope || f.device_scope) &&
  (not r.uses_visibility_chains || f.visibility_chains)
let chains_require_feature (r:requirements) (f:features)
  : Lemma (requires r.uses_visibility_chains /\ sufficient r f) (ensures f.visibility_chains) = ()

// A source while tests before its first body. SPIR-T Loop tests after a body.
// The outer selection's false arm yields initial values. Its true arm yields
// the final body values, never fictitious Loop node outputs.
type loop_values = { initial:list scalar; body_final:list scalar; initial_test:bool }
let while_exit (v:loop_values) : list scalar = if v.initial_test then v.body_final else v.initial
let zero_iteration_values (v:loop_values)
  : Lemma (requires not v.initial_test) (ensures while_exit v == v.initial) = ()
let positive_iteration_values (v:loop_values)
  : Lemma (requires v.initial_test) (ensures while_exit v == v.body_final) = ()

type carried_values = { inputs:list scalar; backedge:list scalar; input_types:list scalar_type }
let rec typed_values (vs:list scalar) (ts:list scalar_type) : Tot bool =
  match vs,ts with
  | [], [] -> true
  | v::rest, t::more -> well_typed v t && typed_values rest more
  | _ -> false
let valid_carried (v:carried_values) : bool =
  typed_values v.inputs v.input_types && typed_values v.backedge v.input_types
let rec typed_values_arity (vs:list scalar) (ts:list scalar_type)
  : Lemma (requires typed_values vs ts) (ensures FStar.List.Tot.length vs = FStar.List.Tot.length ts) =
  match vs,ts with
  | _::rest, _::more -> typed_values_arity rest more
  | _ -> ()

type target_layout = { logical:view; stride:pos; binding_offset:nat; binding_length:nat; alignment:pos }
let layout_valid (l:target_layout) : bool =
  l.stride = l.logical.cell_bytes && l.binding_offset % l.alignment = 0 &&
  (l.logical.start + l.logical.count) * l.stride <= l.binding_length

type annotation = | LiteralOperands | IdOperands
let baseline_annotation_supported (a:annotation) : bool = a = LiteralOperands
type pointer_path = | TypedLogicalAccess | QPtrEligible | RejectedPointerShape
let pointer_path_allowed (p:pointer_path) : bool = p <> RejectedPointerShape
