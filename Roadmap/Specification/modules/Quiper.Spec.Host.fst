module Quiper.Spec.Host
open Quiper.Spec.Foundation

noeq type allocation = { identity:resource; extent:nat; live:bool; pending:nat; contents:store }
type heap = resource -> option allocation
noeq type host_state = { allocations:heap; next_generation:nat }
let replace (h:heap) (r:resource) (a:allocation) : heap = fun q -> if q = r then Some a else h q
let valid_view (h:heap) (v:view) : prop =
  exists a. h v.backing == Some a /\ a.identity = v.backing /\ a.live /\ v.start + v.count <= a.extent
let can_free (a:allocation) : bool = a.live && a.pending = 0
let borrow (a:allocation) : allocation = {a with pending=a.pending+1}
let finish (a:allocation{a.pending > 0}) : allocation = {a with pending=a.pending-1}
let pending_prevents_free (a:allocation)
  : Lemma (ensures not (can_free (borrow a))) = ()
let finish_preserves_live (a:allocation{a.pending > 0})
  : Lemma ((finish a).live = a.live) = ()

noeq type host_step (quiescent:resource -> prop) : host_state -> host_state -> Type0 =
  | Allocate : before:host_state -> id:resource -> n:nat -> initial:store ->
      fresh:(before.allocations id == None /\ id.generation = before.next_generation) ->
      host_step quiescent before {
        allocations=replace before.allocations id {identity=id; extent=n; live=true; pending=0; contents=initial};
        next_generation=before.next_generation+1 }
  | Reserve : before:host_state -> id:resource -> a:allocation ->
      available:(before.allocations id == Some a /\ a.live /\ a.identity = id) ->
      host_step quiescent before {before with allocations=replace before.allocations id (borrow a)}
  | Resolve : before:host_state -> id:resource -> a:allocation{a.pending > 0} ->
      found:(before.allocations id == Some a /\ a.identity = id) ->
      quiet:quiescent id ->
      // O8 must justify the actual observation, bound to this resource generation.
      host_step quiescent before {before with allocations=replace before.allocations id (finish a)}
  | Free : before:host_state -> id:resource -> a:allocation ->
      available:(before.allocations id == Some a /\ a.identity = id /\ can_free a) ->
      host_step quiescent before {before with allocations=replace before.allocations id {a with live=false}}

// Copy is a relation over a snapshot, permitting overlap only when the chosen
// operation contract specifies snapshot/memmove semantics. Other copies must
// prove disjointness before invoking their relation.
let copy_relation (source destination:view) (before after:store) : prop =
  source.count = destination.count /\ source.cell_bytes = destination.cell_bytes /\
  (forall (i:nat). i < source.count ==>
    after {object=destination.backing; cell=destination.start+i} ==
    before {object=source.backing; cell=source.start+i}) /\
  (forall p. not (contains destination p) ==> after p == before p)

// A scope may return only after its registry has no pending borrowed uses.
// Forgetting user-visible handles does not remove registrations.
let scope_may_return (registered:nat -> bool) (pending:nat -> nat) : prop =
  forall id. registered id ==> pending id = 0
let pending_blocks_scope_exit (registered:nat -> bool) (pending:nat -> nat) (id:nat)
  : Lemma (requires registered id /\ pending id > 0) (ensures not (scope_may_return registered pending)) = ()
