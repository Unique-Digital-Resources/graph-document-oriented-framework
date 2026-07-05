//! Defines the DOM events coming from the Web Bridge.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DomEvent {
    Click { target: Uuid },
    Input { target: Uuid, value: String },
    KeyDown { target: Uuid, key: String },
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
            _ => DomEvent::Click { target } // Fallback
        };
        
        Ok(event)
    }
}