//! Property bag for a `Node`.
//!
//! From the PRD, properties fall into five kinds:
//!
//! | Kind        | Saved to disk? | Source                         |
//! |-------------|----------------|--------------------------------|
//! | `Persistent`| yes            | user / commands                |
//! | `Transient` | no             | runtime (UI selection, etc.)   |
//! | `Computed`  | no             | pure function of other props   |
//! | `Derived`   | no             | function of graph relations    |
//! | `Cached`    | no             | memoized result of heavy work  |
//!
//! `PropertyValue` is a tagged union covering the scalar/collection
//! types a developer can declare in the DSL
//! (`title: string`, `count: int`, `tags: string[]`, …). Custom plugin
//! types can round-trip through `PropertyValue::Object`.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Why a property exists and whether it survives save/load.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PropertyKind {
    /// User-authored, persisted to the document. The default kind.
    Persistent,
    /// Runtime-only (e.g. `UISelectedState`). Never serialized.
    Transient,
    /// Pure function of *other properties on the same node*.
    /// Recomputed on dirty propagation (Phase 3).
    Computed,
    /// Function of *graph relations* (e.g. "count of children").
    /// Recomputed by the scheduler when the graph mutates.
    Derived,
    /// Memoized result of an expensive computation (e.g. thumbnail hash).
    /// Invalidated by the scheduler; may be evicted under memory pressure.
    Cached,
}



impl PropertyKind {
    /// `true` if the property must round-trip through save/load.
    #[inline]
    pub fn is_persisted(self) -> bool {
        matches!(self, Self::Persistent)
    }

    /// `true` if the property is automatically produced (not user-set).
    #[inline]
    pub fn is_automatic(self) -> bool {
        matches!(self, Self::Computed | Self::Derived | Self::Cached)
    }

    /// `true` if the property is runtime-only and never saved.
    #[inline]
    pub fn is_transient(self) -> bool {
        matches!(self, Self::Transient)
    }
}



//impl PropertyKind {
    /// `true` if the property must round-trip through save/load.
//    #[inline]
//    pub fn is_persisted(self) -> bool {
//        matches!(self, Self::Persistent)
//    }

    /// `true` if the property is automatically produced (not user-set).
//    #[inline]
//    pub fn is_automatic(self) -> bool {
//        matches!(self, Self::Computed | Self::Derived | Self::Cached)
//    }
//}

/// Polymorphic value type. Covers all primitives the DSL can spell,
/// plus `Array` / `Object` for nested data. Plug-ins can stash
/// arbitrary JSON-shaped data in `Object`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "t", content = "v")]
pub enum PropertyValue {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Uuid(uuid::Uuid),
    Date(chrono::DateTime<chrono::Utc>),
    Array(Vec<PropertyValue>),
    Object(HashMap<String, PropertyValue>),
}

impl PropertyValue {
    /// Human-readable type name, used by the DSL parser for error
    /// messages and by the AI tool layer when introspecting a node.
    pub fn type_name(&self) -> &'static str {
        match self {
            Self::Null => "null",
            Self::Bool(_) => "boolean",
            Self::Int(_) => "int",
            Self::Float(_) => "float",
            Self::String(_) => "string",
            Self::Uuid(_) => "uuid",
            Self::Date(_) => "date",
            Self::Array(_) => "array",
            Self::Object(_) => "object",
        }
    }

    // ---- convenience constructors for the most common DSL literals ----

    #[inline]
    pub fn string(s: impl Into<String>) -> Self {
        Self::String(s.into())
    }

    #[inline]
    pub fn bool(b: bool) -> Self {
        Self::Bool(b)
    }

    #[inline]
    pub fn int(i: i64) -> Self {
        Self::Int(i)
    }

    #[inline]
    pub fn float(f: f64) -> Self {
        Self::Float(f)
    }

    #[inline]
    pub fn null() -> Self {
        Self::Null
    }
}

impl Default for PropertyValue {
    fn default() -> Self {
        Self::Null
    }
}

impl From<&str> for PropertyValue {
    fn from(s: &str) -> Self {
        Self::String(s.to_string())
    }
}

impl From<String> for PropertyValue {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl From<bool> for PropertyValue {
    fn from(b: bool) -> Self {
        Self::Bool(b)
    }
}

impl From<i64> for PropertyValue {
    fn from(i: i64) -> Self {
        Self::Int(i)
    }
}

impl From<f64> for PropertyValue {
    fn from(f: f64) -> Self {
        Self::Float(f)
    }
}

/// A single property: its value + its kind.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    pub value: PropertyValue,
    pub kind: PropertyKind,
}

impl Property {
    pub fn new(value: PropertyValue, kind: PropertyKind) -> Self {
        Self { value, kind }
    }
}

/// Insertion-ordered not required — `HashMap` is fine. The DSL parser
/// (Phase 4) will surface a stable alphabetical order to humans.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Properties {
    entries: HashMap<String, Property>,
}

impl Properties {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set a property with an explicit kind.
    pub fn set(
        &mut self,
        name: impl Into<String>,
        value: PropertyValue,
        kind: PropertyKind,
    ) {
        self.entries.insert(name.into(), Property { value, kind });
    }

    /// Shortcut for `PropertyKind::Persistent`. Most DSL declarations
    /// (`title: string`, `isCompleted: boolean = false`) end up here.
    pub fn set_persistent(&mut self, name: impl Into<String>, value: PropertyValue) {
        self.set(name, value, PropertyKind::Persistent);
    }

    /// Shortcut for `PropertyKind::Transient`.
    /// Maps to the DSL `transient: UISelectedState` form.
    pub fn set_transient(&mut self, name: impl Into<String>, value: PropertyValue) {
        self.set(name, value, PropertyKind::Transient);
    }

    /// Shortcut for `PropertyKind::Computed`.
    pub fn set_computed(&mut self, name: impl Into<String>, value: PropertyValue) {
        self.set(name, value, PropertyKind::Computed);
    }

    /// Shortcut for `PropertyKind::Derived`.
    pub fn set_derived(&mut self, name: impl Into<String>, value: PropertyValue) {
        self.set(name, value, PropertyKind::Derived);
    }

    /// Shortcut for `PropertyKind::Cached`.
    pub fn set_cached(&mut self, name: impl Into<String>, value: PropertyValue) {
        self.set(name, value, PropertyKind::Cached);
    }

    pub fn get(&self, name: &str) -> Option<&Property> {
        self.entries.get(name)
    }

    pub fn get_value(&self, name: &str) -> Option<&PropertyValue> {
        self.entries.get(name).map(|p| &p.value)
    }

    /// Mutable value access. The Command pipeline uses this to apply
    /// property edits; the surrounding transaction captures the old
    /// value for undo.
    pub fn get_value_mut(&mut self, name: &str) -> Option<&mut PropertyValue> {
        self.entries.get_mut(name).map(|p| &mut p.value)
    }

    pub fn remove(&mut self, name: &str) -> Option<Property> {
        self.entries.remove(name)
    }

    pub fn contains(&self, name: &str) -> bool {
        self.entries.contains_key(name)
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &Property)> {
        self.entries.iter()
    }

    /// Only properties that should be written to disk. Used by the
    /// Phase 5 serializer.
    pub fn iter_persisted(&self) -> impl Iterator<Item = (&String, &Property)> {
        self.entries.iter().filter(|(_, p)| p.kind.is_persisted())
    }

    /// Only runtime-only properties. Useful for debug dumps.
    pub fn iter_transient(&self) -> impl Iterator<Item = (&String, &Property)> {
        self.entries
            .iter()
            .filter(|(_, p)| !p.kind.is_persisted())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kinds_partition_correctly() {
        assert!(PropertyKind::Persistent.is_persisted());
        assert!(!PropertyKind::Persistent.is_automatic());

        assert!(!PropertyKind::Transient.is_persisted());
        assert!(!PropertyKind::Transient.is_automatic());

        for k in [PropertyKind::Computed, PropertyKind::Derived, PropertyKind::Cached] {
            assert!(!k.is_persisted());
            assert!(k.is_automatic());
        }
    }

    #[test]
    fn set_and_get_round_trip() {
        let mut p = Properties::new();
        p.set_persistent("title", PropertyValue::string("Hello"));
        p.set_transient("uiSelected", PropertyValue::bool(true));
        p.set_computed("label", PropertyValue::string("Hello [auto]"));

        assert_eq!(p.get_value("title"), Some(&PropertyValue::string("Hello")));
        assert_eq!(p.get("uiSelected").unwrap().kind, PropertyKind::Transient);
        assert_eq!(p.get("label").unwrap().kind, PropertyKind::Computed);
        assert_eq!(p.len(), 3);
    }

    #[test]
    fn iter_persisted_excludes_transient_and_automatic() {
        let mut p = Properties::new();
        p.set_persistent("title", PropertyValue::string("x"));
        p.set_transient("uiSelected", PropertyValue::bool(true));
        p.set_computed("c", PropertyValue::int(1));
        p.set_cached("k", PropertyValue::int(2));

        let persisted: Vec<&String> = p.iter_persisted().map(|(k, _)| k).collect();
        assert_eq!(persisted, vec![&"title".to_string()]);
    }

    #[test]
    fn value_conversions() {
        let v: PropertyValue = "hi".into();
        assert_eq!(v, PropertyValue::String("hi".into()));
        let v: PropertyValue = true.into();
        assert_eq!(v, PropertyValue::Bool(true));
        let v: PropertyValue = 42i64.into();
        assert_eq!(v, PropertyValue::Int(42));
    }

    #[test]
    fn serde_round_trip() {
        let mut p = Properties::new();
        p.set_persistent("title", PropertyValue::string("Hello"));
        p.set_transient("flag", PropertyValue::bool(true));

        let json = serde_json::to_string(&p).unwrap();
        let back: Properties = serde_json::from_str(&json).unwrap();
        assert_eq!(back.get_value("title"), p.get_value("title"));
        assert_eq!(back.get("flag").unwrap().kind, PropertyKind::Transient);
    }

    #[test]
    fn get_value_mut_supports_in_place_edit() {
        let mut p = Properties::new();
        p.set_persistent("count", PropertyValue::int(0));
        if let Some(v) = p.get_value_mut("count") {
            if let PropertyValue::Int(i) = v {
                *i += 1;
            }
        }
        assert_eq!(p.get_value("count"), Some(&PropertyValue::int(1)));
    }
}


// Add this to the bottom of src/core/node/properties.rs

impl std::fmt::Display for PropertyValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Null => write!(f, "null"),
            Self::Bool(b) => write!(f, "{}", b),
            Self::Int(i) => write!(f, "{}", i),
            Self::Float(fl) => write!(f, "{}", fl),
            Self::String(s) => write!(f, "{}", s),
            Self::Uuid(u) => write!(f, "{}", u),
            Self::Date(d) => write!(f, "{}", d.to_rfc3339()),
            Self::Array(arr) => {
                write!(f, "[")?;
                for (i, v) in arr.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
            Self::Object(map) => {
                write!(f, "{{")?;
                for (i, (k, v)) in map.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}: {}", k, v)?;
                }
                write!(f, "}}")
            }
        }
    }
}