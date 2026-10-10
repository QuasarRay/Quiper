module Quiper.Spec.Operations
open Quiper.Spec.Foundation
open Quiper.Spec.Memory

// A semantic family is independent of a frontend, compiler or GPU vendor.
type family = | Scalar | Vector | View | PrivateMemory | GlobalMemory | SharedMemory
  | AtomicOperation | Barrier | SubgroupOperation | MatrixOperation
  | Assertion | Guard | HostOperation | UserExtension
type signature = { inputs:list scalar_type; outputs:list scalar_type; family:family }
let rec values_typed (vs:list scalar) (ts:list scalar_type) : Tot bool =
  match vs,ts with
  | [], [] -> true
  | v::rest, t::more -> well_typed v t && values_typed rest more
  | _ -> false
noeq type operation_definition = {
  id:identity; signature:signature;
  pre:list scalar -> prop;
  result:list scalar -> list scalar -> prop;
  events_legal:execution -> prop;
  typing:(forall args out. pre args /\ result args out ==>
    values_typed args signature.inputs /\ values_typed out signature.outputs);
  // This witness rules out an empty successful value relation on legal inputs.
  total_on_pre: args:list scalar -> h:pre args -> GTot (out:list scalar{result args out})
}

let subtract_u32 (x:nat{x < word_modulus}) (y:nat{y < word_modulus}) : nat =
  (x + word_modulus - y) % word_modulus
let multiply_u32 (x y:nat) : nat = (x * y) % word_modulus
let signed32 (bits:nat{bits < word_modulus}) : int =
  if bits < 2147483648 then bits else bits - word_modulus
let division_pre (x y:int) : bool =
  -2147483648 <= x && x < 2147483648 && -2147483648 <= y && y < 2147483648 &&
  y <> 0 && not (x = -2147483648 && y = -1)
// Quotient/remainder relation fixes truncation toward zero without depending
// on the host language's division convention.
let signed_division (x y q r:int) : prop =
  division_pre x y /\ x = q * y + r /\
  (if y < 0 then y < r /\ r < -y else -y < r /\ r < y) /\
  (x >= 0 ==> r >= 0) /\ (x <= 0 ==> r <= 0)
let shift_pre (amount:nat) : bool = amount < 32

let assert_pre (already_proved:prop) : prop = already_proved
type guard_result = | GuardSuccess | GuardFailure
let guard_relation (predicate:bool) (out:guard_result) : bool =
  if predicate then out = GuardSuccess else out = GuardFailure
let guard_success_establishes (p:bool) (out:guard_result)
  : Lemma (requires guard_relation p out /\ out = GuardSuccess) (ensures p) = ()

// Atomic value relations complement, rather than replace, Memory.admissible.
let compare_exchange (old expected replacement returned next:nat) : bool =
  old < word_modulus && expected < word_modulus && replacement < word_modulus &&
  returned = old && next = (if old = expected then replacement else old)
let atomic_add (old delta returned next:nat) : bool =
  old < word_modulus && delta < word_modulus && returned = old && next = add_u32 old delta

type matrix_shape = { rows:nat; columns:nat; inner:nat }
type integer_matrix = nat -> nat -> int
let rec dot (a b:integer_matrix) (row col k:nat) : Tot int (decreases k) =
  if k = 0 then 0 else dot a b row col (k-1) + a row (k-1) * b (k-1) col
let exact_gemm (shape:matrix_shape) (a b c:integer_matrix) : prop =
  forall row col. row < shape.rows /\ col < shape.columns ==> c row col == dot a b row col shape.inner

// Floating/mixed precision matrix instructions require a separately identified
// numerical relation. Storage, multiplication and accumulator formats differ.
noeq type matrix_policy = {
  id:identity; input_format:identity; product_format:identity; accumulator_format:identity;
  supported:matrix_shape -> bool;
  relation:matrix_shape -> content -> content -> content -> prop
}
noeq type subgroup_contract = {
  size:pos; members:nat -> bool; source_lane:nat -> nat;
  result:nat -> scalar; input:nat -> scalar
}
let shuffle_relation (g:subgroup_contract) : prop =
  forall lane. g.members lane ==> lane < g.size /\ g.source_lane lane < g.size /\
    g.members (g.source_lane lane) /\ g.result lane == g.input (g.source_lane lane)

// Profile coverage is exhaustive over the frozen source dependency closure.
// A name in the ledger is insufficient: it resolves to a concrete definition.
let catalog_covers (required:identity -> bool) (catalog:identity -> option operation_definition) : prop =
  forall id. required id ==> (exists (d:operation_definition). catalog id == Some d /\ d.id = id)
