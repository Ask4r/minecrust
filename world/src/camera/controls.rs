use super::PlayerCamera;
use bevy::{input::mouse::MouseMotion, prelude::*};

const MOVE_SPEED: f32 = 15.0;
const YAW_COEF: f32 = 0.003;
const PITCH_COEF: f32 = 0.002;
const PITCH_LOCK: f32 = 1.37; // 0 to PI/2

#[derive(Component)]
pub struct CameraController {
    pub translation: Vec3,
    pub rotation: Vec2,
}

pub fn rotate_camera(
    mut mouse_motion: EventReader<MouseMotion>,
    mut camera_query: Query<(&mut Transform, &mut CameraController), With<PlayerCamera>>,
) {
    let (mut transform, mut controller) = camera_query.single_mut();

    for motion in mouse_motion.read() {
        let yaw = -motion.delta.x * YAW_COEF;
        let pitch = -motion.delta.y * PITCH_COEF;
        // Order of rotations is important, see <https://gamedev.stackexchange.com/a/136175/103059>
        controller.rotation.y += yaw;
        controller.rotation.x = (controller.rotation.x + pitch).clamp(-PITCH_LOCK, PITCH_LOCK);
    }

    let y_quat = Quat::from_axis_angle(Vec3::Y, controller.rotation.y);
    let x_quat = Quat::from_axis_angle(Vec3::X, controller.rotation.x);
    transform.rotation = y_quat * x_quat;

    //// Logging
    //let forward = transform.forward();
    //let yaw = (forward.as_vec3() - forward.y).angle_between(Vec3::Z);
    //let pitch = std::f32::consts::FRAC_PI_2 - forward.angle_between(Vec3::Y);
    //debug!("Camera yaw: {} pitch: {}", yaw, pitch);
}

pub fn move_camera(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<(&mut Transform, &mut CameraController), With<PlayerCamera>>,
    time: Res<Time>,
) {
    let (mut transform, mut controller) = camera_query.single_mut();
    let forward = *transform.forward();
    let right = *transform.right();
    let up = *transform.up();

    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
        direction -= right;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
        direction += right;
    }
    if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
        direction += forward;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
        direction -= forward;
    }
    if keyboard_input.pressed(KeyCode::KeyE) {
        direction += up;
    }
    if keyboard_input.pressed(KeyCode::KeyQ) {
        direction -= up;
    }

    let delta = direction.normalize_or_zero() * MOVE_SPEED * time.delta_seconds();
    controller.translation += delta;

    transform.translation = transform.translation.lerp(controller.translation, 0.2);
}
