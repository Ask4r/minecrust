use bevy::prelude::*;

pub struct BlocksPlugin;

impl Plugin for BlocksPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_blocks);
    }
}

fn spawn_blocks(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube_mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));

    let texture_image: Handle<Image> = asset_server.load("grass.png");

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_image.clone()),
        reflectance: 0.02,
        unlit: true,
        ..default()
    });

    for x in 0..10 {
        for y in 0..10 {
            commands.spawn(PbrBundle {
                transform: Transform::from_xyz(x as f32, y as f32, 0.0),
                mesh: cube_mesh.clone(),
                material: material_handle.clone(),
                ..default()
            });
        }
    }
}

//fn spawn_blocks(
//    mut commands: Commands,
//    asset_server: Res<AssetServer>,
//    mut meshes: ResMut<Assets<Mesh>>,
//    mut materials: ResMut<Assets<StandardMaterial>>,
//) {
//    let cube_mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
//
//    let texture_image: Handle<Image> = asset_server.load("grass.png");
//
//    let material_handle = materials.add(StandardMaterial {
//        base_color_texture: Some(texture_image.clone()),
//        reflectance: 0.02,
//        unlit: true,
//        ..default()
//    });
//
//    for x in 1..11 {
//        for z in 1..11 {
//            commands.spawn(BlockBundle {
//                marker: Block,
//                mesh: cube_mesh.clone(),
//                sprite: SpriteBundle {
//                    texture: texture_image.clone(),
//                    transform: Transform::from_xyz(x as f32, 0.0, z as f32),
//                    ..default()
//                },
//            });
//        }
//    }
//}

//struct Pos { x: usize, y: usize, z: usize, } #[derive(Clone)] struct Block { id: usize, } impl Block { pub const AIR: Block = Block { id: 0 };
//    pub const GRASS: Block = Block { id: 1 };
//
//    pub fn draw(x: f32, y: f32, z: f32) {
//        let cube_mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
//    }
//}

//struct Chunk {
//    pos: Pos,
//    blocks: Vec<Block>,
//}
//
//impl Chunk {
//    pub const SIDE_SIZE: usize = 64;
//    pub const HEIGHT: usize = 64;
//
//    pub fn new() -> Self {
//        Self {
//            pos: Pos { x: 0, y: 0, z: 0 },
//            blocks: vec![Block::AIR; Self::SIDE_SIZE * Self::SIDE_SIZE * Self::HEIGHT],
//        }
//    }
//
//    pub fn get_block(&self, x: usize, y: usize, z: usize) -> Option<&Block> {
//        if x >= Self::SIDE_SIZE || y >= Self::SIDE_SIZE || z >= Self::HEIGHT {
//            return None;
//        } else {
//            return Some(
//                &self.blocks[x + y * Self::SIDE_SIZE + z * Self::SIDE_SIZE * Self::SIDE_SIZE],
//            );
//        }
//    }
//
//    pub fn draw() {}
//}
