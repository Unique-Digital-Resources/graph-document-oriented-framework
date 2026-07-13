pub mod range_slider;
pub mod color_wheel;
pub mod harmony_wheel;
pub mod color_preview;
pub mod palette_grid;
pub mod rect_test;
pub mod wheel_mode;

use engine::core::graph::Graph;
use headless_ui::view_graph::storage::ViewGraph;
use headless_ui::nodes::widgets::{WidgetKind, ContainerNode, ContainerLayout};
use uuid::Uuid;

pub fn init_view(view: &mut ViewGraph, rect_id: Uuid, palette_id: Uuid) {
    let root = view.insert(WidgetKind::Container(ContainerNode::new(ContainerLayout::Stack)));
    let _ = view.set_root(root);

    let rect_id_ui = view.insert(WidgetKind::Custom(rect_test::create(rect_id)));
    let _ = view.attach(root, rect_id_ui);

    let card_panel = view.insert(WidgetKind::Container(ContainerNode::new(ContainerLayout::Stack)));
    let _ = view.attach(root, card_panel);

    let mode_id = view.insert(WidgetKind::Custom(wheel_mode::create()));
    let _ = view.attach(card_panel, mode_id);

    let wheel_id = view.insert(WidgetKind::Custom(color_wheel::create(rect_id)));
    let _ = view.attach(card_panel, wheel_id);
    
    let hue_id = view.insert(WidgetKind::Custom(range_slider::create_hue(rect_id)));
    let _ = view.attach(card_panel, hue_id);
    
    let sat_id = view.insert(WidgetKind::Custom(range_slider::create_sat(rect_id)));
    let _ = view.attach(card_panel, sat_id);
    
    let light_id = view.insert(WidgetKind::Custom(range_slider::create_light(rect_id)));
    let _ = view.attach(card_panel, light_id);
    
    let alpha_id = view.insert(WidgetKind::Custom(range_slider::create_alpha(rect_id)));
    let _ = view.attach(card_panel, alpha_id);

    let preview_id = view.insert(WidgetKind::Custom(color_preview::create(rect_id)));
    let _ = view.attach(card_panel, preview_id);

    let harm_wheel_id = view.insert(WidgetKind::Custom(harmony_wheel::create(rect_id)));
    let _ = view.attach(card_panel, harm_wheel_id);

    let palette_grid_id = view.insert(WidgetKind::Custom(palette_grid::create(palette_id)));
    let _ = view.attach(card_panel, palette_grid_id);
}

/// Orchestrates the sync by delegating to each component's specific sync function
pub fn sync_ui(view: &mut ViewGraph, graph: &Graph) {
    // 1. Find global UI state (wheel mode)
    let mut current_wheel_mode = "Ranges".to_string();
    let mut find_stack = match view.root() { Some(r) => vec![r], None => return };
    while let Some(id) = find_stack.pop() {
        if let Some(WidgetKind::Custom(c)) = view.get(id) {
            if c.kind == "wheel-mode" {
                if let Some(mode) = c.data["mode"].as_str() {
                    current_wheel_mode = mode.to_string();
                }
            }
        }
        find_stack.extend(view.children(id));
    }

    // 2. Let each component sync itself
    let mut stack = match view.root() { Some(r) => vec![r], None => return };
    while let Some(id) = stack.pop() {
        if let Some(WidgetKind::Custom(c)) = view.get_mut(id) {
            match c.kind.as_str() {
                "rect-test" => rect_test::sync(c, graph),
                "color-wheel" => color_wheel::sync(c, graph, &current_wheel_mode),
                "harmony-wheel" => harmony_wheel::sync(c, graph, &current_wheel_mode),
                "hue-slider" | "sat-slider" | "light-slider" | "alpha-slider" => range_slider::sync(c, graph, &current_wheel_mode),
                "color-preview" => color_preview::sync(c, graph),
                "palette-grid" => palette_grid::sync(c, graph),
                _ => {}
            }
        }
        stack.extend(view.children(id));
    }
}