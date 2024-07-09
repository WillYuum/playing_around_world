use crate::components::enemy;
use bevy::prelude::*;

pub fn enemy_movement_system(
    mut enemies: Query<(&mut enemy::Position, &mut Transform), With<enemy::Enemy>>,
    timer: Res<Time>,
) {
    let speed: f32 = 1.15;
    let move_target = Vec3::new(0.0, 0.0, 0.0);
    for (mut pos, mut transform) in enemies.iter_mut() {
        let direction = move_target - transform.translation;
        transform.translation += direction.normalize() * speed * timer.delta_seconds();
        pos.x = transform.translation.x;
        pos.z = transform.translation.z;
    }
}
