pub mod data;
pub mod ui;
use bevy::prelude::*;

pub struct Tile;
impl Tile {}
impl Plugin for Tile {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_asset::<data::LevelAsset>()
            .init_asset_loader::<data::LevelAssetLoader>()
            .add_systems(Startup, ui::init)
            .add_systems(Update, ui::update);
    }
}
