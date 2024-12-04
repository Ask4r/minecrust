use bevy::prelude::*;

#[derive(Resource)]
pub struct BlocksMaterials {
    pub transparent: MeshMaterial3d<StandardMaterial>,
    pub dirt: MeshMaterial3d<StandardMaterial>,
    pub grass: MeshMaterial3d<StandardMaterial>,
    pub grass_top: MeshMaterial3d<StandardMaterial>,
    pub grass_side: MeshMaterial3d<StandardMaterial>,
}

impl FromWorld for BlocksMaterials {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>().clone();
        let mut materials = world.resource_mut::<Assets<StandardMaterial>>();
        Self {
            transparent: MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::srgba(
                0.0, 0.0, 0.0, 0.0,
            )))),
            dirt: gen_material("dirt.png", &mut materials, &asset_server),
            grass: gen_material("grass.png", &mut materials, &asset_server),
            grass_top: gen_material("grass_top.png", &mut materials, &asset_server),
            grass_side: gen_material("grass_side.png", &mut materials, &asset_server),
        }
    }
}

#[inline(always)]
fn gen_material(
    path: &str,
    materials: &mut Assets<StandardMaterial>,
    asset_server: &AssetServer,
) -> MeshMaterial3d<StandardMaterial> {
    MeshMaterial3d(materials.add(StandardMaterial {
        base_color_texture: Some(asset_server.load(path)),
        ..default()
    }))
}
