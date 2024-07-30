use crate::components::enemy::{Enemy, Position};
use crate::resources::game_state::GameState;
use bevy::math::VectorSpace;
use bevy::prelude::*;

pub fn enemy_disposal_system(
    mut commands: Commands,
    query: Query<(Entity, &Position), With<Enemy>>,
    mut game_state: ResMut<GameState>,
) {
    let target_center = Vec3::ZERO;
    for (entity, pos) in query.iter() {
        let distance_to_center = Vec3::distance_squared(target_center, pos.convert_to_vec3());

        if distance_to_center < 1.0 {
            commands.entity(entity).despawn();
            game_state.enemy_count -= 1;
        }
    }
}
