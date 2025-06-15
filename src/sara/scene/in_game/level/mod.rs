mod ccamera;
mod parallax;
mod tilemap;
use super::super::GameScene;
use super::InGameState;
use crate::data::level::*;
use bevy::prelude::*;

#[derive(Event)]
pub struct LevelWaitChange;

#[derive(SubStates, PartialEq, Eq, Clone, Copy, Debug, Default, Hash)]
#[source(GameScene = GameScene::InGame)]
#[states(scoped_entities)]
enum LevelState {
    #[default]
    Perpare,
    Loading,
    Running,
}

#[derive(Component)]
pub struct ButtonMarker;

#[derive(Component)]
pub struct Level;
impl Level {
    fn init(mut next_state: ResMut<NextState<LevelState>>) {
        next_state.set(LevelState::Loading);
    }

    fn ready(
        level_resource: Res<LevelResource>,
        asset_server: Res<AssetServer>,
        mut next_state: ResMut<NextState<LevelState>>,
    ) {
        if asset_server
            .get_load_state(&level_resource.data_handle)
            .unwrap()
            .is_loaded()
        {
            next_state.set(LevelState::Running);
        }
    }

    fn update(
        mut next_state: ResMut<NextState<GameScene>>,
        query: Single<&Interaction, With<ButtonMarker>>,
    ) {
        if let Interaction::Pressed = *query {
            next_state.set(GameScene::InGame.next());
        }
    }

    fn is_runnable() -> impl Condition<()> + Clone {
        in_state(InGameState::Running).and(in_state(LevelState::Running))
    }
}
impl Plugin for Level {
    fn build(&self, app: &mut App) {
        app.add_event::<LevelWaitChange>()
            .add_sub_state::<LevelState>()
            .add_plugins(ccamera::LevelCamera)
            .add_plugins(parallax::Parallax)
            .add_plugins(tilemap::TileMap)
            .add_systems(OnEnter(GameScene::InGame), Self::init)
            .add_systems(Update, Self::ready.run_if(in_state(LevelState::Loading)))
            .add_systems(Update, Self::update.run_if(Self::is_runnable()));
    }
}
