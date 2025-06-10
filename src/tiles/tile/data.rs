use bevy::asset::{AssetLoader, LoadContext, io::Reader};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub enum TileType {
    Space,
    Wall,
    Pass,
    Hit,
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
    IOError(#[from] std::io::Error),
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
        &[".m"]
    }
}
