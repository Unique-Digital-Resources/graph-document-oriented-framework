//! Maps the Headless View Graph to a Virtual DOM tree.
//!
//! Instead of directly manipulating browser DOM elements (which requires
//! `web-sys` / `wasm-bindgen`), this module generates a serializable
//! `DomNode` tree. A lightweight JavaScript script (see `js_bridge.rs`)
//! receives this JSON tree and applies it to the real browser DOM.

use serde::{Serialize, Deserialize};
use headless_ui::nodes::ui_node::UiNodeId;
use headless_ui::nodes::widgets::WidgetKind;
use headless_ui::view_graph::storage::ViewGraph;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomNode {
    pub id: String,
    pub tag: String,
    pub text: Option<String>,
    pub bounds: (f32, f32, f32, f32),
    pub children: Vec<DomNode>,
    pub event_listeners: Vec<String>,
    /// Custom payload for app-specific widgets. Null for standard widgets.
    pub props: serde_json::Value,
}

pub struct DomMapper;

impl DomMapper {
    /// Generates the root `DomNode` from the `ViewGraph`.
    pub fn map(view: &ViewGraph) -> Option<DomNode> {
        view.root().and_then(|root| Self::map_node(view, root))
    }

    fn map_node(view: &ViewGraph, id: UiNodeId) -> Option<DomNode> {
        let widget = view.get(id)?;
        let ui_node = widget.ui_node();
        
        let (tag, text, listeners, props) = match widget {
            WidgetKind::Container(_) => ("div", None, vec![], serde_json::Value::Null),
            WidgetKind::Label(l) => ("span", Some(l.text.clone()), vec![], serde_json::Value::Null),
            WidgetKind::Button(b) => ("button", Some(b.label.clone()), vec!["click".to_string()], serde_json::Value::Null),
            WidgetKind::TextField(tf) => ("input", Some(tf.value.clone()), vec!["input".to_string(), "keydown".to_string()], serde_json::Value::Null),
            WidgetKind::ListView(_) => ("div", None, vec![], serde_json::Value::Null),
            WidgetKind::Inspector(_) => ("div", None, vec![], serde_json::Value::Null),
            WidgetKind::Custom(c) => (
                c.kind.as_str(), 
                None, 
                c.event_listeners.clone(), 
                c.data.clone()
            ),
        };

        let children = view.children(id).iter()
            .filter_map(|c| Self::map_node(view, *c))
            .collect();

        Some(DomNode {
            id: id.0.to_string(),
            tag: tag.to_string(),
            text,
            bounds: (ui_node.bounds.x, ui_node.bounds.y, ui_node.bounds.width, ui_node.bounds.height),
            children,
            event_listeners: listeners,
            props,
        })
    }
}