use std::time::Duration;

use crate::components::debug::ShowAxes;
use crate::components::enemy::{Enemy, Position};
use crate::resources::{config::Config, game_state::GameState};
use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use rand::prelude::*;

pub fn enemy_spawning_system(
    mut commands: Commands,
    time: Res<Time>,
    config: Res<Config>,
    mut game_state: ResMut<GameState>,
    asset_server: Res<AssetServer>,
    mut timer: Local<Timer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    timer.tick(time.delta());
    // println!("timer ticked {:#?}", timer.duration());

    if timer.finished() {
        if game_state.enemy_count < config.max_enemies {
            let mut rng = thread_rng();
            let x: f32 = rng.gen_range(20.0..50.0);
            let z: f32 = rng.gen_range(20.0..50.0);

            println!("Spawned enemy on {}, {}", x, z);

            let scene_handle = asset_server.load("models/low_poly_naruto/scene.gltf#Scene0");
            // let animation_handle: Handle<AnimationClip> = asset_server.load("models/low_poly_naruto/scene.gltf#Animation0");
            commands
                .spawn(SceneBundle {
                    scene: scene_handle.clone(),
                    transform: Transform::from_xyz(x, 1.5, z).with_scale(Vec3::new(0.4, 0.4, 0.4)),
                    ..default()
                })
                .insert(Enemy)
                .insert(Position { x, y: 2.0, z });
                .insert(Position { x, y: 2.0, z })
                .insert(ShowAxes)
                .insert(Aabb::from_min_max(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0)));
                // .insert(animation_handle);

            game_state.enemy_count += 1;
        } else {
            game_state.cooldown_timer.reset();
        }

        timer.set_duration(Duration::new(1, 0));
        timer.reset();
    }
}
