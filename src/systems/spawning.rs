use std::f32::consts::PI;
use std::time::Duration;
use crate::components::animation_player::CustomAnimator;
use crate::components::enemy::{Enemy, Position};
use crate::resources::animations::EnemyAnimations;
use crate::resources::asset_resources::NarutoResource;
use crate::resources::{config::Config, game_state::GameState};
use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use rand::prelude::*;

pub fn enemy_spawning_system(
    mut commands: Commands,
    time: Res<Time>,
    config: Res<Config>,
    mut game_state: ResMut<GameState>,
    mut timer: Local<Timer>,
    naruto_resource: Res<NarutoResource>,
    enemy_animations: Res<EnemyAnimations>,
) {
    timer.tick(time.delta());
    if timer.finished() {
        if game_state.enemy_count < config.max_enemies {
            let mut rng = thread_rng();
            
            // Constants
            let min_boid_spawn_point_radius = 50.0;
            let max_boid_spawn_point_radius = 80.0; 
            let enemy_dist_from_boid_spawn_point = 10.0;
            let num_enemies_per_spawn_point = rng.gen_range(11..15);

            
            let spawn_point_x = rng.gen_range(min_boid_spawn_point_radius..max_boid_spawn_point_radius);
            let spawn_point_z = rng.gen_range(min_boid_spawn_point_radius..max_boid_spawn_point_radius);
            let angle = rng.gen_range(0.0..PI * 2.0);
            
            let spawn_point = Vec3::new(
                angle.cos() * spawn_point_x,
                0.0,
                angle.sin() * spawn_point_z,
            );

            for _ in 0..num_enemies_per_spawn_point {
                let enemy_offset = Vec3::new(
                    rng.gen_range(-enemy_dist_from_boid_spawn_point..enemy_dist_from_boid_spawn_point),
                    0.0, // Adjust as needed for height offset
                    rng.gen_range(-enemy_dist_from_boid_spawn_point..enemy_dist_from_boid_spawn_point),
                );

                let enemy_position = spawn_point + enemy_offset;

                let enemy_transform = Transform::from_translation(enemy_position)
                    .with_scale(Vec3::splat(0.4))
                    .with_rotation(Quat::IDENTITY);
                
                commands.spawn((
                    SceneBundle {
                        scene: naruto_resource.model.clone(),
                        transform: enemy_transform,
                        ..Default::default()
                    },
                    Enemy,
                    Position {
                        x: enemy_position.x,
                        y: enemy_position.y,
                        z: enemy_position.z,
                    },
                    Aabb::from_min_max(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0)),
                      
                ));

                // setup_animation(&mut commands, entity, &enemy_animations);

                game_state.enemy_count += 1;
            }
        } else {
            game_state.cooldown_timer.reset();
        }

        timer.set_duration(Duration::from_millis(100));
        timer.reset();
    }
}
