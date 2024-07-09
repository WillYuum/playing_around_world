use std::time::Duration;

use bevy::prelude::*;
use crate::components::enemy::{Enemy, Position};
use crate::resources::{config::Config, game_state::GameState};
use rand::prelude::*;

pub fn enemy_spawning_system(
    mut commands: Commands,
    time: Res<Time>,
    config: Res<Config>,
    mut game_state: ResMut<GameState>,
    mut timer: Local<Timer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    timer.tick(time.delta());
    println!("timer ticked {:#?}", timer.duration());

    // if timer.finished() && game_state.cooldown_timer.finished() {

    if timer.finished() {
        if game_state.enemy_count < config.max_enemies {
            // let mut rng = rand::thread_rng();
            let mut rng  = thread_rng();
            let x: f32 = rng.gen_range(-50.0..50.0);
            let z: f32 = rng.gen_range(-50.0..50.0);

            // let mut nums: Vec<i32> = (1..100).collect();
            // nums.shuffle(&mut rng);

            println!("Spawned enemy on {}, {}", x, z);
            
            commands.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
                material: materials.add(Color::srgb_u8(250, 55, 10)),
                transform: Transform::from_xyz(0.0, 0.5, 0.0),
                ..default()
            })
            .insert(Enemy)
            .insert(Position{x, y:0.0, z});
            
            game_state.enemy_count += 1;
        } else {
            game_state.cooldown_timer.reset();
        }

        timer.set_duration(Duration::new(1, 0));
        timer.reset();
    }
}