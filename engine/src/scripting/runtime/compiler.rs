//! Compiles parsed AST nodes into framework schemas.

use crate::core::node::properties::{PropertyKind, PropertyValue};
use crate::scripting::parser::syntax_tree::{AstNode, NodeDefinitionAst};

#[derive(Debug, Clone)]
pub struct CompiledPropertySchema {
    pub name: String,
    pub kind: PropertyKind,
    pub default_value: PropertyValue,
}

#[derive(Debug, Clone)]
pub struct CompiledNodeSchema {
    pub type_id: String,
    pub properties: Vec<CompiledPropertySchema>,
    pub relations: Vec<(String, String, String)>, // (field_name, preset_name, target_type)
}

pub struct DslCompiler;

impl DslCompiler {
    pub fn compile(ast_nodes: Vec<AstNode>) -> Vec<CompiledNodeSchema> {
        ast_nodes
            .into_iter()
            .filter_map(|node| match node {
                AstNode::Node(def) => Some(Self::compile_node(def)),
            })
            .collect()
    }

    fn compile_node(def: NodeDefinitionAst) -> CompiledNodeSchema {
        let properties = def.properties.into_iter().map(|p| {
            let kind = if p.is_transient {
                PropertyKind::Transient
            } else {
                PropertyKind::Persistent
            };
            
            let default_value = match (p.type_name.as_str(), p.default_value) {
                ("string", Some(val)) => PropertyValue::String(val.trim_matches('"').to_string()),
                ("boolean", Some(val)) => PropertyValue::Bool(val.parse().unwrap_or(false)),
                _ => PropertyValue::Null,
            };

            CompiledPropertySchema {
                name: p.name,
                kind,
                default_value,
            }
        }).collect();

        let relations = def.relations.into_iter().map(|r| {
            (r.field_name, r.preset_name, r.target_type)
        }).collect();

        CompiledNodeSchema {
            type_id: def.type_name,
            properties,
            relations,
        }
    }
}