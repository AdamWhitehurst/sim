use amethyst::ecs::{Entities, Join, Read, ReadStorage, System, WriteStorage};
pub struct Sys;

impl<'a> System<'a> for Sys {
    // What data the system will be provided on each call to `run`
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Transform>,
        WriteStorage<'a, Collider>,
    );

    // Called every iteration of the game loop
    fn run(&mut self, (entities, transform_storage, mut collider_storage): Self::SystemData) {
        for (e, transform, collider) in (entities, transform_storage, collider_storage).join() {}
    }
}
