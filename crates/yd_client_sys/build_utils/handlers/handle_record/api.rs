use clang::*;

use crate::build_utils::{
    config::HandlerConfigs, handle_function_prototype::MethodFlavor, process_children, HandlerMap,
};

pub fn handle_api_record(
    entity: &Entity,
    handlers: &HandlerMap,
    configs: &mut HandlerConfigs,
    full_rust_struct_name: &str,
) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let vtable_struct_name = format!("{full_rust_struct_name}VTable");
    let full_trait_name = format!("{full_rust_struct_name}Trait");

    lines.push(format!("\n/* Generated by handle_api_trait */"));
    lines.extend(handle_api_trait(
        entity,
        handlers,
        configs,
        &full_rust_struct_name,
        &full_trait_name,
    ));

    lines
}

pub fn handle_api_trait(
    entity: &Entity,
    handlers: &HandlerMap,
    configs: &HandlerConfigs,
    full_rust_struct_name: &str,
    full_trait_name: &str,
) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(r#"
pub struct {full_trait_name} {{}}
impl {full_trait_name} {{
"#));
    lines.extend(process_children(
        entity,
        handlers,
        &mut HandlerConfigs {
            method_flavor: MethodFlavor::ApiTrait,
            ..configs.clone()
        },
    ));
    lines.push(format!(
        r#"}}
unsafe impl Send for {full_trait_name} {{}}
"#
    ));
    lines
}