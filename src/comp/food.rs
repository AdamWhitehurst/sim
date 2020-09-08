use amethyst::ecs::{Component, DenseVecStorage};
use std::default::Default;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Food;

impl Component for Food {
    type Storage = DenseVecStorage<Self>;
}

impl Default for Food {
    fn default() -> Self {
        Self
    }
}
