pub mod level;
pub mod player;
pub mod prelude;
use crate::scene::GameScene;
use bevy::prelude::*;
use prelude::*;

pub struct DataManager;
impl DataManager {
    fn load_running_resource(
        mut command: Commands,
        mut level_init: EventReader<LevelInit>,
        asset_server: Res<AssetServer>,
    ) {
        let texture_handle = asset_server.load(LevelResource::TEXTURE_ATLAS_PATH);
        let layout_handle = asset_server.add(TextureAtlasLayout::from_grid(
            LevelResource::TILE_SIZE,
            LevelResource::TILE_COLS,
            LevelResource::TILE_ROWS,
            None,
            None,
        ));

        let id = level_init.read().last().unwrap().0;
        command.insert_resource(LevelResource {
            id,
            texture_handle,
            layout_handle: layout_handle.clone(),
            data_handle: asset_server.load(LevelResource::data_path(id)),
        });
        command.insert_resource(PlayerResource::new(&asset_server));
    }
}
impl Plugin for DataManager {
    fn build(&self, app: &mut App) {
        app.init_asset::<level::LevelAsset>()
            .init_asset_loader::<level::LevelAssetLoader>()
            .add_event::<LevelInit>()
            .add_event::<LevelPass>()
            .add_systems(OnEnter(GameScene::InGame), Self::load_running_resource);
    }
}
