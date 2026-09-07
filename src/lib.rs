use swc_core::{
    ecma::{
        ast::{CatchClause, Id, Ident, Pat, Program},
        visit::{visit_mut_pass, Visit, VisitMut, VisitWith},
    },
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};

struct CatchParamUsageVisitor {
    pub target_id: Id,
    pub found: bool,
}

impl Visit for CatchParamUsageVisitor {
    fn visit_ident(&mut self, node: &Ident) {
        if self.found {
            return;
        }

        if node.to_id() == self.target_id {
            self.found = true;
        }
    }
}

pub struct CatchParamMinifier;

impl VisitMut for CatchParamMinifier {
    fn visit_mut_catch_clause(&mut self, catch_clause: &mut CatchClause) {
        let Some(Pat::Ident(binding_ident)) = &catch_clause.param else {
            return;
        };

        let mut usage_visitor = CatchParamUsageVisitor {
            target_id: binding_ident.id.to_id(),
            found: false,
        };

        catch_clause.body.visit_with(&mut usage_visitor);

        if !usage_visitor.found {
            catch_clause.param = None;
        }
    }
}

#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.apply(visit_mut_pass(CatchParamMinifier))
}
