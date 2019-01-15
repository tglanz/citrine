use crate::{
    config
};

use amethyst::{
    prelude::*,

    core::{
        transform::{
            Transform,
        }
    },

    renderer::{
        Camera,
        Projection,
    }
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

pub struct InitialState;

impl SimpleState for InitialState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;
        setup_camera(world);
    }
}