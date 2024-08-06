use std::time::Duration;

use bevy::prelude::*;

use crate::resources::{asset_resources::FoxResource};

pub fn handle_play_enemy_animation_on_spawn(
    mut commands: Commands,
    foxes_resources: Res<FoxResource>,
    mut anim_players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
) {
    for (entity, mut player) in &mut anim_players {
        let mut transitions = AnimationTransitions::new();
        let fox_animation = &foxes_resources.animations;

        transitions
            .play(&mut player, fox_animation.node_indices[0], Duration::ZERO)
            .repeat();

        commands
            .entity(entity)
            .insert(fox_animation.graph.clone())
            .insert(transitions);
    }
}
