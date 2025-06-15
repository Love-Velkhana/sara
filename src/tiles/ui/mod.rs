mod tiles;
use crate::{tile::prelude::*, utils::prelude::*};
use bevy::prelude::*;
use tiles::*;

#[derive(SubStates, Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
#[source(super::TilesState = super::TilesState::Running)]
enum UIState {
    #[default]
    Prepare,
    Loading,
    Running,
}

#[derive(Component)]
struct LoadButtonMarker;

#[derive(Component)]
struct SaveButtonMarker;

pub struct UIPlugin;
impl UIPlugin {
    fn load(mut next_state: ResMut<NextState<UIState>>) {
        next_state.set(UIState::Loading);
    }

    fn ready(
        data: Res<LevelResource>,
        asset_server: Res<AssetServer>,
        mut next_state: ResMut<NextState<UIState>>,
    ) {
        if asset_server.is_loaded(&data.data_handle) {
            next_state.set(UIState::Running);
        }
    }
}
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(TilesPlugin)
        .add_sub_state::<UIState>()
        .add_systems(OnEnter(UIState::Prepare), Self::load)
        .add_systems(Update, Self::ready.run_if(in_state(UIState::Loading)));
    }
}
