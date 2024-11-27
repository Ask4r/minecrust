use bevy::prelude::*;

mod uv;

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_blocks);
    }
}

#[derive(Component)]
pub struct Block;

fn spawn_blocks(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let grass_texture = asset_server.load("grass.png");
    let dirt_texture = asset_server.load("dirt.png");

    let grass_material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(grass_texture),
        reflectance: 0.02,
        unlit: true,
        ..default()
    });

    let dirt_material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(dirt_texture),
        reflectance: 0.02,
        unlit: true,
        ..default()
    });

    let block_mesh = Cuboid::from_length(1.0).mesh().build();

    let mut grass_mesh = block_mesh.clone();
    let mut dirt_mesh = block_mesh;

    grass_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uv::uv_sides_top_bottom_alignment());
    let grass_mesh_handle = meshes.add(grass_mesh);

    dirt_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uv::uv_single_side_alignment());
    let dirt_mesh_handle = meshes.add(dirt_mesh);

    for x in 0..10 {
        for z in 0..10 {
            commands.spawn((
                Block,
                PbrBundle {
                    transform: Transform::from_xyz(x as f32, 0.0, z as f32),
                    mesh: grass_mesh_handle.clone(),
                    material: grass_material_handle.clone(),
                    ..default()
                },
            ));
        }
    }

    for x in 0..10 {
        for z in 0..10 {
            commands.spawn((
                Block,
                PbrBundle {
                    transform: Transform::from_xyz(x as f32, -1.0, z as f32),
                    mesh: dirt_mesh_handle.clone(),
                    material: dirt_material_handle.clone(),
                    ..default()
                },
            ));
        }
    }
}
