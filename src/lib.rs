use swc_core::{
    ecma::{ast::Program, visit::visit_mut_pass},
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};

pub mod minifier;
use crate::minifier::CatchParamMinifier;

#[plugin_transform]
pub fn process_transform(program: Program, _: TransformPluginProgramMetadata) -> Program {
    program.apply(visit_mut_pass(CatchParamMinifier))
}
