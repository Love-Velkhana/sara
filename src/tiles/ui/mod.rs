mod tiles;
mod tools;
use std::collections::HashMap;

use crate::{tile::prelude::*, utils::prelude::*};
use bevy::prelude::*;
use tiles::*;
use tools::*;

#[derive(SubStates, Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
#[source(super::TilesState = super::TilesState::Running)]
enum UIState {
    #[default]
    Prepare,
    Loading,
    Running,
}

#[derive(Resource)]
struct Selected {
    id: Entity,
    typ: TileType,
    rotation: f32,
}

struct TileData {
    id: Entity,
    typ: TileType,
    rotation: f32,
}

#[derive(Resource, Default)]
struct MapData {
    id: usize,
    rows: usize,
    cols: usize,
    data: HashMap<UVec2, TileData>,
    entry: (f32, f32),
    next: Option<usize>,
}

#[derive(SubStates, PartialEq, Eq, Clone, Copy, Default, Debug, Hash)]
#[source(UIState = UIState::Running)]
enum EditorState {
    #[default]
    Tracking,
    Selected,
}

#[derive(Component)]
struct LoadButtonMarker;

#[derive(Component)]
struct SaveButtonMarker;

pub struct UIPlugin;
impl UIPlugin {
    const TILE_VIEWPORT_ORDER: isize = 1;
    const TILE_VIEWPORT_START: Vec2 = Vec2::ZERO;
    const TILE_VIEWPORT_VAL: Vec2 = Vec2::new(0.8, 1.0);
    const TOOL_VIEWPORT_ORDER: isize = 2;
    const TOOL_VIEWPORT_VAL: Vec2 = Vec2::new(1.0 - Self::TILE_VIEWPORT_VAL.x, 1.0);

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
                resize_constraints: WindowResizeConstraints {
                    min_width: 1024.0,
                    min_height: 600.0,
                    ..Default::default()
                },
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(ToolsPlugin)
        .add_plugins(TilesPlugin)
        .init_resource::<MapData>()
        .add_sub_state::<UIState>()
        .add_sub_state::<EditorState>()
        .add_systems(OnEnter(UIState::Prepare), Self::load)
        .add_systems(Update, Self::ready.run_if(in_state(UIState::Loading)));
    }
}
