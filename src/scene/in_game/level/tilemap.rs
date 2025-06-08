use super::{Level, LevelState};
use crate::utils::prelude::*;
use crate::{data::prelude::*, model::prelude::*};
use avian2d::prelude::*;
use bevy::prelude::*;

pub struct TileMap;
impl TileMap {
    fn init(mut level_resource: ResMut<LevelResource>, asset_server: Res<AssetServer>) {
        level_resource.data_handle =
            asset_server.load::<LevelAsset>(LevelResource::data_path(level_resource.id));
    }

    fn parse(
        mut command: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut material: ResMut<Assets<ColorMaterial>>,
        level_resource: Res<LevelResource>,
        level_asset: Res<Assets<LevelAsset>>,
        mut player_state: ResMut<NextState<PlayerState>>,
        mut aseprite_system_state: ResMut<NextState<AsepriteSystemState>>,
    ) {
        let level_data = level_asset.get(&level_resource.data_handle).unwrap();
        for row_iter in 0..level_data.rows {
            for col_iter in 0..level_data.cols {
                let transition = Vec3::new(
                    (col_iter * LevelResource::TILE_SIZE.x as usize) as f32,
                    (row_iter * LevelResource::TILE_SIZE.y as usize) as f32,
                    1.0,
                );
                match level_data.data[row_iter * level_data.cols + col_iter] {
                    TileType::Wall => {
                        let shadow = Shadow::new(
                            meshes.add(Rectangle::new(
                                LevelResource::TILE_COLLIFDER_SIZE.x,
                                LevelResource::TILE_COLLIFDER_SIZE.y,
                            )),
                            material.add(Color::srgba_u8(100, 0, 0, 80)),
                        );
                        command
                            .spawn((
                                Floor::new(transition, &level_resource),
                                StateScoped(LevelState::Running),
                            ))
                            .with_child(shadow);
                    }
                    TileType::Pass => {
                        command
                            .spawn((
                                PassBox::new(transition, &level_resource),
                                StateScoped(LevelState::Running),
                            ))
                            .observe(Self::pass);
                    }
                    TileType::Hit => {
                        command.spawn((
                            HitBox::new(transition, &level_resource),
                            StateScoped(LevelState::Running),
                        ));
                    }
                    TileType::Space => (),
                };
            }
        }
        player_state.set(PlayerState::Loading);
        aseprite_system_state.set(AsepriteSystemState::Running);
    }

    fn pass(
        trigger: Trigger<OnCollisionStart>,
        player: Single<Entity, With<PlayerMarker>>,
        mut level_resource: ResMut<LevelResource>,
        mut next_level_state: ResMut<NextState<LevelState>>,
        mut player_state: ResMut<NextState<PlayerState>>,
    ) {
        if *player == trigger.collider {
            level_resource.id += 1;
            next_level_state.set(LevelState::Loading);
            player_state.set(PlayerState::Prepare);
        }
    }

    fn update() {}
}
impl Plugin for TileMap {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(LevelState::Loading), Self::init)
            .add_systems(OnEnter(LevelState::Running), Self::parse)
            .add_systems(Update, Self::update.run_if(Level::is_runnable()));
    }
}
