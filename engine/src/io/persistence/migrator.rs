//! Handling schema version changes.

use crate::core::graph::Graph;

pub struct Migrator;

impl Migrator {
    /// Runs necessary migrations based on the loaded version.
    pub fn migrate(graph: &mut Graph, from_version: u32, to_version: u32) -> Result<(), String> {
        let mut current = from_version;
        while current < to_version {
            match current {
                1 => Self::migrate_v1_to_v2(graph)?,
                _ => break,
            }
            current += 1;
        }
        Ok(())
    }

    /// Example migration: Add a default "archived" property to all nodes.
    fn migrate_v1_to_v2(graph: &mut Graph) -> Result<(), String> {
        for node in graph.iter_nodes_mut() {
            if !node.properties.contains("archived") {
                node.properties.set_persistent("archived", crate::core::node::properties::PropertyValue::Bool(false));
            }
        }
        Ok(())
    }
}