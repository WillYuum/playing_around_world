use bevy::prelude::*;

#[derive(Resource)]
pub struct EnemyAnimations {
    pub animations: Vec<AnimationNodeIndex>,
    #[allow(dead_code)]
    pub graph: Handle<AnimationGraph>,
}
