use bevy::{
    asset::AssetMetaCheck,
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    log::{Level, LogPlugin},
    prelude::*,
};
use bevy_kira_audio::prelude::*;
use bevy_mod_picking::prelude::*;
use wasm_bindgen::prelude::*;

mod block;
mod camera;
mod cursor;
mod random;
mod ui;

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
            FpsOverlayPlugin {
                config: FpsOverlayConfig {
                    text_config: TextStyle {
                        font_size: 50.0,
                        color: Color::srgb(0.0, 1.0, 0.0),
                        font: default(),
                    },
                },
            },
            DefaultPickingPlugins
                .build()
                .disable::<DefaultHighlightingPlugin>(),
            AudioPlugin,
            camera::PlayerCameraPlugin,
            cursor::CursorPlugin,
            block::BlockPlugin,
            ui::UIPligin,
            random::RandomPlugin,
        ))
        .run();
}
