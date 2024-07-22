use bevy::{
    animation::{animate_targets, RepeatAnimation},
    app::AppLabel, 
    asset::AssetMetaCheck,
    log::LogPlugin,
    prelude::*,
};
use resources::{
    animations::Animations,
    asset_resources::CardAsset,
    game_state::GameState
};

use bevy_dev_console::prelude::*;

pub mod components;
pub mod resources;
pub mod systems;


fn main() {

    use systems::{
        animation::{enemy_animation_system, setup_enemy_animations},
        camera_system::{rotate_camera, zoom_camera},
        spawning::enemy_spawning_system,
        movement::enemy_movement_system,
        disposal::enemy_disposal_system,
        ui_system::ui_system,
    };

    let camera_state = resources::camera_state::CameraState {
        ..Default::default()
    };
    let game_state = resources::game_state::GameState {
        enemy_count: 0,
        cooldown_timer: Timer::from_seconds(10.5, TimerMode::Repeating),
    };
    let config = resources::config::Config {
        enemy_spawn_rate: 0.1,
        max_enemies: 1000,
        spawn_cooldown: 5.0,
    };


    let mut built_app = App::new();
    
    built_app
        // .add_plugins(ConsoleLogPlugin::default())
        .add_plugins(DefaultPlugins.build().disable::<LogPlugin>())
        // .add_plugins(DevConsolePlugin)
        // .add_plugins((
        //     // Start capturing logs before the default plugins initiate.
        //     ConsoleLogPlugin::default(),
        //     // Add the default plugins without the LogPlugin.
        //     // Not removing the LogPlugin will cause a panic!
        //     DefaultPlugins.build().disable::<LogPlugin>(),
        //     // Add the dev console plugin itself.
        //     // DevConsolePlugin,
        //     // default()
        // ))
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes_override: Some(true),

            // Wasm builds will check for meta files (that don't exist) if this isn't set.
            // This causes errors and even panics in web builds on itch.
            // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
            meta_check: AssetMetaCheck::Never,
            ..Default::default()
        }))
        .insert_resource(game_state)
        .insert_resource(config)
        .insert_resource(camera_state)
        .add_systems(Startup, setup)
        .add_systems(Update, setup_enemy_animations.before(animate_targets))
        .add_systems(Update, (rotate_camera, zoom_camera))
        .add_systems(Update, enemy_spawning_system)
        .add_systems(Update, enemy_movement_system)
        .add_systems(Update, enemy_disposal_system)
        .add_systems(Update, enemy_animation_system)
        .add_systems(Update, ui_system)
        .add_systems(Update, systems::debug::draw_axes);


    built_app.run();
}

fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    use resources::asset_resources::{
        // Cards2DResources,
        CardAsset,
        CardType,
        NarutoResource,
    };
        
    let naruto_model_handle = asset_server.load("models/low_poly_naruto/scene.gltf#Scene0");
    commands.insert_resource(NarutoResource {
        model: naruto_model_handle,
    });

    let card_files = [
        ("sprites/cards_2d/PineTools.com_files/red_ace.png", CardType::Ace),
        ("sprites/cards_2d/PineTools.com_files/red_king.png", CardType::King),
        ("sprites/cards_2d/PineTools.com_files/red_queen.png", CardType::Queen),
        ("sprites/cards_2d/PineTools.com_files/red_jack.png", CardType::Jack),
    ];

    let all_textures: Vec<CardAsset> = card_files.iter().map(|(path, card_type)| {
        let texture : Handle<Image>  = asset_server.load(*path);
        CardAsset { texture, card_type: card_type.clone() }
    })
    .collect();


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
