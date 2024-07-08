use bevy::prelude::*;

#[derive(Resource)]
pub struct SpawnData {
    pub spawn_enemy_rate: f32,
    pub time_till_next_spawn: f32,
}


pub fn handle_spawn_enemy(
    time: Res<Time>,
    mut spawner_data: ResMut<SpawnData>,
    mut commands: Commands,
) {
    // Update the time until the next spawn
    spawner_data.time_till_next_spawn -= time.delta_seconds();

    // Check if it's time to spawn a new enemy
    if spawner_data.time_till_next_spawn <= 0.0 {
        // Reset the timer
        spawner_data.time_till_next_spawn = spawner_data.spawn_enemy_rate;

        print!("spawning enemy");
        // Spawn the enemy here
        // Example spawn command:
        // commands.spawn_bundle(EnemyBundle::default());

        // Add your enemy spawn logic above
    }
}


fn create_enemy_instance(){

}