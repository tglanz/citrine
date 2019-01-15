use std::sync::Arc;
use std::path::Path;

use amethyst::{
    assets::{
        Directory,
        Asset,
        Handle,
        Format,
        Result as AssetResult,
        ProcessingState,
        ResultExt,
        Source,
        FormatValue,
    },

    ecs::{
        VecStorage,
    },
};

use tiled;

#[derive(Clone)]
pub struct TiledMapFormat;

pub struct TiledMap {
    map: tiled::Map
}

pub type TiledMapHandle = Handle<TiledMap>;

impl Asset for TiledMap {
    const NAME: &'static str = "citrine::TiledMap";
    type Data = Self;
    type HandleStorage = VecStorage<TiledMapHandle>;
}

impl From<TiledMap> for AssetResult<ProcessingState<TiledMap>> {
    fn from(tiled_map: TiledMap) -> AssetResult<ProcessingState<TiledMap>> {
        Ok(ProcessingState::Loaded(tiled_map))
    }
}

impl Format<TiledMap> for TiledMapFormat {

    const NAME: &'static str = "citrine::TiledMapFormat";
    type Options = (String,);

    fn import(
        &self,
        name: String,
        source: Arc<dyn Source>,
        options: Self::Options,
        _create_reload: bool,
    ) -> AssetResult<FormatValue<TiledMap>> {

        let path = format!("{}/{}", &options.0, &name);

        tiled::parse_file(&Path::new(&path))
            .map(|map| TiledMap { map })
            .map(FormatValue::data)
            .chain_err(|| "Failed to parse a tiled map")
    }
}
