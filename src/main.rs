#![cfg_attr(not(doc), no_main)]
#![feature(stmt_expr_attributes)]
#![feature(try_trait_v2)]
#![allow(dead_code)]
#[allow(unused_imports)]

use nannou::prelude::*;
use nannou::{
    app::{self, App},
    wgpu::{Backends, DeviceDescriptor, Limits},
};
use std::sync::RwLock;
use wasm_bindgen::prelude::*;
use std::rc::Rc;

pub mod task;
pub mod console;

#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let window = web_sys::window().unwrap();

    thread_local!(static MODEL: RwLock<Option<Model>> = Default::default());
    let model = model();

    MODEL.with(|m| m.write().unwrap().replace(model));

    task::block_on(async { app::Builder::new_async(|app| {
            Box::new(async {
                create_window(app).await;
                MODEL.with(|m| m.write().unwrap().take().unwrap())
            })
        })
        .backends(Backends::PRIMARY | Backends::GL)
        .update(update)
        .run_async()
        .await;
    });

    Ok(())
}

async fn create_window(app: &App) {
    let device_desc = DeviceDescriptor {
        limits: Limits {
            max_texture_dimension_2d: 8192,
            ..Limits::downlevel_webgl2_defaults()
        },
        ..Default::default()
    };

    app.new_window()             
        .size(1024, 1024)
        .device_descriptor(device_desc)
        .title("Pathfinder 2e Battle Sim")
        .view(view)
        // .mouse_pressed(mouse_pressed)
        // .mouse_released(mouse_released)
        .event(event)
        .build_async()
        .await
        .unwrap();
}

fn event(_app: &App, _model: &mut Model, _event: WindowEvent) {
}

struct Model {
}

fn model() -> Model {
    Model {
    }
}

fn update(_app: &App, _m: &mut Model, _update: Update) {
}

fn view(app: &App, m: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    draw.to_frame(app, &frame).unwrap();

    let win_rect = app.main_window().rect().pad(20.0);

    let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.\n\nResize the window to test dynamic layout.";

    //                         L     o     r     e     m           i    p    s    u    m
    let glyph_colors = vec![BLUE, BLUE, BLUE, BLUE, BLUE, BLACK, RED, RED, RED, RED, RED];

    draw.text(text)
        .color(BLACK)
        .glyph_colors(glyph_colors)
        .font_size(24)
        .wh(win_rect.wh());

    draw.to_frame(app, &frame).unwrap();
}
