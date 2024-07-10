use std::time::Duration;

use bevy::{
    animation::{animate_targets, RepeatAnimation},
    prelude::*,
};
use resources::{animations::Animations, asset_resources::NarutoResource, game_state::GameState};


pub mod components;
pub mod resources;
pub mod systems;

fn main() {
    let camera_state = resources::camera_state::CameraState {
        ..Default::default()
    };
    let game_state = resources::game_state::GameState {
        enemy_count: 0,
        cooldown_timer: Timer::from_seconds(10.5, TimerMode::Repeating),
    };
    let config = resources::config::Config {
        enemy_spawn_rate: 0.5,
        max_enemies: 100,
        spawn_cooldown: 5.0,
    };

    


    

    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes_override: Some(true),
            ..Default::default()
        }))
        .insert_resource(game_state)
        .insert_resource(config)
        .insert_resource(camera_state)
        .add_systems(Startup, setup)
        .add_systems(Update, systems::animation::setup_enemy_animations.before(animate_targets))
        .add_systems(Update, systems::camera_system::rotate_camera)
        .add_systems(Update, systems::spawning::enemy_spawning_system)
        .add_systems(Update, systems::movement::enemy_movement_system)
        .add_systems(Update, systems::disposal::enemy_disposal_system)
        .add_systems(Update, systems::animation::enemy_animation_system)
        .add_systems(Update, systems::ui_system::ui_system)
        .add_systems(Update, systems::debug::draw_axes)
        .run();
}

fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {

        
    let naruto_model_handle = asset_server.load("models/low_poly_naruto/scene.gltf#Scene0");
    commands.insert_resource(NarutoResource {
        model: naruto_model_handle,
    });
    

    // Build the animation graph
    let mut graph = AnimationGraph::new();
    let animations = graph
        .add_clips(
            [
                GltfAssetLabel::Animation(0).from_asset("models/low_poly_naruto/scene.gltf#Scene0"),
            ]
            .into_iter()
            .map(|path| asset_server.load(path)),
            1.0,
            graph.root,
        )
        .collect();

    // Insert a resource with the current scene information
    let graph = graphs.add(graph);
    commands.insert_resource(Animations {
        animations,
        graph: graph.clone(),
    });


    
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
        camera: Camera {
            hdr: true,
            ..default()
        },
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

      // UI setup
    //   commands.spawn(Camera2dBundle::default());
      commands.spawn((
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "hello\nbevy!",
            TextStyle {
                // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 25.0,
                ..default()
            },
        ) // Set the justification of the Text
        .with_text_justify(JustifyText::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        }),
        components::ui_components::EnemyCount,
    ));

}
