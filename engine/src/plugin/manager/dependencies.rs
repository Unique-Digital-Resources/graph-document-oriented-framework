//! Resolving plugin dependency graphs.

use std::collections::{HashMap, HashSet};

use super::lifecycle::PluginManifest;

/// Resolves the correct initialization order for plugins based on their dependencies.
/// Returns an error if a dependency is missing or a cycle is detected.
pub fn resolve_plugin_order(manifests: &[PluginManifest]) -> Result<Vec<String>, String> {
    let mut graph: HashMap<&String, Vec<&String>> = HashMap::new();
    for m in manifests {
        graph.insert(&m.id, m.dependencies.iter().collect());
    }

    let mut visited = HashSet::new();
    let mut visiting = HashSet::new(); // For cycle detection
    let mut order = Vec::new();

    fn visit(
        id: &String,
        graph: &HashMap<&String, Vec<&String>>,
        visited: &mut HashSet<String>,
        visiting: &mut HashSet<String>,
        order: &mut Vec<String>,
    ) -> Result<(), String> {
        if visited.contains(id) {
            return Ok(());
        }
        if visiting.contains(id) {
            return Err(format!("Cyclic dependency detected involving plugin '{}'", id));
        }
        
        visiting.insert(id.clone());

        if let Some(deps) = graph.get(id) {
            for dep in deps {
                if !graph.contains_key(dep) {
                    return Err(format!("Plugin '{}' requires missing dependency '{}'", id, dep));
                }
                visit(dep, graph, visited, visiting, order)?;
            }
        }

        visiting.remove(id);
        visited.insert(id.clone());
        order.push(id.clone());
        Ok(())
    }

    for m in manifests {
        visit(&m.id, &graph, &mut visited, &mut visiting, &mut order)?;
    }

    Ok(order)
}