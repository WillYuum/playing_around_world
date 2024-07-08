use bevy::{ prelude::*};


fn main() {
    println!("Hello, world!");
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup)
    .add_systems(Update, camera_rotation)
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


fn camera_rotation(time: Res<Time>,
     keyboard_input:Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>
){
    let rotation_speed = std::f32::consts::PI / 2.0; // Rotate 90 degrees per second
    for mut transform in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::KeyQ) {
            transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(rotation_speed * time.delta_seconds()));
        }
        if keyboard_input.pressed(KeyCode::KeyE) {
            transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(-rotation_speed * time.delta_seconds()));
        }
    }
}