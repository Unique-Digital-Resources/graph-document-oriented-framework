//! Filesystem, Network capability checks.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Permission {
    Filesystem,
    Network,
    Clipboard,
    Database,
    Secrets,
    Commands,
}

impl Permission {
    /// Parses a string from the manifest into a typed Permission.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "filesystem" => Some(Self::Filesystem),
            "network" => Some(Self::Network),
            "clipboard" => Some(Self::Clipboard),
            "database" => Some(Self::Database),
            "secrets" => Some(Self::Secrets),
            "commands" => Some(Self::Commands),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PermissionDenied(pub String);