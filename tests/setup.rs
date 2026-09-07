use swc_core::{
    common::Mark,
    ecma::{
        ast::Pass,
        parser::{EsSyntax, Syntax},
        transforms::base::resolver,
        visit::{visit_mut_pass, VisitMut},
    },
};
use swc_plugin_minify_catch_param::CatchParamMinifier;

pub fn syntax() -> Syntax {
    Syntax::Es(EsSyntax {
        jsx: false,
        ..Default::default()
    })
}

pub fn visitor() -> impl VisitMut + Pass {
    (
        resolver(Mark::new(), Mark::new(), false),
        visit_mut_pass(CatchParamMinifier),
    )
}
