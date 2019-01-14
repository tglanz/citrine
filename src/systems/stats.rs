use amethyst::{
    core::timing::Time,
    input::InputHandler,
    ecs::prelude::{Join, Read}
};

struct StatsSystem;

impl<'s> System for StatsSystem {
    type SystemData = (
        Read<'s, Time>,
    );

    fn run(&mut self, (time)): Self:SystemData {
        let fps = 1.0 / time.delta_seconds();
        println!("Fps: {:?}", fps);
    }
}