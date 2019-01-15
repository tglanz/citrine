use crate::{
    config,
    asset_loaders,
};

use amethyst::{
    prelude::*,

    utils::{
        application_root_dir,
    },

    core::{
        transform::{
            Transform,
        }
    },

    renderer::{
        Camera,
        Projection,
    },

    assets::{
        Loader,
        ProgressCounter,
        Progress,
        AssetStorage,
    },
};

fn setup_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    
    let projection = {
        let projection_config = &world.read_resource::<config::Config>().projection;
        Projection::orthographic(
            projection_config.top,
            projection_config.bottom,
            projection_config.left,
            projection_config.right
        )
    };

    world
        .create_entity()
        .with(Camera::from(projection))
        .with(transform)
        .build();
}

fn load_tiled_maps(world: &mut World, progress_counter: &mut ProgressCounter) -> asset_loaders::tiled_map::TiledMapHandle {
    let storage = &world.read_resource();
    let loader = &world.read_resource::<Loader>();

    loader.load(
        "tiled-desert/desert.tmx",
        asset_loaders::TiledMapFormat,
        (format!("{}/assets", application_root_dir()), ),
        progress_counter,
        &storage
    )
}

pub struct InitialState {
    /// Tracks loaded assets.
    progress_counter: ProgressCounter,

    /// Handle to the energy blast.
    tiled_map_handle: Option<asset_loaders::tiled_map::TiledMapHandle>,
}

impl InitialState {
    pub fn new() -> Self {
        InitialState {
            progress_counter: ProgressCounter::new(),
            tiled_map_handle: None
        }
    }
}

impl SimpleState for InitialState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;
        setup_camera(world);
        self.tiled_map_handle = Some(load_tiled_maps(world, &mut self.progress_counter))
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if self.progress_counter.is_complete() {
            // Trans::Quit
            Trans::None
        } else {
            Trans::None
        }
    }
}