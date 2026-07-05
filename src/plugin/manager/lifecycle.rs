//! Plugin lifecycle management.

use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct PluginManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub dependencies: Vec<String>,
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginState {
    Loaded,
    Initialized,
    Activated,
    Deactivated,
    Unloaded,
}

pub trait Plugin: Send + Sync {
    fn manifest(&self) -> &PluginManifest;
    fn initialize(&self) -> Result<(), String> { Ok(()) }
    fn activate(&self) -> Result<(), String> { Ok(()) }
    fn deactivate(&self) -> Result<(), String> { Ok(()) }
    fn shutdown(&self) -> Result<(), String> { Ok(()) }
}

struct PluginEntry {
    plugin: Box<dyn Plugin>,
    state: PluginState,
}

pub struct PluginManager {
    plugins: HashMap<String, PluginEntry>,
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }

    pub fn load(&mut self, plugin: Box<dyn Plugin>) -> Result<(), String> {
        let id = plugin.manifest().id.clone();
        if self.plugins.contains_key(&id) {
            return Err(format!("Plugin {} already loaded", id));
        }
        self.plugins.insert(id, PluginEntry { plugin, state: PluginState::Loaded });
        Ok(())
    }

    pub fn initialize_all(&mut self) -> Result<(), String> {
        let ids: Vec<String> = self.plugins.keys().cloned().collect();
        for id in ids {
            self.transition(&id, PluginState::Initialized, |p| p.initialize())?;
        }
        Ok(())
    }

    pub fn activate_all(&mut self) -> Result<(), String> {
        let ids: Vec<String> = self.plugins.keys().cloned().collect();
        for id in ids {
            self.transition(&id, PluginState::Activated, |p| p.activate())?;
        }
        Ok(())
    }

    pub fn deactivate_all(&mut self) -> Result<(), String> {
        let ids: Vec<String> = self.plugins.keys().cloned().collect();
        for id in ids {
            self.transition(&id, PluginState::Deactivated, |p| p.deactivate())?;
        }
        Ok(())
    }

    pub fn unload_all(&mut self) -> Result<(), String> {
        let ids: Vec<String> = self.plugins.keys().cloned().collect();
        for id in ids {
            self.transition(&id, PluginState::Unloaded, |p| p.shutdown())?;
        }
        Ok(())
    }

    fn transition<F>(&mut self, id: &str, new_state: PluginState, action: F) -> Result<(), String>
    where
        F: Fn(&dyn Plugin) -> Result<(), String>,
    {
        let entry = self.plugins.get_mut(id).ok_or("Plugin not found")?;
        action(entry.plugin.as_ref())?;
        entry.state = new_state;
        Ok(())
    }

    pub fn get_state(&self, id: &str) -> Option<PluginState> {
        self.plugins.get(id).map(|e| e.state)
    }
}