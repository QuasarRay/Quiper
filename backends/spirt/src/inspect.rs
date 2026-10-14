use kuiper_contracts::{Diagnostic, Result};
use spirt::visit::{InnerVisit, Visitor};
use spirt::{
    AttrSet, Const, Context, DataInstDef, DataInstKind, DiagMsgPart, Func, GlobalVar, Import,
    Module, Type, TypeKind,
};
use std::collections::HashSet;

struct Inspection<'a> {
    module: &'a Module,
    cx: &'a Context,
    attributes: HashSet<AttrSet>,
    types: HashSet<Type>,
    constants: HashSet<Const>,
    globals: HashSet<GlobalVar>,
    functions: HashSet<Func>,
    problems: Vec<String>,
}
impl<'a> Visitor<'a> for Inspection<'a> {
    fn visit_attr_set_use(&mut self, attrs: AttrSet) {
        if !self.attributes.insert(attrs) {
            return;
        }
        let definition = &self.cx[attrs];
        for diagnostic in definition.diags() {
            let message: String = diagnostic
                .message
                .iter()
                .map(|part| match part {
                    DiagMsgPart::Plain(message) => message.as_ref(),
                    _ => "<SPIR-T value>",
                })
                .collect();
            self.problems
                .push(format!("attached diagnostic: {message}"));
        }
        definition.inner_visit_with(self);
    }
    fn visit_type_use(&mut self, ty: Type) {
        if !self.types.insert(ty) {
            return;
        }
        let definition = &self.cx[ty];
        if matches!(definition.kind, TypeKind::QPtr) {
            self.problems.push("unsupported QPtr type".into());
        }
        definition.inner_visit_with(self);
    }
    fn visit_const_use(&mut self, ct: Const) {
        if self.constants.insert(ct) {
            self.cx[ct].inner_visit_with(self);
        }
    }
    fn visit_global_var_use(&mut self, gv: GlobalVar) {
        if self.globals.insert(gv) {
            self.module.global_vars[gv].inner_visit_with(self);
        }
    }
    fn visit_func_use(&mut self, function: Func) {
        if self.functions.insert(function) {
            self.module.funcs[function].inner_visit_with(self);
        }
    }
    fn visit_import(&mut self, _: &Import) {
        self.problems.push("unresolved executable import".into());
    }
    fn visit_data_inst_def(&mut self, definition: &'a DataInstDef) {
        if matches!(definition.kind, DataInstKind::QPtr(_)) {
            self.problems.push("unsupported QPtr operation".into());
        }
        definition.inner_visit_with(self);
    }
}
pub(crate) fn clean(module: &Module) -> Result<()> {
    let mut inspector = Inspection {
        module,
        cx: module.cx_ref(),
        attributes: HashSet::new(),
        types: HashSet::new(),
        constants: HashSet::new(),
        globals: HashSet::new(),
        functions: HashSet::new(),
        problems: vec![],
    };
    inspector.visit_module(module);
    if inspector.problems.is_empty() {
        return Ok(());
    }
    Err(Diagnostic::new(
        "spirt",
        "unqualified-ir",
        inspector
            .problems
            .join("; ")
            .chars()
            .take(4096)
            .collect::<String>(),
    ))
}
