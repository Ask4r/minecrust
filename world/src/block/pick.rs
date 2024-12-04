use super::{events::BlockClickEvent, materials::BlocksMaterials, meshes::BlocksMeshes, Block};
use crate::random::RandomSource;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use rand::Rng;

pub fn delete_block(
    mut commands: Commands,
    mut event_reader: EventReader<BlockClickEvent>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut random_source: ResMut<RandomSource>,
) {
    for event in event_reader.read() {
        if event.button != MouseButton::Right {
            continue;
        }

        commands.entity(event.entity).despawn();

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
                continue;
            }
        };
        audio.play(asset_server.load(audio_path));
    }
}

pub fn pick_block(
    mut commands: Commands,
    mut event_reader: EventReader<BlockClickEvent>,
    blocks_query: Query<&Transform, With<Block>>,
    blocks_materials: Res<BlocksMaterials>,
    blocks_meshes: Res<BlocksMeshes>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut random_source: ResMut<RandomSource>,
) {
    for event in event_reader.read() {
        if event.button != MouseButton::Left {
            continue;
        }

        let Ok(transform) = blocks_query.get(event.entity) else {
            continue;
        };
        let spawn_translation = transform.translation + event.normal;

        commands.spawn((
            Block,
            blocks_meshes.single_side.clone(),
            blocks_materials.dirt.clone(),
            Transform::from_translation(spawn_translation),
        ));

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
                continue;
            }
        };
        audio.play(asset_server.load(audio_path));
    }
}
