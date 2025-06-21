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
impl Tile for HitBox {
    type Output = Self;
    fn new(translation: Vec3, rotation: f32, level_resource: &Res<LevelResource>) -> Self::Output {
        Self(
            Sprite {
                image: level_resource.texture_handle.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: level_resource.layout_handle.clone(),
                    index: TileType::Trap.texture_atlas_index(),
                }),
                ..Default::default()
            },
            Sensor,
            Collider::rectangle(
                LevelResource::TILE_COLLIFDER_SIZE.x,
                LevelResource::TILE_COLLIFDER_SIZE.y,
            ),
            Transform::from_translation(translation).with_rotation(Quat::from_rotation_z(rotation)),
            CollisionLayers::new(GameCollisionLayers::Hit, GameCollisionLayers::Player),
            HitBoxMarker,
        )
    }
}
