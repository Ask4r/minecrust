use crate::{camera::PlayerCamera, random::RandomSource};
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_kira_audio::prelude::*;
use bevy_mod_raycast::prelude::*;
use rand::Rng;

const MAX_DISTANCE: f32 = 5.0;

pub fn delete_block(
    mut commands: Commands,
    mut raycast: Raycast,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<PlayerCamera>>,
    mouse: Res<ButtonInput<MouseButton>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut random_source: ResMut<RandomSource>,
) {
    if !mouse.just_pressed(MouseButton::Right) {
        return;
    }

    let window = window_query.single();
    let (camera, transform) = camera_query.single();

    let cursor_position = window.size() * 0.5;
    let Some(ray) = camera.viewport_to_world(transform, cursor_position) else {
        error!("Cannot compute viewport or camera plane.");
        return;
    };

    let Some((entity, data)) = raycast.cast_ray(ray, &default()).get(0) else {
        return;
    };
    if data.distance() > MAX_DISTANCE {
        return;
    }

    commands.entity(*entity).despawn();

    let rng = &mut random_source.0;
    let audio_path = match rng.gen_range(1..=10) {
        1 => "audio/dirt_break1.mp3",
        2 => "audio/dirt_break2.mp3",
        3 => "audio/dirt_break3.mp3",
        4 => "audio/dirt_break4.mp3",
        5 => "audio/dirt_break5.mp3",
        6 => "audio/dirt_break6.mp3",
        7 => "audio/dirt_break7.mp3",
        8 => "audio/dirt_break8.mp3",
        9 => "audio/dirt_break9.mp3",
        10 => "audio/dirt_break10.mp3",
        _ => {
            error!("Unreachable. Rand cases coverage is not exhaustive.");
            return;
        }
    };

    audio.play(asset_server.load(audio_path));
}
