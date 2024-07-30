use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Default for Position {
    fn default() -> Self {
        Position {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl Position {
    pub fn convert_to_vec3(&self) -> Vec3 {
        return Vec3::new(self.x, self.y, self.z);
    }
}
