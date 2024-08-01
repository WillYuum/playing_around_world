use bevy::a11y::accesskit::Size;
use bevy::asset::load_internal_asset;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::render::render_resource::AsBindGroup;
use bevy::{
    animation::{animate_targets, RepeatAnimation},
    asset::AssetMetaCheck,
    prelude::*,
};
use components::progress_bar::ProgressBarMaterial;
use components::ui_components;
use resources::{animations::EnemyAnimations, asset_resources::CardAsset, game_state::GameState};

pub mod components;
pub mod resources;
pub mod systems;

fn main() {
    use systems::{
        animation::handle_play_enemy_animation_on_spawn,
        camera_system::{rotate_camera, zoom_camera},
        disposal::enemy_disposal_system,
        movement::enemy_movement_system,
        spawning::enemy_spawning_system,
        ui_system::{fps_counter_showhide, fps_text_update_system, ui_system},
    };

    let camera_state = resources::camera_state::CameraState {
        ..Default::default()
    };
    let game_state = resources::game_state::GameState {
        enemy_count: 0,
        cooldown_timer: Timer::from_seconds(10.5, TimerMode::Repeating),
        mana_max: 10.0,
        curr_mana: 0.0,
    };
    let config = resources::config::Config {
        enemy_spawn_rate: 0.1,
        max_enemies: 1000,
        spawn_cooldown: 5.0,
    };

    let mut built_app = App::new();

    built_app
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes_override: Some(true),

            // Wasm builds will check for meta files (that don't exist) if this isn't set.
            // This causes errors and even panics in web builds on itch.
            // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
            meta_check: AssetMetaCheck::Never,
            ..Default::default()
        }))
        .add_plugins(UiMaterialPlugin::<ProgressBarMaterial>::default()) // Register the custom material
        .insert_resource(game_state)
        .insert_resource(config)
        .insert_resource(camera_state)
        .add_systems(Startup, systems::mana_system::setup_mana_progress_bar)
        .add_systems(Startup, setup)
        .add_systems(Startup, setup_fps_counter)
        .add_systems(Update, (rotate_camera, zoom_camera))
        .add_systems(Update, enemy_spawning_system)
        .add_systems(Update, enemy_movement_system)
        .add_systems(Update, enemy_disposal_system)
        .add_systems(Update, handle_play_enemy_animation_on_spawn)
        .add_systems(Update, ui_system)
        .add_systems(Update, (fps_counter_showhide, fps_text_update_system))
        .add_systems(Update, (systems::mana_system::increase_mana_as_time_pass, systems::mana_system::update_mana_progress))
        .add_systems(Update, systems::debug::draw_axes);

    built_app.add_plugins(FrameTimeDiagnosticsPlugin::default());

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
        (
            "sprites/cards_2d/PineTools.com_files/red_ace.png",
            CardType::Ace,
        ),
        (
            "sprites/cards_2d/PineTools.com_files/red_king.png",
            CardType::King,
        ),
        (
            "sprites/cards_2d/PineTools.com_files/red_queen.png",
            CardType::Queen,
        ),
        (
            "sprites/cards_2d/PineTools.com_files/red_jack.png",
            CardType::Jack,
        ),
    ];

    let all_textures: Vec<CardAsset> = card_files
        .iter()
        .map(|(path, card_type)| {
            let texture: Handle<Image> = asset_server.load(*path);
            CardAsset {
                texture,
                card_type: card_type.clone(),
            }
        })
        .collect();

    // Build the animation graph
    let mut graph = AnimationGraph::new();
    let animations = graph
        .add_clips(
            [GltfAssetLabel::Animation(0).from_asset("models/low_poly_naruto/scene.gltf#Scene0")]
                .into_iter()
                .map(|path| asset_server.load(path)),
            1.0,
            graph.root,
        )
        .collect();

    // Insert a resource with the current scene information
    let graph = graphs.add(graph);
    commands.insert_resource(EnemyAnimations {
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

fn setup_fps_counter(mut commands: Commands) {
    let root = commands
        .spawn((
            ui_components::FpsRoot,
            NodeBundle {
                background_color: BackgroundColor(Color::BLACK.with_alpha(0.5)),
                z_index: ZIndex::Global(i32::MAX),
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(1.0),
                    top: Val::Percent(1.0),
                    bottom: Val::Auto,
                    padding: UiRect::all(Val::Px(4.0)),
                    flex_wrap: FlexWrap::Wrap,
                    max_width: Val::Px(150.0),
                    max_height: Val::Px(200.0),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .id();

    let text_fps = commands
        .spawn((
            ui_components::FpsText,
            TextBundle {
                text: Text::from_sections([
                    TextSection {
                        value: "FPS: ".into(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    },
                    TextSection {
                        value: " N/A".into(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    },
                    TextSection {
                        value: " Draw Calls:".into(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    },
                    TextSection {
                        value: " N/A".into(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    },
                ]),
                ..Default::default()
            },
        ))
        .id();
    commands.entity(root).push_children(&[text_fps]);
}

