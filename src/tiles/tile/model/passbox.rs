use super::*;
use crate::utils::aseprite::*;
use bevy::prelude::*;

#[derive(Component)]
struct PassBoxMarker;

#[derive(Bundle)]
pub struct PassBox(Aseprite, Transform, PassBoxMarker);
impl PassBox {
    const FRAME_START_INDEX: usize = TileType::Pass.texture_atlas_index();
    const FRAME_LAST_INDEX: usize = 4;
}
impl Tile for PassBox {
    type Output = Self;
    fn new(translation: Vec3, rotation: f32, level_resource: &Res<LevelResource>) -> Self::Output {
        Self(
            Aseprite::new(
                Sprite {
                    image: level_resource.texture_handle.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: level_resource.layout_handle.clone(),
                        index: Self::FRAME_START_INDEX,
                    }),
                    ..Default::default()
                },
                AsepriteIndices::new(Self::FRAME_START_INDEX, Self::FRAME_LAST_INDEX),
                AsepritePlaying(true),
                AsepriteTimer(Timer::from_seconds(0.15, TimerMode::Repeating)),
            ),
            Transform::from_translation(translation).with_rotation(Quat::from_rotation_z(rotation)),
            PassBoxMarker,
        )
    }
}
