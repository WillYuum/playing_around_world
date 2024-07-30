use bevy::prelude::*;

#[derive(Resource)]
pub struct NarutoResource {
    pub model: Handle<Scene>,
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
