use std::time::Duration;

use bevy:: prelude::*;


pub mod systems;
pub mod resources;
pub mod components;


fn main() {

    let camera_state = resources::camera_state::CameraState{..Default::default()};
    let game_state = resources::game_state::GameState{enemy_count: 0, cooldown_timer: Timer::from_seconds(10.5, TimerMode::Repeating)};
    let config = resources::config::Config{enemy_spawn_rate: 0.5, max_enemies: 100, spawn_cooldown: 5.0};

    // game_state.cooldown_timer.set_duration(Duration::new(5, 0));

    App::new()
    .add_plugins(DefaultPlugins)
    .insert_resource(game_state)
    .insert_resource(config)
    .add_systems(Startup, setup)
    .add_systems(Update, systems::camera_system::rotate_camera)
    .add_systems(Update, systems::spawning::enemy_spawning_system)
    .add_systems(Update, systems::movement::enemy_movement_system)
    .add_systems(Update, systems::disposal::enemy_disposal_system)
    .insert_resource(camera_state)
    .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
){
    //Circular Plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(4.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });


    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::srgb_u8(124, 144, 255)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });


        // light
        commands.spawn(PointLightBundle {
            point_light: PointLight {
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        });
    

        // camera
        commands.spawn(Camera3dBundle {
            camera: Camera{
                hdr: true,
                ..default()
            },
            transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        });
    
}
