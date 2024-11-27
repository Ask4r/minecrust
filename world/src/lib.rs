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
const LOG_FILTER: &str =
    "wgpu=warn,bevy_app=info,bevy_render=info,bevy_ecs=info,bevy_time=info,naga=info,winit=info";

//#[wasm_bindgen]
//extern "C" {
//    #[wasm_bindgen(js_namespace = console)]
//    fn log(s: &str);
//}

#[wasm_bindgen]
pub fn run(canvas_id: &str) {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: WINDOW_TITLE.into(),
                        canvas: Some(canvas_id.into()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(LogPlugin {
                    filter: LOG_FILTER.into(),
                    level: Level::DEBUG,
                    custom_layer: |_| None,
                })
                .set(ImagePlugin::default_nearest()),
            PlayerCameraPlugin,
            CursorPlugin,
            BlockPlugin,
        ))
        .run();
}
