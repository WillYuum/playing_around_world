use std::time::Duration;

use crate::components::enemy::{Enemy, Position};
use crate::resources::{config::Config, game_state::GameState};
use bevy::prelude::*;
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
    // println!("timer ticked {:#?}", timer.duration());

    if timer.finished() {
        if game_state.enemy_count < config.max_enemies {
            let mut rng = thread_rng();
            let x: f32 = rng.gen_range(20.0..50.0);
            let z: f32 = rng.gen_range(20.0..50.0);

            println!("Spawned enemy on {}, {}", x, z);

            commands
                .spawn(PbrBundle {
                    mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
                    material: materials.add(Color::srgb_u8(250, 55, 10)),
                    transform: Transform::from_xyz(x, 1.5, z),
                    ..default()
                })
                .insert(Enemy)
                .insert(Position { x, y: 2.0, z });

            game_state.enemy_count += 1;
        } else {
            game_state.cooldown_timer.reset();
        }

        timer.set_duration(Duration::new(1, 0));
        timer.reset();
    }
}
