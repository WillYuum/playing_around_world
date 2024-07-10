use bevy::prelude::*;

use crate::resources::game_state::GameState;
use crate::components::ui_components::EnemyCount;

pub fn ui_system(
    mut query: Query<&mut Text, With<EnemyCount>>,
    game_state: Res<GameState>,
) {
    for mut text in query.iter_mut() {
        text.sections[0].value = format!("Narutos: {}", game_state.enemy_count);
    }
}