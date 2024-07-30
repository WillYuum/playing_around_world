use bevy::prelude::*;

#[derive(Resource)]
pub struct GameState {
    pub enemy_count: usize,
    pub cooldown_timer: Timer,
}
