use super::*;
use crate::data::level::*;
use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct FloorMarker;

#[derive(Bundle)]
pub struct Floor(
    Sprite,
    Mass,
    RigidBody,
    Collider,
    Transform,
    Restitution,
    CollisionLayers,
    FloorMarker,
);
impl Tile for Floor {
    type Output = Self;
    fn new(translation: Vec3, rotation: f32, level_resource: &Res<LevelResource>) -> Self::Output {
        Self(
            Sprite {
                image: level_resource.texture_handle.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: level_resource.layout_handle.clone(),
                    index: TileType::Wall.texture_atlas_index(),
                }),
                ..Default::default()
            },
            Mass(1000.0),
            RigidBody::Static,
            Collider::rectangle(
                LevelResource::TILE_COLLIFDER_SIZE.x,
                LevelResource::TILE_COLLIFDER_SIZE.y,
            ),
            Transform::from_translation(translation).with_rotation(Quat::from_rotation_z(rotation)),
            Restitution::ZERO,
            CollisionLayers::new(GameCollisionLayers::Enviroment, GameCollisionLayers::Player),
            FloorMarker,
        )
    }
}
