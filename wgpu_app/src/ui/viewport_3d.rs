use engine::core::node::node::NodeSchema;
use engine::core::node::properties::PropertyKind;

/// Returns the schema for the Viewport3DWidget.
/// The framework uses this for auto-registration and serialization validation.
pub fn get_viewport_3d_schema() -> NodeSchema {
    let mut schema = NodeSchema::new("Viewport3DWidget");
    
    // Declares that this node is entirely runtime-only and won't be saved to disk
    schema.set_kind("tag", PropertyKind::Transient);
    schema.set_kind("data", PropertyKind::Transient);
    
    schema
}