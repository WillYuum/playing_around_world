use bevy::prelude::*;

#[derive(Resource)]
pub struct ModelAnimations {
    pub node_indices: Vec<AnimationNodeIndex>,
    pub graph: Handle<AnimationGraph>,
}
