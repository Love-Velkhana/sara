use bevy::prelude::*;
use std::collections::HashMap;
use strum::{EnumIter, IntoEnumIterator};

#[derive(PartialEq, Eq, Hash, Clone, Copy, EnumIter)]
pub enum PlayerAsepriteType {
    Jump,
    Walk,
    Idle,
    Fall,
}
impl PlayerAsepriteType {
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Jump => "jump",
            Self::Walk => "walk",
            Self::Idle => "idle",
            Self::Fall => "fall",
        }
    }

    pub const fn frame_count(&self) -> usize {
        match self {
            Self::Jump => 3,
            Self::Walk => 3,
            Self::Idle => 1,
            Self::Fall => 1,
        }
    }

    pub fn layout(&self) -> TextureAtlasLayout {
        match self {
            Self::Jump => {
                TextureAtlasLayout::from_grid(PlayerResource::TEXTURE_SIZE, 3, 1, None, None)
            }

            Self::Walk => {
                TextureAtlasLayout::from_grid(PlayerResource::TEXTURE_SIZE, 3, 1, None, None)
            }

            Self::Idle => {
                TextureAtlasLayout::from_grid(PlayerResource::TEXTURE_SIZE, 1, 1, None, None)
            }

            Self::Fall => {
                TextureAtlasLayout::from_grid(PlayerResource::TEXTURE_SIZE, 1, 1, None, None)
            }
        }
    }
}

#[derive(Resource)]
pub struct PlayerResource {
    pub texture_atlas_handles:
        HashMap<PlayerAsepriteType, (Handle<Image>, Handle<TextureAtlasLayout>)>,
}
impl PlayerResource {
    pub const TEXTURE_SIZE: UVec2 = UVec2::new(48, 48);
    pub const TEXTURE_BASE_PATH: &'static str = "images/sara/";
    pub const SUFFIX: &'static str = ".png";

    fn texture_path(texture_name: &str) -> String {
        Self::TEXTURE_BASE_PATH.to_string() + texture_name + Self::SUFFIX
    }

    pub fn new(asset_server: &Res<AssetServer>) -> Self {
        let mut texture_atlas_handles = HashMap::new();
        for aseprite_type in PlayerAsepriteType::iter() {
            let name = aseprite_type.name();
            texture_atlas_handles.insert(
                aseprite_type,
                (
                    asset_server.load(Self::texture_path(name)),
                    asset_server.add(aseprite_type.layout()),
                ),
            );
        }
        Self {
            texture_atlas_handles,
        }
    }
}
