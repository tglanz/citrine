extern crate serde_derive;
extern crate serde;

extern crate amethyst;
extern crate tiled;

mod config;
mod states;
mod asset_loaders;

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
    assets::{
        Processor
    },
    utils::{
        application_root_dir
    },
};


fn main() -> amethyst::Result<()> {
    let mut logger_config: amethyst::LoggerConfig = Default::default();
    logger_config.level_filter = amethyst::LogLevelFilter::Debug;
    amethyst::start_logger(logger_config);

    let app_path = application_root_dir();

    let display_config_path = format!("{}/resources/display.ron", app_path);
    let config_path = format!("{}/resources/config.ron", app_path);
    let assets_dir = format!("{}/assets", app_path);

    let config = config::Config::load(&config_path);

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_basic_renderer(display_config_path, DrawFlat::<PosTex>::new(), true)?
        .with(Processor::<asset_loaders::tiled_map::TiledMap>::new(), "", &[]);

    let mut application = Application::build(assets_dir, states::InitialState::new())?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            120,
        )
        .with_resource(config)
        .build(game_data)?;

    application.run();

    Ok(())
}
