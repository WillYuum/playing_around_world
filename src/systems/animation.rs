use std::time::Duration;

use bevy::prelude::*;

use crate::resources::{animations::EnemyAnimations, asset_resources::FoxResource};

pub fn handle_play_enemy_animation_on_spawn(
    mut commands: Commands,
    animations: Res<FoxResource>,
    mut enemies: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
) {
    for (entity, mut player) in &mut enemies {
        let mut transitions = AnimationTransitions::new();
        transitions
            .play(&mut player, animations.animations.node_indices[0], Duration::ZERO)
            .repeat();

        commands
            .entity(entity)
            .insert(animations.animations.graph.clone())
            .insert(transitions);
    }
}
