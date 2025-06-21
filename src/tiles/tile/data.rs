use bevy::asset::{AssetLoader, LoadContext, io::Reader};
use bevy::prelude::*;
use bincode::{Decode, Encode, config};
use strum::EnumIter;
use thiserror::Error;

#[derive(Resource)]
pub struct LevelResource {
    pub id: usize,
    pub texture_handle: Handle<Image>,
    pub layout_handle: Handle<TextureAtlasLayout>,
    pub data_handle: Handle<LevelAsset>,
}
impl LevelResource {
    const PATH_BASE: &'static str = "data/level";
    const SUFFIX: &'static str = ".sbc";
    pub const TEXTURE_ATLAS_PATH: &'static str = "images/building/tiles.png";
    pub const TILE_SIZE: UVec2 = UVec2::new(32, 32);
    pub const TILE_ROWS: u32 = 16;
    pub const TILE_COLS: u32 = 16;

    pub fn data_path(id: usize) -> String {
        Self::PATH_BASE.to_string() + &id.to_string() + Self::SUFFIX
    }
}

#[derive(Debug, Encode, Decode, Clone, Copy, EnumIter, PartialEq, Eq)]
pub enum TileType {
    Wall,
    Pass,
    Trap,
}
impl TileType {
    pub const fn texture_atlas_index(&self) -> usize {
        match self {
            Self::Wall => 36,
            Self::Trap => 194,
            Self::Pass => 0,
        }
    }
}

#[derive(Debug, Encode, Decode, Clone, Copy)]
pub struct TileDescriptor {
    pub tile_pos: (f32, f32),
    pub tile_typ: TileType,
    pub rotation: f32,
}

#[derive(Asset, TypePath, Debug, Encode, Decode)]
pub struct LevelAsset {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<TileDescriptor>,
    pub entry: (f32, f32),
    pub next: Option<usize>,
}

#[derive(Error, Debug)]
pub enum LevelAssetError {
    #[error("Could not load asset: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Could not decode sbc: {0}")]
    DecodeError(#[from] bincode::error::DecodeError),
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
        let (asset, _) = bincode::decode_from_slice(&buf, config::standard())?;
        Ok(asset)
    }

    fn extensions(&self) -> &[&str] {
        &[LevelResource::SUFFIX]
    }
}
