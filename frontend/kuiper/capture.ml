(* The capture protocol is private to this frontend. It never crosses the KIR
   worker boundary. Compile against one coherent compiler/Pulse build. *)
module S = FStarC_Syntax_Syntax
module P = Pulse_Syntax_Base
module J = Yojson.Safe

let obj fields = `Assoc fields
let str s = `String s
let list f xs = `List (List.map f xs)
let node tag fields = obj (("tag", str tag)::fields)
let count = ref 0
let bounded depth =
  incr count;
  if depth > 128 || !count > 16384 then failwith "Pulse capture exceeds its syntax budget"
let name fv = FStarC_Ident.string_of_lid fv.S.fv_name
let show t =
  let s = FStarC_Syntax_Print.term_to_string t in
  if String.length s > 65536 then failwith "Pulse type exceeds its capture budget";
  s
let bypass = function
  | "Prims.admit" | "Prims.assume" | "Prims._assume" | "Prims.magic"
  | "Prims.unsafe_coerce" | "FStar.Pervasives.admit"
  | "FStar.Pervasives.assume" | "FStar.Pervasives.unsafe_coerce"
  | "Pulse.Lib.Core.admit" | "Pulse.Lib.Core.assume" | "Pulse.Lib.Core.assume_"
  | "Pulse.Lib.Core.stt_admit" | "Pulse.Lib.Core.stt_atomic_admit"
  | "Pulse.Lib.Core.stt_ghost_admit" -> true
  | _ -> false
let inspect_pure t =
  (* The JSON encoder has a closed executable grammar. Inspect the complete
     pure term first, including constructs that the encoder cannot represent,
     so a ghost lambda or let cannot conceal a proof bypass. This is a local
     syntax check, not a transitive trust-closure analysis of imported symbols. *)
  let visited = ref 0 in
  ignore (FStarC_Syntax_Visit.visit_term true (fun node ->
    incr visited;
    if !visited > 16384 then failwith "Pure term exceeds its inspection budget";
    (match node.S.n with
      | S.Tm_fvar fv when bypass (name fv) -> failwith "Explicit proof bypass in checked source"
      | _ -> ());
    node) t)
let rec pure depth (t:S.term) =
  bounded depth;
  if depth = 0 then inspect_pure t;
  let t = FStarC_Syntax_Subst.compress t in
  match t.S.n with
  | S.Tm_bvar v -> node "bound" ["index", `Intlit (Z.to_string v.S.index)]
  | S.Tm_name _ -> node "unsupported" ["constructor",str "free local"]
  | S.Tm_fvar fv -> node "symbol" ["name",str (name fv)]
  | S.Tm_uinst (t,_) -> pure (depth+1) t
  | S.Tm_ascribed x -> pure (depth+1) x.S.tm
  | S.Tm_meta x -> pure (depth+1) x.S.tm
  | S.Tm_app _ ->
      let head,args = FStarC_Syntax_Util.head_and_args_full t in
      node "apply" ["head",pure (depth+1) head;
        "arguments",list (fun (arg,qual) -> obj [
          "implicit",`Bool (match qual with None -> false | Some x -> x.S.aqual_implicit);
          "value",pure (depth+1) arg]) args]
  | S.Tm_constant FStarC_Const.Const_unit -> node "unit" []
  | S.Tm_constant (FStarC_Const.Const_bool b) -> node "bool" ["value",`Bool b]
  | S.Tm_constant (FStarC_Const.Const_int (i,_)) -> node "integer" ["value",`Intlit (Z.to_string i)]
  | _ -> node "unsupported" ["constructor",str (S.fStarC_Class_Tagged_tag_of__syntax_term' t)]
let effect_tag = function
  | None -> str "unspecified"
  | Some P.STT -> str "stateful"
  | Some P.STT_Div -> str "divergent"
  | Some P.STT_Atomic -> str "atomic"
  | Some P.STT_Ghost -> str "ghost"
let checked_pure depth t = inspect_pure t; pure depth t
let binder depth b =
  List.iter inspect_pure b.P.binder_attrs;
  obj ["type",checked_pure (depth+1) b.P.binder_ty; "type_display",str (show b.P.binder_ty)]
let inspect_comp = function
  | P.C_Tot ty -> inspect_pure ty
  | P.C_ST c | P.C_STDiv c -> List.iter inspect_pure [c.P.res; c.P.pre; c.P.post]
  | P.C_STAtomic (inv,_,c) | P.C_STGhost (inv,c) ->
      List.iter inspect_pure [inv; c.P.res; c.P.pre; c.P.post]
let comp = function
  | P.C_Tot _ -> "total"
  | P.C_ST _ -> "stateful"
  | P.C_STDiv _ -> "divergent"
  | P.C_STAtomic _ -> "atomic"
  | P.C_STGhost _ -> "ghost"
let rec stateful depth (t:P.st_term) =
  bounded depth;
  let base = ["effect",effect_tag t.P.effect_tag; "source",`Bool t.P.source;
              "range",str (FStarC_Range.string_of_range t.P.range)] in
  let st = stateful (depth+1) in
  let pt = checked_pure (depth+1) in
  let tag,fields = match t.P.term with
  | P.Tm_Abs x ->
      (match x.P.q with Some (P.Meta t) -> inspect_pure t | _ -> ());
      Option.iter inspect_comp x.P.ascription.P.annotated;
      Option.iter inspect_comp x.P.ascription.P.elaborated;
      "abstract",[
      "binder",binder depth x.P.b;
      "implicit",`Bool (x.P.q <> None);
      "computation",(match x.P.ascription.P.elaborated with None -> `Null | Some c -> str (comp c));
      "body",st x.P.body]
  | P.Tm_Return x -> "return",["type",pt x.P.expected_type; "value",pt x.P.term]
  | P.Tm_ST x -> "stateful_apply",["function",pt x.P.t; "arguments",list st x.P.args]
  | P.Tm_Bind x -> "bind",["binder",binder depth x.P.binder; "head",st x.P.head; "body",st x.P.body]
  | P.Tm_TotBind x -> "pure_bind",["binder",binder depth x.P.binder; "head",pt x.P.head; "body",st x.P.body]
  | P.Tm_If x ->
      Option.iter inspect_pure x.P.pre;
      Option.iter inspect_pure x.P.post;
      "if",["condition",st x.P.b; "then",st x.P.then__; "else",st x.P.else__]
  | P.Tm_IntroPure p -> "introduce_pure",["proposition",pt p]
  | P.Tm_IntroExists x -> "introduce_exists",["proposition",pt x.P.p;
      "witnesses",list pt x.P.witnesses]
  | P.Tm_ElimExists p -> "eliminate_exists",["proposition",pt p]
  | P.Tm_Rewrite x -> "rewrite",["left",pt x.P.t1; "right",pt x.P.t2;
      "tactic",(match x.P.tac_opt with None -> `Null | Some t -> pt t)]
  | P.Tm_Admit _ -> "admit",[]
  | P.Tm_Unreachable _ -> "unreachable",[]
  | P.Tm_While x ->
      List.iter inspect_pure (x.P.invariant::x.P.loop_requires::x.P.meas);
      "unsupported",["constructor",str "while";
      "condition",st x.P.condition; "body",st x.P.body]
  | P.Tm_WithLocal x ->
      ignore (binder depth x.P.binder);
      Option.iter inspect_pure x.P.initializer_;
      "unsupported",["constructor",str "local reference"; "body",st x.P.body]
  | P.Tm_WithLocalArray x ->
      ignore (binder depth x.P.binder);
      Option.iter inspect_pure x.P.initializer_;
      inspect_pure x.P.length;
      "unsupported",["constructor",str "local array"; "body",st x.P.body]
  | P.Tm_Match x ->
      Option.iter inspect_pure x.P.returns_;
      "unsupported",["constructor",str "match";
      "scrutinee",st x.P.sc; "branches",list (fun b -> st b.P.e) x.P.brs]
  | P.Tm_ProofHintWithBinders _ -> failwith "Residual proof hints require supported elaboration"
  | P.Tm_PragmaWithOptions x -> "pragma",["options",str x.P.options; "body",st x.P.body]
  | P.Tm_Defer x ->
      inspect_pure x.P.handler_pre;
      "unsupported",["constructor",str "defer";
      "handler",st x.P.handler; "body",st x.P.body]
  | P.Tm_ForwardJumpLabel x ->
      inspect_comp x.P.post;
      "label",["body",st x.P.body;
      "result_type",(match x.P.post with
        | P.C_ST c | P.C_STDiv c | P.C_STAtomic (_,_,c) | P.C_STGhost (_,c) -> pt c.P.res
        | P.C_Tot ty -> pt ty)]
  | P.Tm_Goto x -> "jump",["label",pt x.P.lbl; "argument",pt x.P.arg]
  in
  node tag (base @ fields)
let getenv key = match Sys.getenv_opt key with Some v -> v | None -> failwith ("missing " ^ key)
let module_name = getenv "KUIPER_CAPTURE_MODULE"
let target = getenv "KUIPER_CAPTURE_DIRECTORY"
let observe names typ term =
  match List.rev names with
  | fn::rev_mod when String.concat "." (List.rev rev_mod) = module_name ->
      if not (FStarC_Options.should_verify module_name) || FStarC_Options.admit_smt_queries ()
      then failwith "Pulse capture requires strict source checking";
      if String.length fn > 128 || not (String.for_all (function
          | 'a'..'z' | 'A'..'Z' | '0'..'9' | '_' -> true | _ -> false) fn)
      then failwith "unsupported capture symbol name";
      count := 0;
      inspect_pure typ;
      let body:P.st_term = Pulse_RuntimeUtils.unembed_st_term_for_extraction term in
      let record = obj ["schema",str "kuiper.pulse-capture/1";
        "name",str (String.concat "." names); "type_display",str (show typ);
        "body",stateful 0 body] in
      let bytes = J.to_string record in
      if String.length bytes > 1048576 then failwith "Pulse capture exceeds its byte budget";
      let path = Filename.concat target (fn ^ ".json") in
      let channel = open_out_gen [Open_wronly;Open_creat;Open_excl;Open_binary] 0o600 path in
      Fun.protect ~finally:(fun () -> close_out channel) (fun () -> output_string channel bytes)
  | _ -> ()
let () = Pulse_RuntimeUtils.register_checked_term_observer observe
