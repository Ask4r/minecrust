use bevy::prelude::*;
use mesh::set_block_uv;

mod mesh;
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
    //let cube_mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let texture_image: Handle<Image> = asset_server.load("grass.png");

    // TODO: This is ugly. Include UV setting in mesh creation.
    let mut block_mesh = mesh::create_block_mesh();
    set_block_uv(uv::TextureUVAlignment::SideTopBottom, &mut block_mesh);

    let mesh_handle = meshes.add(block_mesh);

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_image),
        reflectance: 0.02,
        unlit: true,
        ..default()
    });

    for x in 0..10 {
        for z in 0..10 {
            commands.spawn((
                Block,
                PbrBundle {
                    transform: Transform::from_xyz(x as f32, 0.0, z as f32),
                    mesh: mesh_handle.clone(),
                    material: material_handle.clone(),
                    ..default()
                },
            ));
        }
    }
}
