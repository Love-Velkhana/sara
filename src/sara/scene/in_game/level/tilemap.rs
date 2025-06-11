use super::{Level, LevelState};
use crate::scene::GameScene;
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
        level_resource: Res<LevelResource>,
        level_asset: Res<Assets<LevelAsset>>,
        mut player_state: ResMut<NextState<PlayerState>>,
        mut aseprite_system_state: ResMut<NextState<AsepriteSystemState>>,
    ) {
        let level_data = level_asset.get(&level_resource.data_handle).unwrap();
        for row_iter in 0..level_data.rows {
            for col_iter in 0..level_data.cols {
                let transition = Vec3::new(
                    (col_iter as isize * LevelResource::TILE_SIZE.x as isize
                        + (LevelResource::TILE_SIZE.x >> 1) as isize) as f32,
                    (row_iter as isize * LevelResource::TILE_SIZE.y as isize
                        + (LevelResource::TILE_SIZE.y >> 1) as isize) as f32,
                    1.0,
                );
                match level_data.data[row_iter * level_data.cols + col_iter] {
                    TileType::Wall => {
                        command.spawn((
                            Floor::new(transition, &level_resource),
                            StateScoped(LevelState::Running),
                        ));
                    }
                    TileType::Pass => {
                        command
                            .spawn((
                                PassBox::new(transition, &level_resource),
                                StateScoped(LevelState::Running),
                            ))
                            .observe(Self::pass);
                    }
                    TileType::Trap => {
                        command.spawn((
                            HitBox::new(transition, &level_resource),
                            StateScoped(LevelState::Running),
                        ));
                    }
                    TileType::None => (),
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
        level_asset: Res<Assets<LevelAsset>>,
        mut next_level_state: ResMut<NextState<LevelState>>,
        mut player_state: ResMut<NextState<PlayerState>>,
        mut next_scene: ResMut<NextState<GameScene>>,
        mut level_event: EventWriter<LevelPass>,
    ) {
        if *player != trigger.collider {
            return;
        }
        if let Some(next_id) = level_asset.get(&level_resource.data_handle).unwrap().next {
            level_resource.id = next_id;
            next_level_state.set(LevelState::Loading);
            player_state.set(PlayerState::Prepare);
        } else {
            level_event.write(LevelPass(true));
            next_scene.set(GameScene::GameOver);
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
