//! Enforcing permissions on plugin actions.

use std::collections::HashMap;

use super::policy::{Permission, PermissionDenied};

#[derive(Default)]
pub struct Sandbox {
    permissions: HashMap<String, Vec<Permission>>,
}

impl Sandbox {
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a plugin's permissions from its manifest.
    pub fn register_plugin(&mut self, plugin_id: &str, perms: &[String]) {
        let parsed: Vec<Permission> = perms
            .iter()
            .filter_map(|p| Permission::from_str(p))
            .collect();
        self.permissions.insert(plugin_id.to_string(), parsed);
    }

    /// Checks if a plugin has the required permission.
    pub fn check(&self, plugin_id: &str, required: Permission) -> Result<(), PermissionDenied> {
        let perms = self.permissions.get(plugin_id).cloned().unwrap_or_default();
        
        if perms.contains(&required) {
            Ok(())
        } else {
            Err(PermissionDenied(format!(
                "Plugin '{}' lacks required permission: {:?}",
                plugin_id, required
            )))
        }
    }
}