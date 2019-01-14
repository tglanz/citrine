#[macro_use]
extern crate serde_derive;
extern crate amethyst;

mod config;
mod states;

use std::time::{Duration};

use amethyst::{
    prelude::*,
    core::{
        transform::{
            TransformBundle,
        },
        frame_limiter::{
            FrameRateLimitStrategy,
        }
    },
    renderer::{
        DrawFlat,
        PosTex
    },

    utils::{
        application_root_dir
    },
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_path = application_root_dir();

    let display_config_path = format!("{}/resources/display.ron", app_path);
    let config_path = format!("{}/resources/config.ron", app_path);
    let assets_dir = format!("{}/assets", app_path);

    let config = config::Config::load(&config_path);

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_basic_renderer(display_config_path, DrawFlat::<PosTex>::new(), true)?;

    let mut application = Application::build(assets_dir, states::InitialState)?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            120,
        )
        .with_resource(config)
        .build(game_data)?;

    application.run();

    Ok(())
}
