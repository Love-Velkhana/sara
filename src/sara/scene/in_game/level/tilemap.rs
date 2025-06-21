use super::{Level, LevelState, LevelWaitChange};
use crate::scene::GameScene;
use crate::utils::prelude::*;
use crate::{data::prelude::*, model::prelude::*};
use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct TileMapMarker;

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
        for descriptor in &level_data.data {
            let translation = Vec3::new(descriptor.tile_pos.0, descriptor.tile_pos.1, 0.0);
            match descriptor.tile_typ {
                TileType::Pass => {
                    command
                        .spawn((
                            PassBox::new(translation, descriptor.rotation, &level_resource),
                            TileMapMarker,
                            StateScoped(LevelState::Running),
                        ))
                        .observe(Self::pass);
                }
                TileType::Wall => {
                    command.spawn((
                        Floor::new(translation, descriptor.rotation, &level_resource),
                        TileMapMarker,
                        StateScoped(LevelState::Running),
                    ));
                }
                TileType::Trap => {
                    command.spawn((
                        HitBox::new(translation, descriptor.rotation, &level_resource),
                        TileMapMarker,
                        StateScoped(LevelState::Running),
                    ));
                }
            };
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

    fn pause(
        _: Trigger<LevelWaitChange>,
        mut command: Commands,
        player: Single<Entity, With<PlayerMarker>>,
        mut playings: Query<&mut AsepritePlaying, With<TileMapMarker>>,
    ) {
        for mut playing in playings.iter_mut() {
            playing.0 = !playing.0;
        }
        command.trigger_targets(PlayerWaitChange, *player);
    }

    fn update() {}
}
impl Plugin for TileMap {
    fn build(&self, app: &mut App) {
        app.add_observer(Self::pause)
            .add_systems(OnEnter(LevelState::Loading), Self::init)
            .add_systems(OnEnter(LevelState::Running), Self::parse)
            .add_systems(Update, Self::update.run_if(Level::is_runnable()));
    }
}
