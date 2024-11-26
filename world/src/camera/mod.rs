use bevy::{core_pipeline::Skybox, prelude::*};

mod controls;
mod skybox;

//pub const SKYBOX_SHEET: &str = "mars_spritesheet.png";
const SKYBOX_SHEET: &str = "skysheet.png";

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
struct PlayerCamera;

fn create_camera_with_skybox(mut commands: Commands, asset_server: Res<AssetServer>) {
    let skybox_image = asset_server.load(SKYBOX_SHEET);

    commands.insert_resource(skybox::SkyboxCubemap {
        image: skybox_image.clone(),
        loaded: false,
    });
    commands.spawn((
        PlayerCamera,
        controls::CameraController {
            rotation: Vec2::ZERO,
        },
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 4.0, 5.0),
            ..default()
        },
        Skybox {
            image: skybox_image,
            brightness: 1000.0,
        },
    ));
}
