use super::*;
use crate::data::level::*;
use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct HitBoxMarker;

#[derive(Bundle)]
pub struct HitBox(
    Sprite,
    Sensor,
    Collider,
    Transform,
    CollisionLayers,
    HitBoxMarker,
);
impl HitBox {
    const HITBOX_TEXTURE_ATLAS_INDEX: usize = 194;
}
impl Tile for HitBox {
    type Output = Self;
    fn new(translation: Vec3, level_resource: &Res<LevelResource>) -> Self::Output {
        Self(
            Sprite {
                image: level_resource.texture_handle.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: level_resource.layout_handle.clone(),
                    index: Self::HITBOX_TEXTURE_ATLAS_INDEX,
                }),
                ..Default::default()
            },
            Sensor,
            Collider::rectangle(
                LevelResource::TILE_COLLIFDER_SIZE.x,
                LevelResource::TILE_COLLIFDER_SIZE.y,
            ),
            Transform::from_translation(translation),
            CollisionLayers::new(GameCollisionLayers::Hit, GameCollisionLayers::Player),
            HitBoxMarker,
        )
    }
}
