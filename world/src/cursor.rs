use bevy::{prelude::*, window::CursorGrabMode};

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, cursor_grab);
    }
}

fn cursor_grab(
    mut windows_query: Query<&mut Window>,
    mouse: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
) {
    // NOTE: bevy uses native "requestPointerLock" canvas method when
    // compiled to wasm that hides and locks cursor, so you needn't to
    // change cursor visibility for wasm compilation.
    //
    // However bevy currently "v0.14.1" does not detect native pointer unlocking
    // events to automatically update grab_mode to unlocked state. But you
    // can still change grab_mode directly to unlock pointer and update grab_mode
    // value at once.
    //
    // You may but haven't to listen to events other than <Escape> clicking
    // for unlocking cursor on web and have valid grab_mode in a single action.
    // User might still click <Escape> since encouraging message pop-ups
    // on screen during native pointer locking, which would result in unlocked
    // cursor and outdated grab_mode value. In this scenario trying to lock
    // cursor again by triggering the same event does't lock it, because
    // you cannot change grab_mode from locked to locked thus cannot trigger
    // requestPointerLock. User have to trigger your unlocking event after
    // clicking <Escape> to update grab_mode, so it becomes possible to
    // lock cursor again, which is worse user experience.
    //
    // The chosen option here is to force user to double click <Escape> key:
    // first for native pointer unlocking and second will be detected by bevy
    // for grab_mode updating. This is not very intuitive for users, but allows
    // to avoid need in separate events for unlocking.

    let mut window = windows_query.single_mut();

    if mouse.just_pressed(MouseButton::Left) {
        #[cfg(not(target_family = "wasm"))]
        {
            window.cursor.visible = false;
        }
        window.cursor.grab_mode = CursorGrabMode::Locked;
        info!("Cursor locked. Grab mode {:?}", window.cursor.grab_mode);
    }

    if key.just_pressed(KeyCode::Escape) {
        #[cfg(not(target_family = "wasm"))]
        {
            window.cursor.visible = true;
        }
        window.cursor.grab_mode = CursorGrabMode::None;
        info!("Cursor unlocked. Grab mode {:?}", window.cursor.grab_mode);
    }

    // Possible implementation for event other than <Escape> click
    // which would update grab_mode and unlock pointer at once on web.
    //if mouse.just_pressed(MouseButton::Left) {
    //    #[cfg(target_family = "wasm")]
    //    {
    //        window.cursor.visible = true;
    //    }
    //    window.cursor.grab_mode = CursorGrabMode::None;
    //}
}
