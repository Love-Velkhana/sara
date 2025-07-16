use bevy::asset::{AssetLoader, LoadContext, io::Reader};
use bevy::prelude::*;
use bincode::{Decode, Encode, config};
use strum::EnumIter;
use thiserror::Error;

#[derive(Resource)]
pub struct LevelDynamicResource(pub Handle<LevelAsset>);
impl LevelDynamicResource {
    const PATH_BASE: &'static str = "data/level";
    const SUFFIX: &'static str = ".sbc";

    fn asset_path(id: usize) -> String {
        Self::PATH_BASE.to_string() + &id.to_string() + Self::SUFFIX
    }

    pub fn data_path(id: usize) -> String {
        String::from("assets/") + &Self::asset_path(id)
    }

    pub fn new(id: usize, asset_server: &Res<AssetServer>) -> Self {
        let data_handle = asset_server.load(Self::asset_path(id));
        Self(data_handle)
    }
}

#[derive(Resource)]
pub struct LevelStaticResource {
    pub texture_handle: Handle<Image>,
    pub layout_handle: Handle<TextureAtlasLayout>,
}
impl LevelStaticResource {
    pub const TEXTURE_ATLAS_PATH: &'static str = "images/building/tiles.png";
    pub const TILE_SIZE: UVec2 = UVec2::new(32, 32);
    pub const TILE_ROWS: u32 = 16;
    pub const TILE_COLS: u32 = 16;

    pub fn new(asset_server: &Res<AssetServer>) -> Self {
        Self {
            texture_handle: asset_server.load(Self::TEXTURE_ATLAS_PATH),
            layout_handle: asset_server.add(TextureAtlasLayout::from_grid(
                Self::TILE_SIZE,
                Self::TILE_COLS,
                Self::TILE_ROWS,
                None,
                None,
            )),
        }
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
        &[LevelDynamicResource::SUFFIX]
    }
}
