pub mod data;
pub mod model;
pub mod prelude;
use bevy::prelude::*;
use prelude::*;

pub struct Tile;
impl Tile {
    pub fn load_resource(
        mut command: Commands,
        asset_server: Res<AssetServer>,
        mut next_state: ResMut<NextState<super::TilesState>>,
    ) {
        command.insert_resource(LevelResource {
            _id: 0,
            texture_handle: asset_server.load(LevelResource::TEXTURE_ATLAS_PATH),
            layout_handle: asset_server.add(TextureAtlasLayout::from_grid(
                LevelResource::TILE_SIZE,
                LevelResource::TILE_COLS,
                LevelResource::TILE_ROWS,
                None,
                None,
            )),
            data_handle: asset_server.load(LevelResource::data_path(0)),
        });
        next_state.set(super::TilesState::Running);
    }
}
impl Plugin for Tile {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_asset::<data::LevelAsset>()
            .init_asset_loader::<data::LevelAssetLoader>()
            .add_systems(OnEnter(super::TilesState::Prepare), Self::load_resource);
    }
}
