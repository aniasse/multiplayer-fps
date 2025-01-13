use bevy::{prelude::*, window::CursorGrabMode};

use crate::input::CursorState;

#[derive(Resource)]
pub struct MouseSensitivity(pub f32);

#[derive(Resource)]
pub struct PlayerRotation {
    pub yaw: f32,
    pub pitch: f32,
}

impl Default for PlayerRotation {
    fn default() -> Self {
        Self { yaw: 0.0, pitch: 0.0 }
    }
}

#[derive(Component)]
pub struct PlayerCamera;

pub fn setup_fps_camera(mut windows: Query<&mut Window>, cursor_state: Res<CursorState>) {
    if let Ok(mut window) = windows.get_single_mut() {
        if cursor_state.captured {
            window.cursor.grab_mode = CursorGrabMode::Locked;
            window.cursor.visible = false;
        } else {
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
        }
    }
}