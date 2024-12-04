use bevy::{
    asset::AssetMetaCheck,
    log::{Level, LogPlugin},
    prelude::*,
};
use bevy_kira_audio::prelude::*;
use wasm_bindgen::prelude::*;

mod block;
mod camera;
mod cursor;
mod random;
mod ui;

const LOG_FILTER: &str =
    "wgpu=warn,bevy_app=info,bevy_render=info,bevy_ecs=info,bevy_time=info,naga=info,winit=info,cosmic_text=info,offset_allocator=info";

#[wasm_bindgen]
pub fn run(canvas_id: &str) {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Minecrust".into(),
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
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                }),
            MeshPickingPlugin,
            AudioPlugin,
            block::BlockPlugin,
            camera::PlayerCameraPlugin,
            cursor::CursorPlugin,
            ui::UIPligin,
            random::RandomPlugin,
        ))
        .run();
}
