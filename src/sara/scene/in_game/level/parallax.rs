use super::{Level, LevelState, ccamera::*};
use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
struct ParallaxMarker;
type ParallaxQuery<'a, 'b> =
    Single<'a, &'b mut LinearVelocity, (With<ParallaxMarker>, Without<LevelCameraMarker>)>;

pub struct Parallax;
impl Parallax {
    const BACKGROUND_PATH: &'static str = "images/building/background_mountain.png";
    const BACKGROUND_SCALE: Vec3 = Vec3::new(2.2, 2.2, 1.0);
    const MOVE_SPEEP_FACTOR: f32 = 0.9;
    fn init(mut command: Commands, asset_server: Res<AssetServer>, window: Single<&Window>) {
        command.spawn((
            Sprite::from_image(asset_server.load(Self::BACKGROUND_PATH)),
            ParallaxMarker,
            Transform {
                translation: Vec3::new(window.width() / 2.0, window.resolution.height() / 2.0, 0.0),
                scale: Self::BACKGROUND_SCALE,
                ..Default::default()
            },
            RigidBody::Kinematic,
            LinearVelocity::ZERO,
            StateScoped(LevelState::Running),
        ));
    }

    fn update(
        mut parallax_linear_velocity_query: ParallaxQuery,
        camera_linear_velocity_query: LevelCameraLinearVelocityQuery,
    ) {
        parallax_linear_velocity_query.0 = camera_linear_velocity_query.0 * Self::MOVE_SPEEP_FACTOR;
    }
}
impl Plugin for Parallax {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(LevelState::Loading), Self::init)
            .add_systems(Update, Self::update.run_if(Level::is_runnable()));
    }
}
