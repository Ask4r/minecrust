use bevy::prelude::*;

mod events;
mod materials;
mod meshes;
mod pick;
mod uv;

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<materials::BlocksMaterials>()
            .init_resource::<meshes::BlocksMeshes>()
            .add_event::<events::BlockClickEvent>()
            .add_systems(Startup, spawn_blocks)
            .add_systems(
                Update,
                (events::click_block, pick::pick_block, pick::delete_block),
            );
    }
}

#[derive(Component)]
#[require(Mesh3d)]
pub struct Block;

fn spawn_blocks(
    mut commands: Commands,
    blocks_materials: Res<materials::BlocksMaterials>,
    blocks_meshes: Res<meshes::BlocksMeshes>,
) {
    for x in 0..10 {
        for z in 0..10 {
            commands.spawn((
                Block,
                blocks_meshes.side_top_bottom.clone(),
                blocks_materials.grass.clone(),
                Transform::from_xyz(x as f32, 0.0, z as f32),
            ));
        }
    }

    for x in 0..10 {
        for y in -10..=-1 {
            for z in 0..10 {
                commands.spawn((
                    Block,
                    blocks_meshes.single_side.clone(),
                    blocks_materials.dirt.clone(),
                    Transform::from_xyz(x as f32, y as f32, z as f32),
                ));
            }
        }
    }
}
