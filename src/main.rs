use bevy:: prelude::*;

pub mod camera_behavior;
pub mod enemy_behavior;

fn main() {

    let camera_state = camera_behavior::controls::CameraState{..Default::default()};
    let spawner_data = enemy_behavior::enemy_spawner::SpawnData{
        spawn_enemy_rate: 2.0,
        time_till_next_spawn: 2.0,
    };

    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup)
    .add_systems(Update, camera_behavior::controls::rotate_camera)
    .add_systems(Update, enemy_behavior::enemy_attack::handle_enemy_attack)
    .add_systems(Update, enemy_behavior::enemy_spawner::handle_spawn_enemy)
    .insert_resource(camera_state)
    .insert_resource(spawner_data)
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
