//! Traverses the View Graph and generates `DrawCall`s.

use crate::ui::headless::nodes::ui_node::UiNodeId;
use crate::ui::headless::nodes::widgets::WidgetKind;
use crate::ui::headless::view_graph::storage::ViewGraph;
use super::draw_calls::DrawCall;

pub struct Painter;

impl Painter {
    pub fn render(view: &ViewGraph) -> Vec<DrawCall> {
        let mut calls = Vec::new();
        // Default clear screen
        calls.push(DrawCall::ClearRect(0.0, 0.0, 1920.0, 1080.0)); 
        
        if let Some(root) = view.root() {
            Self::render_node(view, root, &mut calls);
        }
        calls
    }

    fn render_node(view: &ViewGraph, id: UiNodeId, calls: &mut Vec<DrawCall>) {
        let widget = match view.get(id) {
            Some(w) => w,
            None => return,
        };
        let b = widget.ui_node().bounds;

        match widget {
            WidgetKind::Container(_) => {
                calls.push(DrawCall::StrokeRect {
                    x: b.x, y: b.y, w: b.width, h: b.height,
                    color: "#cccccc".into(), width: 1.0,
                });
            }
            WidgetKind::Button(btn) => {
                calls.push(DrawCall::FillRect {
                    x: b.x, y: b.y, w: b.width, h: b.height,
                    color: "#007bff".into(),
                });
                calls.push(DrawCall::FillText {
                    text: btn.label.clone(),
                    x: b.x + 10.0, y: b.y + 20.0,
                    font: "14px Arial".into(),
                    color: "#ffffff".into(),
                });
            }
            WidgetKind::Label(lbl) => {
                calls.push(DrawCall::FillText {
                    text: lbl.text.clone(),
                    x: b.x, y: b.y + 16.0,
                    font: "14px Arial".into(),
                    color: "#000000".into(),
                });
            }
            _ => {
                calls.push(DrawCall::StrokeRect {
                    x: b.x, y: b.y, w: b.width, h: b.height,
                    color: "#ff0000".into(), width: 1.0,
                });
            }
        }

        for child in view.children(id) {
            Self::render_node(view, child, calls);
        }
    }
}