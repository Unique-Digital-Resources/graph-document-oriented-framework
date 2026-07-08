//! Converts `DomEvent`s into Framework Commands and executes them.

use serde_json::Value;
use engine::core::command::pipeline::CommandPipeline;
use engine::core::node::properties::PropertyValue;
use headless_ui::nodes::ui_node::UiNodeId;
use headless_ui::nodes::widgets::WidgetKind;
use headless_ui::view_graph::storage::ViewGraph;
use super::listeners::DomEvent;

pub struct InputDispatcher;

impl InputDispatcher {
    pub fn dispatch(
        event: DomEvent,
        view: &ViewGraph,
        pipeline: &mut CommandPipeline,
    ) -> Result<(), String> {
        let target_id = match &event {
            DomEvent::Click { target } => *target,
            DomEvent::Input { target, .. } => *target,
            DomEvent::KeyDown { target, .. } => *target,
            DomEvent::Custom { target, .. } => *target,
        };

        let widget = view.get(UiNodeId(target_id))
            .ok_or("Target UI node not found in View Graph")?;

        match (widget, &event) {
            (WidgetKind::Button(btn), DomEvent::Click { .. }) => {
                if let Some(cmd_id) = &btn.command_id {
                    let mut params = serde_json::Map::new();
                    for (k, v) in &btn.command_params {
                        params.insert(k.clone(), Self::extract_json_value(v));
                    }
                    let params_val = Value::Object(params);
                        
                    pipeline.execute(cmd_id.as_str(), params_val)
                        .map_err(|e| format!("Pipeline Error: {:?}", e))?;
                }
            }
            (WidgetKind::TextField(tf), DomEvent::Input { value, .. }) => {
                if let Some((node_id, prop)) = &tf.bound_property {
                    let params = serde_json::json!({
                        "node_id": node_id.to_string(),
                        "property": prop,
                        "value": value
                    });
                    
                    pipeline.execute("SetProperty", params)
                        .map_err(|e| format!("Pipeline Error: {:?}", e))?;
                }
            }
            (_, DomEvent::Custom { command_id, params, .. }) => {
                pipeline.execute(command_id.as_str(), params.clone())
                    .map_err(|e| format!("Pipeline Error: {:?}", e))?;
            }
            _ => {
                // Unhandled event/widget combination
            }
        }

        Ok(())
    }

    /// Helper to convert the framework's internal `PropertyValue` enum 
    /// into a clean primitive `serde_json::Value` for command parameters.
    fn extract_json_value(prop: &PropertyValue) -> Value {
        match prop {
            PropertyValue::Null => Value::Null,
            PropertyValue::Bool(b) => serde_json::json!(b),
            PropertyValue::Int(i) => serde_json::json!(i),
            PropertyValue::Float(f) => serde_json::json!(f),
            PropertyValue::String(s) => serde_json::json!(s),
            PropertyValue::Uuid(u) => serde_json::json!(u.to_string()),
            PropertyValue::Date(d) => serde_json::json!(d.to_rfc3339()),
            PropertyValue::Array(arr) => serde_json::json!(arr),
            PropertyValue::Object(map) => serde_json::json!(map),
        }
    }
}