use bevy::prelude::*;

#[derive(Resource)]
pub struct Config {
    pub enemy_spawn_rate: f32,
    pub max_enemies: usize,
    pub spawn_cooldown: f32,
}
