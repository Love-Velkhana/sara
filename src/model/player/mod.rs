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
    LinearVelocity,
    CollisionLayers,
    PlayerMarker,
);

impl Player {
    const PLAYER_SIZE: (f32, f32) = (32.0, 32.0);
    const PLAYER_COLLIDER_SIZE: (f32, f32) = (10.0, 14.0);

    fn new(transition: Vec3) -> Self {
        Self(
            Camera2d,
            Aseprite::default()
                .with_size(Vec2::new(Self::PLAYER_SIZE.0, Self::PLAYER_SIZE.1))
                .with_filp_x(true),
            HP::default(),
            Transform::from_translation(transition),
            RigidBody::Dynamic,
            Collider::capsule(Self::PLAYER_COLLIDER_SIZE.0, Self::PLAYER_COLLIDER_SIZE.1),
            CollisionEventsEnabled,
            LockedAxes::ROTATION_LOCKED,
            Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
            Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
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

#[derive(Component)]
struct FloorChecker1;

#[derive(Component)]
struct FloorChecker2;

type GroundQuery<'a, 'b, 'c> =
    Query<'a, 'b, &'c RayHits, Or<(With<FloorChecker1>, With<FloorChecker2>)>>;

#[derive(Component)]
struct FrontChecker1;

#[derive(Component)]
struct FrontChecker2;

type FrontWallQuery<'a, 'b, 'c> =
    Query<'a, 'b, &'c RayHits, Or<(With<FrontChecker1>, With<FrontChecker2>)>>;

#[derive(Component)]
struct BackChecker1;

#[derive(Component)]
struct BackChecker2;

type BackWallQuery<'a, 'b, 'c> =
    Query<'a, 'b, &'c RayHits, Or<(With<BackChecker1>, With<BackChecker2>)>>;

struct PlayerCheckers;
impl PlayerCheckers {
    const CHECKER_X: f32 = (Player::PLAYER_COLLIDER_SIZE.0 - 0.8) / 2.0;
    const FLOOR_CHECKER_Y: f32 = -(Player::PLAYER_SIZE.1 - 0.5) / 2.0;
    const FLOOR_CHECKER_MAX_DISTANCE: f32 = 16.0;
    const WALL_CHECKER_Y: f32 = (-Player::PLAYER_SIZE.1 - 1.6) / 2.0;
    const WALL_CHECKER_MAX_DISTANCE: f32 = 6.0;

    fn add_to(command: &mut ChildSpawnerCommands) {
        command.spawn((
            RayCaster::new(
                Vec2::new(Self::CHECKER_X, Self::FLOOR_CHECKER_Y),
                Dir2::NEG_Y,
            )
            .with_max_hits(1)
            .with_max_distance(Self::FLOOR_CHECKER_MAX_DISTANCE)
            .with_query_filter(
                SpatialQueryFilter::default().with_mask(GameCollisionLayers::Enviroment),
            ),
            FloorChecker1,
        ));
        command.spawn((
            RayCaster::new(
                Vec2::new(-Self::CHECKER_X, Self::FLOOR_CHECKER_Y),
                Dir2::NEG_Y,
            )
            .with_max_hits(1)
            .with_max_distance(Self::FLOOR_CHECKER_MAX_DISTANCE)
            .with_query_filter(
                SpatialQueryFilter::default().with_mask(GameCollisionLayers::Enviroment),
            ),
            FloorChecker2,
        ));
        command.spawn((
            RayCaster::new(Vec2::new(Self::CHECKER_X, Self::WALL_CHECKER_Y), Dir2::X)
                .with_max_hits(1)
                .with_max_distance(Self::WALL_CHECKER_MAX_DISTANCE)
                .with_query_filter(
                    SpatialQueryFilter::default().with_mask(GameCollisionLayers::Enviroment),
                ),
            FrontChecker1,
        ));
        command.spawn((
            RayCaster::new(Vec2::new(Self::CHECKER_X, -Self::WALL_CHECKER_Y), Dir2::X)
                .with_max_hits(1)
                .with_max_distance(Self::WALL_CHECKER_MAX_DISTANCE)
                .with_query_filter(
                    SpatialQueryFilter::default().with_mask(GameCollisionLayers::Enviroment),
                ),
            FrontChecker2,
        ));
        command.spawn((
            RayCaster::new(
                Vec2::new(-Self::CHECKER_X, Self::WALL_CHECKER_Y),
                Dir2::NEG_X,
            )
            .with_max_hits(1)
            .with_max_distance(Self::WALL_CHECKER_MAX_DISTANCE)
            .with_query_filter(
                SpatialQueryFilter::default().with_mask(GameCollisionLayers::Enviroment),
            ),
            BackChecker1,
        ));
        command.spawn((
            RayCaster::new(
                Vec2::new(-Self::CHECKER_X, -Self::WALL_CHECKER_Y),
                Dir2::NEG_X,
            )
            .with_max_hits(1)
            .with_max_distance(Self::WALL_CHECKER_MAX_DISTANCE)
            .with_query_filter(
                SpatialQueryFilter::default().with_mask(GameCollisionLayers::Enviroment),
            ),
            BackChecker2,
        ));
    }
}
