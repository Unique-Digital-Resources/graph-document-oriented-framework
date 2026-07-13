use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use std::rc::Rc;
use std::cell::RefCell;

use crate::WgpuApp;

static mut APP_INSTANCE: Option<WgpuApp> = None;
static mut RENDERER: Option<crate::renderer::WgpuRenderer> = None;

#[wasm_bindgen]
pub fn init_renderer(canvas: HtmlCanvasElement) -> Result<(), JsValue> {
    // Initialize panic hook to see Rust errors in the browser console!
    console_error_panic_hook::set_once();

    let app = WgpuApp::new();
    unsafe { APP_INSTANCE = Some(app); }

    // Fallback to 800x600 if the canvas hasn't been sized by CSS yet
    let width = if canvas.client_width() > 0 { canvas.client_width() as u32 } else { 800 };
    let height = if canvas.client_height() > 0 { canvas.client_height() as u32 } else { 600 };
    
    let size = wgpu::Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(), // Use all backends so WebGPU is tried
        ..Default::default()
    });
    
    let surface = instance.create_surface_from_canvas(canvas)
        .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
    
    wasm_bindgen_futures::spawn_local(async move {
        let renderer = crate::renderer::WgpuRenderer::new(surface, size.width, size.height).await;
        unsafe { RENDERER = Some(renderer); }
        start_render_loop();
    });

    Ok(())
}

fn start_render_loop() {
    let f = Rc::new(RefCell::new(None::<Closure<dyn FnMut()>>));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        unsafe {
            if let (Some(app), Some(renderer)) = (APP_INSTANCE.as_ref(), RENDERER.as_mut()) {
                match renderer.render(&app.graph) {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => renderer.resize(renderer.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => {}
                    Err(e) => web_sys::console::error_1(&format!("Render error: {:?}", e).into()),
                }
            }
        }

        if let Some(cb) = f.borrow().as_ref() {
            web_sys::window()
                .unwrap()
                .request_animation_frame(cb.as_ref().unchecked_ref::<js_sys::Function>())
                .unwrap();
        }
    })));

    let cb = g.borrow();
    let cb = cb.as_ref().unwrap();
    web_sys::window()
        .unwrap()
        .request_animation_frame(cb.as_ref().unchecked_ref::<js_sys::Function>())
        .unwrap();
}

#[wasm_bindgen]
pub fn handle_viewport_input(_camera_id: &str, dx: f32, dy: f32, zoom_delta: f32, interaction_type: &str) {
    unsafe {
        if let Some(renderer) = RENDERER.as_mut() {
            renderer.camera_controller.process_input(dx, dy, zoom_delta, interaction_type);
        }
    }
}

#[wasm_bindgen]
pub fn commit_camera_state(camera_id: &str) {
    unsafe {
        if let Some(app) = APP_INSTANCE.as_mut() {
            if let Some(renderer) = RENDERER.as_ref() {
                let cam_pos = &renderer.camera_controller.position;
                let cam_tgt = &renderer.camera_controller.target;
                let cam_up = &renderer.camera_controller.up;
                
                let params = serde_json::json!({
                    "camera_id": camera_id,
                    "position": [cam_pos.x, cam_pos.y, cam_pos.z],
                    "target": [cam_tgt.x, cam_tgt.y, cam_tgt.z],
                    "up": [cam_up.x, cam_up.y, cam_up.z]
                });

                let mut pipeline = app.create_pipeline();
                if let Err(e) = pipeline.execute("CommitViewportCamera", params) {
                    web_sys::console::error_1(&format!("Failed to commit camera: {:?}", e).into());
                }
            }
        }
    }
}