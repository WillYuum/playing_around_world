use bevy::prelude::*;
use crate::components::enemy;

pub fn enemy_movement_system(
    mut query: Query<(&mut enemy::Position, &mut Transform), With<enemy::Enemy>>,
    time: Res<Time>,
) {
    for (mut pos, mut transform) in query.iter_mut() {
        let direction = Vec3::ZERO - transform.translation;
        let distance = direction.length();
        
        if distance > 0.1 {
            let movement: Vec3 = direction.normalize() * 0.1 * time.delta_seconds();
            transform.translation += movement;
            pos.x = transform.translation.x;
            pos.z = transform.translation.z;
        }
    }
}