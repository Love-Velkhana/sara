use super::{Level, LevelState};
use crate::data::level::*;
use crate::model::player::*;
use avian2d::prelude::*;
use bevy::ecs::query::QueryData;
use bevy::prelude::*;

struct Area {
    half_width: f32,
    half_height: f32,
}

#[derive(Component)]
struct LimitArea(Area);

#[derive(Component)]
struct HoverArea(Area);

#[derive(Component)]
struct HoverFlag {
    x: bool,
    y: bool,
}

#[derive(Component)]
pub struct LevelCameraMarker;
pub type LevelCameraLinearVelocityQuery<'a, 'b> =
    Single<'a, &'b LinearVelocity, With<LevelCameraMarker>>;

#[derive(QueryData)]
#[query_data(mutable)]
struct LevelCameraParam<'a> {
    linear_velocity: &'a mut LinearVelocity,
    tranform: &'a mut Transform,
    limit_area: &'a LimitArea,
    hover_area: &'a HoverArea,
    hover_flag: &'a mut HoverFlag,
}

type LevelCameraParamQuery<'a, 'b> =
    Single<'a, LevelCameraParam<'b>, (With<LevelCameraMarker>, Without<PlayerMarker>)>;

pub struct LevelCamera;
impl LevelCamera {
    const HOVER_AREA_VAL: f32 = 80.0;

    fn init(
        mut command: Commands,
        window: Single<&Window>,
        level_resource: Res<LevelResource>,
        level_asset: Res<Assets<LevelAsset>>,
    ) {
        let data = level_asset.get(&level_resource.data_handle).unwrap();
        command.spawn((
            Camera2d,
            Sprite {
                color: Color::srgb_u8(100, 0, 0),
                custom_size: Some(Vec2::new(4.0, 4.0)),
                ..Default::default()
            },
            Transform::from_translation(Vec3::new(
                window.resolution.width() / 2.0,
                window.resolution.height() / 2.0,
                3.0,
            )),
            RigidBody::Kinematic,
            LinearVelocity::ZERO,
            LimitArea(Area {
                half_width: ((data.cols * LevelResource::TILE_SIZE.x as usize) >> 1) as f32,
                half_height: ((data.rows * LevelResource::TILE_SIZE.y as usize) >> 1) as f32,
            }),
            HoverArea(Area {
                half_width: Self::HOVER_AREA_VAL,
                half_height: Self::HOVER_AREA_VAL,
            }),
            HoverFlag { x: false, y: false },
            LevelCameraMarker,
            StateScoped(LevelState::Running),
        ));
    }

    fn follow(
        time: Res<Time>,
        window: Single<&Window>,
        mut camera_param: LevelCameraParamQuery,
        player_transform: PlayerTransformQuery,
        player_linear_velocity: PlayerLinearVelocityQuery,
    ) {
        camera_param.linear_velocity.0 = Vec2::ZERO;
        if player_linear_velocity.0 == Vec2::ZERO {
            return;
        }

        if !camera_param.hover_flag.x
            && (camera_param.tranform.translation.x - player_transform.translation.x).abs()
                < camera_param.hover_area.0.half_width
        {
            camera_param.hover_flag.x = true;
        }

        if !camera_param.hover_flag.y
            && (camera_param.tranform.translation.y - player_transform.translation.y).abs()
                < camera_param.hover_area.0.half_height
        {
            camera_param.hover_flag.y = true;
        }

        let (x_dir, y_dir) = (
            if player_linear_velocity.x > 0.0 {
                1.0
            } else {
                -1.0
            },
            if player_linear_velocity.y > 0.0 {
                1.0
            } else {
                -1.0
            },
        );

        if camera_param.hover_flag.x {
            if ((camera_param.tranform.translation.x
                + player_linear_velocity.x * time.delta_secs()
                + x_dir * window.width() / 2.0)
                - camera_param.limit_area.0.half_width)
                .abs()
                < camera_param.limit_area.0.half_width
            {
                camera_param.linear_velocity.x = player_linear_velocity.x;
            } else {
                camera_param.tranform.translation.x = camera_param.limit_area.0.half_width
                    + x_dir * (camera_param.limit_area.0.half_width - window.width() / 2.0);
                camera_param.hover_flag.x = false;
            }
        }

        if camera_param.hover_flag.y {
            if ((camera_param.tranform.translation.y
                + player_linear_velocity.y * time.delta_secs()
                + y_dir * window.height() / 2.0)
                - camera_param.limit_area.0.half_height)
                .abs()
                < camera_param.limit_area.0.half_height
            {
                camera_param.linear_velocity.y = player_linear_velocity.y;
            } else {
                camera_param.tranform.translation.y = camera_param.limit_area.0.half_height
                    + y_dir * (camera_param.limit_area.0.half_height - window.height() / 2.0);
                camera_param.hover_flag.y = false;
            }
        }
    }
}
impl Plugin for LevelCamera {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(LevelState::Running), Self::init)
            .add_systems(Update, Self::follow.run_if(Level::is_runnable()));
    }
}
