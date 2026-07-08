//! Defines the DOM events coming from the Web Bridge.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DomEvent {
    Click { target: Uuid },
    Input { target: Uuid, value: String },
    KeyDown { target: Uuid, key: String },
    /// A custom event dispatched by an app-specific Web Component.
    /// Allows the frontend to specify exactly which Command to run 
    /// and with what parameters.
    Custom { 
        target: Uuid, 
        command_id: String, 
        params: serde_json::Value 
    },
}

pub struct EventListener;

impl EventListener {
    /// Parses an incoming JSON payload from the WebSocket into a `DomEvent`.
    pub fn parse(payload: &str) -> Result<DomEvent, serde_json::Error> {
        let v: serde_json::Value = serde_json::from_str(payload)?;
        
        let target = Uuid::parse_str(v["target"].as_str().unwrap_or("")).unwrap_or_else(|_| Uuid::nil());
        let event_type = v["type"].as_str().unwrap_or("");
        
        let event = match event_type {
            "click" => DomEvent::Click { target },
            "input" => DomEvent::Input { 
                target, 
                value: v["value"].as_str().unwrap_or("").to_string() 
            },
            "keydown" => DomEvent::KeyDown { 
                target, 
                key: v["key"].as_str().unwrap_or("").to_string() 
            },
            "custom" => DomEvent::Custom {
                target,
                command_id: v["command_id"].as_str().unwrap_or("").to_string(),
                params: v["params"].clone(),
            },
            _ => DomEvent::Click { target } // Fallback
        };
        
        Ok(event)
    }
}