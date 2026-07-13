use headless_ui::nodes::ui_node::{UiNode, UiNodeRole};
use headless_ui::nodes::widgets::CustomWidget;

pub fn create() -> CustomWidget {
    CustomWidget {
        base: UiNode::new("WheelModeWidget", UiNodeRole::Custom),
        kind: "wheel-mode".to_string(),
        data: serde_json::json!({
            "mode": "Ranges" // FIX: Default to Ranges to avoid initial layout timing issues
        }),
        event_listeners: vec!["click".to_string()],
    }
}