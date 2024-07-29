use std::time::Duration;

use bevy::prelude::*;

use crate::resources::animations::EnemyAnimations;

pub fn handle_play_enemy_animation_on_spawn(
    mut commands: Commands,
    animations: Res<EnemyAnimations>,
    mut enemies: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
) {
    for (entity, mut player) in &mut enemies {
        let mut transitions = AnimationTransitions::new();

        
        transitions
            .play(&mut player, animations.animations[0], Duration::ZERO)
            .repeat();

        commands
            .entity(entity)
            .insert(animations.graph.clone())
            .insert(transitions);
    }

}