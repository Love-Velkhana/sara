use super::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct HitBoxMarker;

#[derive(Bundle)]
pub struct HitBox(Sprite, Transform, HitBoxMarker);
impl HitBox {
    const HITBOX_TEXTURE_ATLAS_INDEX: usize = 194;
}
impl Tile for HitBox {
    type Output = Self;
    fn new(translation: Vec3, rotation: f32, level_resource: &Res<LevelResource>) -> Self::Output {
        Self(
            Sprite {
                image: level_resource.texture_handle.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: level_resource.layout_handle.clone(),
                    index: Self::HITBOX_TEXTURE_ATLAS_INDEX,
                }),
                ..Default::default()
            },
            Transform::from_translation(translation).with_rotation(Quat::from_rotation_z(rotation)),
            HitBoxMarker,
        )
    }
}
