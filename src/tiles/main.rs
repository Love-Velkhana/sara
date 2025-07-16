mod tile;
mod ui;
mod utils;
use bevy::prelude::*;

#[derive(States, Copy, Clone, Eq, PartialEq, Debug, Default, Hash)]
enum AppState {
    #[default]
    Prepare,
    Running,
}

fn main() {
    App::new()
        .add_plugins(ui::UIPlugin)
        .add_plugins(tile::Tile)
        .add_plugins(utils::aseprite::AsepritePlugin)
        .add_plugins(utils::editline::EditLinePlugin)
        .init_state::<AppState>()
        .run();
}
