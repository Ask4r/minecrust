use bevy::{core_pipeline::Skybox, prelude::*};

mod controls;
mod skybox;

pub struct PlayerCameraPlugin;

impl Plugin for PlayerCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_camera_with_skybox)
            .add_systems(
                Update,
                (
                    controls::rotate_camera,
                    controls::move_camera,
                    skybox::reinterpret_cubemap,
                ),
            );
    }
}

#[derive(Component)]
pub struct PlayerCamera;

fn create_camera_with_skybox(mut commands: Commands, asset_server: Res<AssetServer>) {
    let skybox_image = asset_server.load("skysheet.png");

    commands.insert_resource(skybox::SkyboxCubemap {
        image: skybox_image.clone(),
        loaded: false,
    });
    commands.spawn((
        PlayerCamera,
        Camera3d::default(),
        controls::CameraController {
            translation: Vec3::new(0.0, 3.0, 5.0),
            rotation: Vec2::ZERO,
        },
        Skybox {
            image: skybox_image,
            brightness: 1000.0,
            rotation: default(),
        },
    ));
}
