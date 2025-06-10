pub mod hitbox;
pub mod passbox;
pub mod prelude;
pub mod wall;
use super::GameCollisionLayers;
use crate::data::level::*;
use bevy::prelude::*;

pub trait Tile {
    type Output;
    fn new(translation: Vec3, level_resource: &Res<LevelResource>) -> Self::Output;
}
