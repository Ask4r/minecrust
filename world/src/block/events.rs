use crate::camera::PlayerCamera;
use bevy::{picking::prelude::*, prelude::*, window::PrimaryWindow};

const BLOCK_REACH: f32 = 5.0;

#[derive(Event)]
pub struct BlockClickEvent {
    pub entity: Entity,
    pub button: MouseButton,
    pub normal: Vec3,
}

pub fn click_block(
    mut event_writer: EventWriter<BlockClickEvent>,
    mut raycast: MeshRayCast,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<PlayerCamera>>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    let button = if mouse.just_pressed(MouseButton::Right) {
        MouseButton::Right
    } else if mouse.just_pressed(MouseButton::Left) {
        MouseButton::Left
    } else {
        return;
    };

    let window = window_query.single();
    let (camera, transform) = camera_query.single();

    let cursor_position = window.size() * 0.5;
    let Ok(ray) = camera.viewport_to_world(transform, cursor_position) else {
        error!("Cannot compute viewport or camera plane.");
        return;
    };

    let Some((entity, hit)) = raycast.cast_ray(ray, &default()).get(0) else {
        return;
    };
    if hit.distance > BLOCK_REACH {
        return;
    }

    event_writer.send(BlockClickEvent {
        entity: *entity,
        normal: hit.normal,
        button,
    });
}
