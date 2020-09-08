use amethyst::ecs::{Component, DenseVecStorage};
use std::default::Default;

/// Represents a sentient lifeform.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Creature {
    /// How fast a creature moves
    pub movespeed: f32,
}

// Implement default settings for a creature.
impl Default for Creature {
    fn default() -> Self {
        Self { movespeed: 100.0 }
    }
}

impl Component for Creature {
    type Storage = DenseVecStorage<Creature>;
}
