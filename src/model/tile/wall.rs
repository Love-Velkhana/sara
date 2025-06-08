use super::*;
use crate::data::level::*;
use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct FloorMarker;

#[derive(Bundle)]
pub struct Floor(
    Sprite,
    RigidBody,
    Collider,
    Transform,
    CollisionLayers,
    FloorMarker,
);
impl Floor {
    const FLOOR_TEXTURE_ATLAS_INDEX: usize = 36;
}
impl Tile for Floor {
    type Output = Self;
    fn new(translation: Vec3, level_resource: &Res<LevelResource>) -> Self::Output {
        Self(
            Sprite {
                image: level_resource.texture_handle.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: level_resource.layout_handle.clone(),
                    index: Self::FLOOR_TEXTURE_ATLAS_INDEX,
                }),
                ..Default::default()
            },
            RigidBody::Static,
            Collider::rectangle(
                LevelResource::TILE_COLLIFDER_SIZE.x,
                LevelResource::TILE_COLLIFDER_SIZE.y,
            ),
            Transform::from_translation(translation),
            CollisionLayers::new(GameCollisionLayers::Enviroment, GameCollisionLayers::Player),
            FloorMarker,
        )
    }
}
