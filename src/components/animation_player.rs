use bevy::prelude::*;

#[derive(Component)]
pub struct CustomAnimator {
    pub animation: Handle<AnimationClip>,
}