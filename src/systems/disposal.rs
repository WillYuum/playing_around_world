use bevy::prelude::*;
use crate::components::enemy::{Enemy, Position};
use crate::resources::game_state::GameState;

pub fn enemy_disposal_system(
    mut commands: Commands,
    query: Query<(Entity, &Position), With<Enemy>>,
    mut game_state: ResMut<GameState>,
) {
    for (entity, pos) in query.iter() {
        let distance_to_center = (pos.x * pos.x + pos.y * pos.y).sqrt();
        
        if distance_to_center < 1.0 {
            commands.entity(entity).despawn();
            game_state.enemy_count -= 1;
        }
    }
}