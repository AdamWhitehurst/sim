use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Creature;

impl Component for Creature {
    type Storage = DenseVecStorage<Creature>;
}
