pub mod data;
pub mod model;
pub mod prelude;
use crate::AppState;
use bevy::prelude::*;

pub struct Tile;
impl Plugin for Tile {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_asset::<data::LevelAsset>()
            .init_asset_loader::<data::LevelAssetLoader>()
            .add_systems(
                OnEnter(AppState::Prepare),
                |mut command: Commands,
                 asset_server: Res<AssetServer>,
                 mut next_state: ResMut<NextState<AppState>>| {
                    command.insert_resource(data::LevelStaticResource::new(&asset_server));
                    next_state.set(AppState::Running);
                },
            );
    }
}
