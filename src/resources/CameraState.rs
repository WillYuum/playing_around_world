#[derive(Resource)]
pub struct CameraState {
    pub rotating: bool,
    pub last_cursor_position: Vec2,
}

impl Default for CameraState {
    fn default() -> Self {
        CameraState {
            rotating: false,
            last_cursor_position: Vec2::ZERO,
        }
    }
}

