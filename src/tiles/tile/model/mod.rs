pub mod hitbox;
pub mod passbox;
pub mod prelude;
pub mod wall;
use crate::tile::data::*;
use bevy::prelude::*;

pub trait Tile {
    type Output;
    fn new(
        translation: Vec3,
        rotation: f32,
        level_resource: &Res<LevelStaticResource>,
    ) -> Self::Output;
}
