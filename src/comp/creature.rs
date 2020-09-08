use amethyst::ecs::{Component, DenseVecStorage};
use rand::prelude::*;
use std::default::Default;

/// Represents a sentient lifeform.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Creature {
    /// How fast a creature moves
    pub movespeed: f32,
    pub size: u32,
}

// Implement default settings for a creature.
impl Default for Creature {
    fn default() -> Self {
        Self {
            movespeed: 100.0,
            size: 1,
        }
    }
}

impl Creature {
    pub fn new() -> Self {
        Self {
            size: thread_rng().gen_range(1, 10),
            ..Self::default()
        }
    }
}

impl Component for Creature {
    type Storage = DenseVecStorage<Creature>;
}
