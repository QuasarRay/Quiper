module Quiper.Spec.Foundation

// Logical identities and quantities. No machine address or implementation ABI.
type identity = { name:string; definition:string }
type resource = { allocation:nat; generation:nat }
type access = | Read | Write | Atomic
type view = { backing:resource; start:nat; count:nat; cell_bytes:pos }
type location = { object:resource; cell:nat }
type scalar_type = | Boolean | Unsigned32 | Signed32 | Float32
type scalar = | B:bool -> scalar | U:nat -> scalar | I:int -> scalar | F:nat -> scalar

let word_modulus : pos = 4294967296
let well_typed (v:scalar) (t:scalar_type) : bool =
  match v,t with
  | B _, Boolean -> true
  | U n, Unsigned32 -> n < word_modulus
  | I n, Signed32 -> -2147483648 <= n && n < 2147483648
  | F bits, Float32 -> bits < word_modulus
  | _ -> false

let contains (v:view) (p:location) : bool =
  p.object = v.backing && v.start <= p.cell && p.cell < v.start + v.count

let disjoint (a b:view) : bool =
  a.count = 0 || b.count = 0 || a.backing <> b.backing ||
  a.start + a.count <= b.start || b.start + b.count <= a.start

let subview (child parent:view) : bool =
  child.backing = parent.backing && child.cell_bytes = parent.cell_bytes &&
  parent.start <= child.start && child.start + child.count <= parent.start + parent.count

let subview_preserves_bounds (c p:view) (x:location)
  : Lemma (requires subview c p /\ contains c x) (ensures contains p x) = ()

let disjoint_excludes_shared_cell (a b:view) (x:location)
  : Lemma (requires disjoint a b /\ contains a x) (ensures not (contains b x)) = ()

// Permissions describe an invocation's entitlement. Refinements must also
// establish global compatibility of simultaneous entitlements.
type permission = { range:view; mode:access }
let compatible (a b:permission) : bool =
  disjoint a.range b.range || (a.mode = Read && b.mode = Read) ||
  (a.mode = Atomic && b.mode = Atomic)

type store = location -> option scalar
let write_cell (h:store) (p:location) (v:scalar) : store =
  fun q -> if q = p then Some v else h q

let write_frame (h:store) (p q:location) (v:scalar)
  : Lemma (requires q <> p) (ensures write_cell h p v q == h q) = ()

let add_u32 (x y:nat) : nat = (x + y) % word_modulus
let add_u32_range (x y:nat) : Lemma (add_u32 x y < word_modulus) = ()

// Exact byte sequences, rather than hashes, are the mathematical identities.
// A wire digest refines this equality under its explicit collision assumption.
type byte = n:nat{n < 256}
type content = list byte
