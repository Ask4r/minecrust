use super::uv::{uv_sides_top_bottom_alignment, uv_single_side_alignment};
use bevy::prelude::*;

#[derive(Resource)]
pub struct BlocksMeshes {
    pub side_top_bottom: Mesh3d,
    pub single_side: Mesh3d,
}

impl FromWorld for BlocksMeshes {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world.resource_mut::<Assets<Mesh>>();

        let block: Mesh = Cuboid::from_length(1.0).into();

        let mut side_top_bottom_mesh = block.clone();
        side_top_bottom_mesh
            .insert_attribute(Mesh::ATTRIBUTE_UV_0, uv_sides_top_bottom_alignment());
        let side_top_bottom = Mesh3d(meshes.add(side_top_bottom_mesh));

        let mut single_side_mesh = block;
        single_side_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uv_single_side_alignment());
        let single_side = Mesh3d(meshes.add(single_side_mesh));

        Self {
            side_top_bottom,
            single_side,
        }
    }
}
