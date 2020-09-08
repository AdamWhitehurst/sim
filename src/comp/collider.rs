use amethyst::ecs::{Component, DenseVecStorage, Entity};
use std::default::Default;
use std::vec::Vec;

#[derive(Debug)]
struct Collider {
    collisions: Vec<Entity>,
}

impl Default for Collider {
    fn default() -> Self {
        Self {
            collisions: Vec::new(),
        }
    }
}

impl Component for Collider {
    type Storage = DenseVecStorage<Self>;
}
