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
        .insert_resource(NarrowPhaseConfig {
            contact_tolerance: 0.0,
            default_speculative_margin: 0.0,
            ..Default::default()
        })
        .add_plugins(utils::aseprite::AsepritePlugin)
        .add_plugins(data::DataManager)
        .add_plugins(model::ModelManager)
        .add_plugins(sound::SoundManager)
        .run();
}
