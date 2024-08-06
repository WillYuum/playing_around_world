use bevy::prelude::*;

use super::animations::ModelAnimations;

#[derive(Resource)]
pub struct FoxResource{
    pub model: Handle<Scene>,
    pub animations: ModelAnimations,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum CardType {
    Ace,
    King,
    Queen,
    Jack,
}

#[derive(Resource)]
pub struct CardAsset {
    pub card_type: CardType,
    pub texture: Handle<Image>,
}
