use bevy::prelude::*;

#[derive(Resource)]
pub struct CameraState {
    pub rotating: bool,
    pub last_cursor_position: Vec2,
    pub distance: f32,
}

impl Default for CameraState {
    fn default() -> Self {
        CameraState {
            rotating: false,
            last_cursor_position: Vec2::ZERO,
            distance: 5.0,
        }
    }
}

