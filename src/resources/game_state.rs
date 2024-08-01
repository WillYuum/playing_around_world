use bevy::prelude::*;

#[derive(Resource)]
pub struct GameState {
    pub enemy_count: usize,
    pub cooldown_timer: Timer,
    pub mana_max: f32,
    pub curr_mana: f32,
}
