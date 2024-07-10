use bevy::prelude::*;

#[derive(Component)]
pub struct AnimationPlayer {
    pub animation: Handle<AnimationClip>,
}