mod data;
mod model;
mod scene;
mod sound;
mod utils;
use avian2d::prelude::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(scene::ScenePlugins)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(utils::aseprite::AsepritePlugin)
        .add_plugins(data::DataManager)
        .add_plugins(model::ModelManager)
        .add_plugins(sound::SoundManager)
        .run();
}
