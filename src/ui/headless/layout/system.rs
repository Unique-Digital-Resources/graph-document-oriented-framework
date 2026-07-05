use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use crate::core::graph::storage::Graph;
use crate::core::scheduler::queue::Scheduler;
use crate::core::signal::types::Signal;
use crate::core::system::interface::System;

use crate::ui::headless::nodes::ui_node::{Bounds, UiNodeId};
use crate::ui::headless::nodes::widgets::{ContainerLayout, WidgetKind};
use crate::ui::headless::view_graph::storage::ViewGraph;

pub struct LayoutSystem {
    view_graph: Arc<Mutex<ViewGraph>>,
    viewport: (f32, f32),
}

impl LayoutSystem {
    pub fn new(view_graph: Arc<Mutex<ViewGraph>>) -> Self {
        Self {
            view_graph,
            viewport: (1280.0, 720.0),
        }
    }

    pub fn set_viewport(&mut self, w: f32, h: f32) {
        self.viewport = (w, h);
    }

    fn compute_layout(
        view: &ViewGraph,
        id: UiNodeId,
        origin: (f32, f32),
        available: (f32, f32),
        results: &mut HashMap<UiNodeId, Bounds>,
    ) {
        let widget = match view.get(id) {
            Some(w) => w,
            None => return,
        };

        let (bounds, layout) = match widget {
            WidgetKind::Container(c) => {
                let b = Bounds::new(origin.0, origin.1, available.0, available.1);
                (b, c.layout)
            }
            _ => {
                // Leaf nodes take the available width, but a fixed height of 24.0
                let b = Bounds::new(origin.0, origin.1, available.0, 24.0);
                results.insert(id, b);
                return;
            }
        };

        results.insert(id, bounds);

        let children = view.children(id);
        match layout {
            ContainerLayout::Stack | ContainerLayout::Absolute => {
                for child in children {
                    Self::compute_layout(view, child, origin, (available.0, available.1), results);
                }
            }
            ContainerLayout::Row => {
                let mut x = origin.0;
                let count = children.len().max(1);
                let slot_w = available.0 / count as f32;
                for child in children {
                    Self::compute_layout(view, child, (x, origin.1), (slot_w, available.1), results);
                    x += slot_w;
                }
            }
            ContainerLayout::Column => {
                let mut y = origin.1;
                let count = children.len().max(1);
                let slot_h = available.1 / count as f32;
                for child in children {
                    Self::compute_layout(view, child, (origin.0, y), (available.0, slot_h), results);
                    y += slot_h;
                }
            }
            ContainerLayout::Grid => {
                let n = children.len() as f32;
                let cols = n.sqrt().ceil().max(1.0);
                let rows = (n / cols).ceil();
                let cell_w = available.0 / cols;
                let cell_h = available.1 / rows;
                for (i, child) in children.into_iter().enumerate() {
                    let r = (i as f32 / cols).floor();
                    let c = i as f32 % cols;
                    Self::compute_layout(
                        view, child,
                        (origin.0 + c * cell_w, origin.1 + r * cell_h),
                        (cell_w, cell_h),
                        results,
                    );
                }
            }
        }
    }
}

impl System for LayoutSystem {
    fn name(&self) -> &str {
        "ui.layout"
    }

    fn filter(&self, signal: &Signal) -> bool {
        matches!(signal.signal_type.as_str(), "UiNodeAdded" | "UiNodeRemoved" | "UiNodePropertyChanged" | "NodePropertyChanged")
    }

    fn execute(&self, _graph: &Graph, _scheduler: &mut Scheduler, _signal: &Signal) {
        let mut view = match self.view_graph.lock() {
            Ok(guard) => guard,
            Err(_) => return,
        };

        let root = match view.root() {
            Some(r) => r,
            None => return,
        };

        let (w, h) = self.viewport;

        // 1. Recursively compute bounds for the whole tree
        let mut computed_bounds = HashMap::new();
        Self::compute_layout(&view, root, (0.0, 0.0), (w, h), &mut computed_bounds);

        // 2. Apply the computed bounds to the widgets
        for (id, bounds) in computed_bounds {
            if let Some(widget) = view.get_mut(id) {
                widget.ui_node_mut().bounds = bounds;
            }
        }
    }
}