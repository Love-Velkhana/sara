use bevy::prelude::*;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
pub enum PlayerAsepriteType {
    Jump,
    Walk,
    Idle,
    Fall,
}
impl PlayerAsepriteType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Jump => "jump",
            Self::Walk => "walk",
            Self::Idle => "idle",
            Self::Fall => "fall",
        }
    }
    pub fn frame_count(&self) -> usize {
        match self {
            Self::Jump => 3,
            Self::Walk => 3,
            Self::Idle => 1,
            Self::Fall => 1,
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
        let param: [(PlayerAsepriteType, TextureAtlasLayout); 4] = [
            (
                PlayerAsepriteType::Jump,
                TextureAtlasLayout::from_grid(Self::TEXTURE_SIZE, 3, 1, None, None),
            ),
            (
                PlayerAsepriteType::Walk,
                TextureAtlasLayout::from_grid(Self::TEXTURE_SIZE, 3, 1, None, None),
            ),
            (
                PlayerAsepriteType::Idle,
                TextureAtlasLayout::from_grid(Self::TEXTURE_SIZE, 1, 1, None, None),
            ),
            (
                PlayerAsepriteType::Fall,
                TextureAtlasLayout::from_grid(Self::TEXTURE_SIZE, 1, 1, None, None),
            ),
        ];

        let mut texture_atlas_handles = HashMap::new();
        for aseprite_param in param {
            let name = aseprite_param.0.name();
            texture_atlas_handles.insert(
                aseprite_param.0,
                (
                    asset_server.load(Self::texture_path(name)),
                    asset_server.add(aseprite_param.1),
                ),
            );
        }
        Self {
            texture_atlas_handles,
        }
    }
}
