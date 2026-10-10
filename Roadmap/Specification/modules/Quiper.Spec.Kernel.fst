module Quiper.Spec.Kernel
open Quiper.Spec.Foundation

type environment = nat -> option scalar
noeq type state = { locals:environment; memory:store }
type expression = | Literal:scalar -> expression | Variable:nat -> expression
  | AddU32:expression -> expression -> expression
  | LessU32:expression -> expression -> expression

let rec eval (e:expression) (s:state) : Tot (option scalar) =
  match e with
  | Literal v -> Some v
  | Variable n -> s.locals n
  | AddU32 a b ->
    (match eval a s, eval b s with
     | Some (U x), Some (U y) ->
       if x < word_modulus && y < word_modulus then Some (U (add_u32 x y)) else None
     | _ -> None)
  | LessU32 a b ->
    (match eval a s, eval b s with
     | Some (U x), Some (U y) ->
       if x < word_modulus && y < word_modulus then Some (B (x < y)) else None
     | _ -> None)

let bind (s:state) (n:nat) (v:scalar) : state =
  {s with locals = (fun m -> if n = m then Some v else s.locals m)}

type event = | ReadCell:location -> scalar -> event | WriteCell:location -> scalar -> event
  | ExtensionEvent:identity -> content -> event
type trace = list event
type command = | Skip | Assign:nat -> expression -> command
  | Load:nat -> location -> scalar_type -> command
  | Store:location -> scalar_type -> expression -> command
  | Sequence:command -> command -> command
  | Select:expression -> command -> command -> command
  | While:expression -> command -> command
  | Invoke:identity -> list scalar -> command

// Extension meanings are explicit parameters, not trusted implementations.
// Admission requires a separately defined, inhabited, typed relation and its
// frame, memory-event, numerical, convergence and progress obligations.
type meaning = identity -> list scalar -> state -> trace -> state -> Type0
type authority = location -> access -> Type0

noeq type executes (ext:meaning) (can:authority) : command -> state -> trace -> state -> Type0 =
  | ExecSkip : s:state -> executes ext can Skip s [] s
  | ExecAssign : n:nat -> e:expression -> s:state -> v:scalar ->
      valid:(eval e s == Some v) -> executes ext can (Assign n e) s [] (bind s n v)
  | ExecLoad : n:nat -> p:location -> t:scalar_type -> s:state -> v:scalar ->
      owns:can p Read -> value:(s.memory p == Some v /\ well_typed v t) ->
      executes ext can (Load n p t) s [ReadCell p v] (bind s n v)
  | ExecStore : p:location -> t:scalar_type -> e:expression -> s:state -> v:scalar ->
      owns:can p Write -> value:(eval e s == Some v /\ well_typed v t) ->
      executes ext can (Store p t e) s [WriteCell p v] {s with memory=write_cell s.memory p v}
  | ExecSequence : a:command -> b:command -> s:state -> m:state -> z:state -> x:trace -> y:trace ->
      first:executes ext can a s x m -> second:executes ext can b m y z ->
      executes ext can (Sequence a b) s (FStar.List.Tot.append x y) z
  | ExecTrue : e:expression -> a:command -> b:command -> s:state -> z:state -> t:trace ->
      condition:(eval e s == Some (B true)) -> body:executes ext can a s t z ->
      executes ext can (Select e a b) s t z
  | ExecFalse : e:expression -> a:command -> b:command -> s:state -> z:state -> t:trace ->
      condition:(eval e s == Some (B false)) -> body:executes ext can b s t z ->
      executes ext can (Select e a b) s t z
  | ExecWhileDone : e:expression -> b:command -> s:state ->
      condition:(eval e s == Some (B false)) -> executes ext can (While e b) s [] s
  | ExecWhileMore : e:expression -> b:command -> s:state -> m:state -> z:state -> x:trace -> y:trace ->
      condition:(eval e s == Some (B true)) -> body:executes ext can b s x m ->
      rest:executes ext can (While e b) m y z ->
      executes ext can (While e b) s (FStar.List.Tot.append x y) z
  | ExecExtension : id:identity -> args:list scalar -> s:state -> z:state -> t:trace ->
      justified:ext id args s t z -> executes ext can (Invoke id args) s t z

// A successful relation alone excludes neither stuckness nor divergence.
// Implementations additionally owe safety on every prefix and progress under
// their declared scheduling assumptions (see Refinement.obligations).
let no_iteration (ext:meaning) (can:authority) (e:expression) (b:command)
  (s:state) (h:eval e s == Some (B false)) : executes ext can (While e b) s [] s =
  ExecWhileDone e b s h

let unsigned_add_is_typed (x y:nat) : Lemma (well_typed (U (add_u32 x y)) Unsigned32) =
  add_u32_range x y
