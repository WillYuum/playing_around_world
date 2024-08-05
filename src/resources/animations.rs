use bevy::prelude::*;

#[derive(Resource)]
pub struct EnemyAnimations {
    pub animations: Vec<AnimationNodeIndex>,
    #[allow(dead_code)]
    pub graph: Handle<AnimationGraph>,
}


#[derive(Resource)]
pub struct ModelAnimations {
    pub node_indices: Vec<AnimationNodeIndex>,
    pub graph: Handle<AnimationGraph>,
}
