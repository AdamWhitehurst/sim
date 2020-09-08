use crate::comp::creature::*;
use amethyst::core::math::Vector3;
use amethyst::core::{timing::Time, Transform};
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use rand::prelude::*;
// Store data that is local to only this system on this struct
pub struct Sys;

impl<'a> System<'a> for Sys {
    // What data the system will be provided on each call to `run`
    type SystemData = (
        // Get read-only access to the Time Resource
        Read<'a, Time>,
        // Get read-only access to Creature Components Storage
        ReadStorage<'a, Creature>,
        // get write access to Transform Components Storage
        WriteStorage<'a, Transform>,
    );

    // Called every iteration of the game loop
    fn run(&mut self, mut data: Self::SystemData) {
        let mut rng = thread_rng();
        // Iterate over all entities with both `Creature` and `Transform` components
        for (creature, transform) in (&data.1, &mut data.2).join() {
            let dt = data.0.delta_time();
            // Random movement
            let dx = rng.gen_range(-1.0, 1.0) * dt.as_secs_f32();
            let dy = rng.gen_range(-1.0, 1.0) * dt.as_secs_f32();

            transform.append_translation(Vector3::new(dx, dy, 0.));
        }
    }
}
