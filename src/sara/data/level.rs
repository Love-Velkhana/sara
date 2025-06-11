use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Event)]
pub struct LevelInit(pub usize);

#[derive(Event)]
pub struct LevelPass(pub bool);

#[derive(Resource)]
pub struct LevelResource {
    pub id: usize,
    pub texture_handle: Handle<Image>,
    pub layout_handle: Handle<TextureAtlasLayout>,
    pub data_handle: Handle<LevelAsset>,
}
impl LevelResource {
    const PATH_BASE: &'static str = "data/level";
    const SUFFIX: &'static str = ".m";
    pub const TEXTURE_ATLAS_PATH: &'static str = "images/building/tiles.png";
    pub const TILE_SIZE: UVec2 = UVec2::new(32, 32);
    pub const TILE_COLLIFDER_SIZE: Vec2 = Vec2::new(32.0, 32.0);
    pub const TILE_ROWS: u32 = 16;
    pub const TILE_COLS: u32 = 16;

    pub fn data_path(id: usize) -> String {
        Self::PATH_BASE.to_string() + &id.to_string() + Self::SUFFIX
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TileType {
    None,
    Wall,
    Pass,
    Trap,
}

#[derive(Asset, TypePath, Debug, Serialize, Deserialize)]
pub struct LevelAsset {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<TileType>,
    pub entry: Vec2,
    pub next: Option<usize>,
}

#[derive(Error, Debug)]
pub enum LevelAssetError {
    #[error("Could not load asset: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Could not parse json: {0}")]
    SerdeError(#[from] serde_json::Error),
}

#[derive(Default)]
pub struct LevelAssetLoader;
impl AssetLoader for LevelAssetLoader {
    type Asset = LevelAsset;
    type Error = LevelAssetError;
    type Settings = ();

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf).await?;
        Ok(serde_json::from_slice(&buf)?)
    }

    fn extensions(&self) -> &[&str] {
        &[LevelResource::SUFFIX]
    }
}
