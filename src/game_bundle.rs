use systems::{
    StatsSystem,
};

use amethyst::{
    core::Bundle::{
        Result,
        SystemBundle
    },
    ecs::prelude::{
        DispatcherBuilder
    }
};

pub struct GameBundle;

impl <'a, 'b> SystemBundle<'a, 'b> for GameBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {

        builder.add(
            StatsSystem,
            "stats-system",
            &["input_system"]
        );

        Ok(())
    }
}