pub mod player_manager;
pub mod prelude;
use super::GameCollisionLayers;
use crate::utils::prelude::*;
use crate::{data::prelude::*, scene::GameScene};
use avian2d::prelude::*;
use bevy::ecs::query::QueryData;
use bevy::prelude::*;

#[derive(SubStates, Clone, Copy, Default, Debug, Hash, PartialEq, Eq)]
#[source(GameScene = GameScene::InGame)]
#[states(scoped_entities)]
pub enum PlayerState {
    #[default]
    Prepare,
    Loading,
    Running,
}

#[derive(Component)]
pub struct PlayerMarker;
pub type PlayerLinearVelocityQuery<'a, 'b> = Single<'a, &'b mut LinearVelocity, With<PlayerMarker>>;

#[derive(Component)]
struct HP(usize);
impl HP {
    const MAX_HP: usize = 2;
}
impl Default for HP {
    fn default() -> Self {
        Self(Self::MAX_HP)
    }
}
type PlayerHPQuery<'a, 'b> = Single<'a, &'b mut HP, With<PlayerMarker>>;

#[derive(SubStates, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[source(PlayerState = PlayerState::Running)]
enum PlayerRunningState {
    #[default]
    Fall,
    Jump,
    Walk,
    Idle,
}

#[derive(QueryData)]
#[query_data(mutable)]
pub struct PlayerAsepriteQueryData<'a> {
    sprite: &'a mut Sprite,
    indices: &'a mut AsepriteIndices,
    playing: &'a mut AsepritePlaying,
    timer: &'a mut AsepriteTimer,
}
type PlayerAsepriteQuery<'a, 'b> = Single<'a, PlayerAsepriteQueryData<'b>, With<PlayerMarker>>;

#[derive(Bundle)]
pub struct Player(
    Camera2d,
    Aseprite,
    HP,
    Transform,
    RigidBody,
    Collider,
    CollisionEventsEnabled,
    LockedAxes,
    Friction,
    Restitution,
    ShapeCaster,
    LinearVelocity,
    CollisionLayers,
    PlayerMarker,
);

type GroundQuery<'a, 'b> = Single<'a, (&'b ShapeHits, &'b Rotation), With<PlayerMarker>>;

impl Player {
    const PLAYER_SIZE: (f32, f32) = (32.0, 32.0);
    const PLAYER_HURT_BOX_SIZE: (f32, f32) = (10.0, 14.0);
    const PLAYER_MAX_DISTANCE: f32 = 0.8;

    fn new(transition: Vec3) -> Self {
        let collider =
            Collider::capsule(Self::PLAYER_HURT_BOX_SIZE.0, Self::PLAYER_HURT_BOX_SIZE.1);
        let mut scale_collider = collider.clone();
        scale_collider.set_scale(Vec2::ONE * 0.99, 1);
        Self(
            Camera2d,
            Aseprite::default()
                .with_size(Vec2::new(Self::PLAYER_SIZE.0, Self::PLAYER_SIZE.1))
                .with_filp_x(true),
            HP::default(),
            Transform::from_translation(transition),
            RigidBody::Dynamic,
            collider,
            CollisionEventsEnabled,
            LockedAxes::ROTATION_LOCKED,
            Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
            Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
            ShapeCaster::new(scale_collider, Vec2::ZERO, 0.0, Dir2::NEG_Y)
                .with_max_distance(Self::PLAYER_MAX_DISTANCE),
            LinearVelocity(Vec2::new(0.0, 0.0)),
            CollisionLayers::new(
                GameCollisionLayers::Player,
                [
                    GameCollisionLayers::Hit,
                    GameCollisionLayers::Enviroment,
                    GameCollisionLayers::Operation,
                ],
            ),
            PlayerMarker,
        )
    }
}
