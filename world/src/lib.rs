use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};
use wasm_bindgen::prelude::*;

use block::BlockPlugin;
use camera::PlayerCameraPlugin;
use cursor::CursorPlugin;

mod block;
mod camera;
mod cursor;

const WINDOW_TITLE: &str = "Minecrust";
const CANVAS_DOM_ID: &str = "#minecrust-world-canvas";
const LOG_FILTER: &str =
    "wgpu=warn,bevy_app=info,bevy_render=info,bevy_ecs=info,bevy_time=info,naga=info,winit=info";

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn run() {
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            title: WINDOW_TITLE.into(),
            canvas: Some(CANVAS_DOM_ID.into()),
            fit_canvas_to_parent: true,
            prevent_default_event_handling: true,
            ..default()
        }),
        ..default()
    };

    let log_plugin = LogPlugin {
        filter: LOG_FILTER.into(),
        level: Level::DEBUG,
        custom_layer: |_| None,
    };

    App::new()
        .add_plugins((
            DefaultPlugins.set(window_plugin).set(log_plugin),
            PlayerCameraPlugin,
            CursorPlugin,
            BlockPlugin,
        ))
        .run();
}
