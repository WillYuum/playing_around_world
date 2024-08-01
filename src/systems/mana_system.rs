use bevy::{prelude::*};

use crate::{
    components::progress_bar::{ProgressBar, ProgressBarBundle, ProgressBarMaterial},
    resources::game_state::GameState
};


pub fn setup_mana_progress_bar(mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ProgressBarMaterial>>,
){
    asset_server.watching_for_changes();


    let fill_color = Color::srgb(216.0 / 255.0, 48.0 / 255.0, 209.0 / 255.0).to_linear();

    let progress_bar = ProgressBar::new(
        fill_color, 
        LinearRgba::rgb(0.0, 0.0, 0.0).with_alpha(0.5), 
    );

  commands.spawn(ProgressBarBundle {
    progressbar: progress_bar,
    material_node_bundle: MaterialNodeBundle {
        style: Style {
            position_type: bevy::ui::PositionType::Absolute,
            bottom: Val::Px(0.0),
            left: Val::Percent(0.0),
            right: Val::Percent(0.0),
            width: Val::Percent(100.0),
            height: Val::Px(35.0),
            ..Default::default()
        },
        material: materials.add(ProgressBarMaterial::default()),
        ..Default::default()
    },
});

}


pub fn increase_mana_as_time_pass(
    time: Res<Time>,
    mut game_state: ResMut<GameState>
){
        game_state.curr_mana += time.delta_seconds();

        if game_state.curr_mana >= game_state.mana_max {
            game_state.curr_mana = game_state.mana_max;
        }

}

pub fn update_mana_progress(
    mut query: Query<(&mut ProgressBar, &Handle<ProgressBarMaterial>)>,
    mut materials: ResMut<Assets<ProgressBarMaterial>>,
    game_state: Res<GameState>,
) {
    for (mut bar, handle) in query.iter_mut() {
        if let Some(material) = materials.get_mut(handle) {
            bar.set_progress(game_state.curr_mana / game_state.mana_max); // Mutate the ProgressBar component
            material.update(&bar); // Pass an immutable reference to update the material
        }
    }
}