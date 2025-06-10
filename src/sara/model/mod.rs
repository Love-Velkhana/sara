pub mod player;
pub mod prelude;
pub mod tile;
use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(PhysicsLayer, Default)]
pub enum GameCollisionLayers {
    #[default]
    Default,
    Enviroment,
    Operation,
    Player,
    Hit,
}

pub struct ModelManager;
impl Plugin for ModelManager {
    fn build(&self, app: &mut App) {
        app.add_plugins(prelude::PlayerManager);
    }
}
