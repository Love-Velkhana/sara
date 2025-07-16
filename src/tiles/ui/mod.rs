mod tiles;
mod tools;
use std::collections::HashMap;

use crate::{AppState, tile::prelude::*, utils::prelude::*};
use bevy::{ecs::system::IntoObserverSystem, prelude::*};
use tiles::*;
use tools::*;

#[derive(SubStates, Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
#[source(AppState = AppState::Running)]
enum UIState {
    #[default]
    Prepare,
    Loading,
    Running,
    Waiting,
}

#[derive(Resource)]
struct Selected {
    id: Entity, // selected id
    typ: TileType,
    rotation: f32,
}

struct TileData {
    id: Entity, //tile id
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
#[require(EditableText)]
struct IdEditLineText;

pub struct UIPlugin;
impl UIPlugin {
    const TILE_VIEWPORT_ORDER: isize = 1;
    const TILE_VIEWPORT_START: Vec2 = Vec2::ZERO;
    const TILE_VIEWPORT_VAL: Vec2 = Vec2::new(0.8, 1.0);
    const TOOL_VIEWPORT_ORDER: isize = 2;
    const TOOL_VIEWPORT_VAL: Vec2 = Vec2::new(1.0 - Self::TILE_VIEWPORT_VAL.x, 1.0);
    const TOPMOST_ORDER: isize = 3;

    fn init(mut command: Commands) {
        let entity = command
            .spawn((
                Camera2d,
                Camera {
                    order: Self::TOPMOST_ORDER,
                    ..Default::default()
                },
            ))
            .id();
        Self::create_ui(
            command,
            entity,
            IdEditLineText,
            "init id",
            "0",
            move |_: Trigger<EditFinished>,
                  mut command: Commands,
                  mut map_data: ResMut<MapData>,
                  editor: Single<(Entity, &Text), With<IdEditLineText>>,
                  mut next_state: ResMut<NextState<UIState>>|
                  -> Result {
                let id = editor.1.0.parse::<usize>()?;
                map_data.id = id;
                command.entity(entity).despawn();
                command.entity(editor.0).despawn();
                if !std::fs::exists(&LevelDynamicResource::data_path(id))? {
                    map_data.cols = 0;
                    map_data.rows = 0;
                    map_data.next = None;
                    command.trigger(ParseTilesEvent);
                    command.trigger(UpdateEditLine);
                    next_state.set(UIState::Running);
                } else {
                    next_state.set(UIState::Loading);
                }
                Ok(())
            },
        );
    }

    fn load(mut command: Commands, asset_server: Res<AssetServer>, map_data: Res<MapData>) {
        command.insert_resource(LevelDynamicResource::new(map_data.id, &asset_server));
    }

    fn ready(
        mut map_data: ResMut<MapData>,
        mut command: Commands,
        asset: Res<Assets<LevelAsset>>,
        asset_server: Res<AssetServer>,
        data: Res<LevelDynamicResource>,
        mut next_state: ResMut<NextState<UIState>>,
    ) {
        if asset_server.is_loaded(&data.0) {
            let level_asset = asset.get(&data.0).unwrap();
            map_data.rows = level_asset.rows;
            map_data.cols = level_asset.cols;
            map_data.entry = level_asset.entry;
            map_data.next = level_asset.next;
            command.trigger(ParseTilesEvent);
            command.trigger(UpdateEditLine);
            next_state.set(UIState::Running);
        }
    }

    fn wait(mut command: Commands, map_data: Res<MapData>) {
        let entity = command
            .spawn((
                Camera2d,
                Camera {
                    order: Self::TOPMOST_ORDER,
                    ..Default::default()
                },
            ))
            .id();
        Self::create_ui(
            command,
            entity,
            IdEditLineText,
            "save id",
            &map_data.id.to_string(),
            move |_: Trigger<EditFinished>,
                  mut command: Commands,
                  mut map_data: ResMut<MapData>,
                  editor: Single<(Entity, &Text), With<IdEditLineText>>,
                  mut next_state: ResMut<NextState<UIState>>|
                  -> Result {
                let id = editor.1.0.parse::<usize>()?;
                map_data.id = id;
                command.entity(entity).despawn();
                command.entity(editor.0).despawn();
                let mut data = Vec::new();
                for key in map_data.data.keys() {
                    let tile_data = map_data.data.get(key).unwrap();
                    data.push(TileDescriptor {
                        tile_typ: tile_data.typ,
                        tile_pos: (key.x as f32, key.y as f32),
                        rotation: tile_data.rotation,
                    });
                }
                let map = LevelAsset {
                    rows: map_data.rows,
                    cols: map_data.cols,
                    data,
                    entry: map_data.entry,
                    next: map_data.next,
                };
                std::fs::write(
                    &LevelDynamicResource::data_path(map_data.id),
                    &bincode::encode_to_vec(&map, bincode::config::standard())?,
                )?;
                Ok(next_state.set(UIState::Running))
            },
        );
    }

    fn create_ui<E, B, M, K>(
        mut command: Commands,
        camera_entity: Entity,
        marker: K,
        label_text: &str,
        text_val: &str,
        observer: impl IntoObserverSystem<E, B, M>,
    ) where
        E: Event,
        B: Bundle,
        K: Component,
    {
        command
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                UiTargetCamera(camera_entity),
                BackgroundColor(Color::srgb(1.0, 0.0, 0.0)),
            ))
            .with_children(|parent| {
                parent
                    .spawn(Node {
                        width: Val::Px(128.0),
                        height: Val::Px(64.0),
                        row_gap: Val::Px(1.0),
                        flex_direction: FlexDirection::Column,
                        ..Default::default()
                    })
                    .with_children(|edit_parent| {
                        EditLinePlugin::spawn_edit(
                            edit_parent,
                            marker,
                            label_text,
                            text_val,
                            observer,
                        );
                    });
            });
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
        .add_systems(
            OnEnter(AppState::Running),
            |mut next_state: ResMut<NextState<AsepriteSystemState>>| {
                next_state.set(AsepriteSystemState::Running);
            },
        )
        .add_systems(OnEnter(UIState::Prepare), Self::init)
        .add_systems(OnEnter(UIState::Loading), Self::load)
        .add_systems(OnEnter(UIState::Waiting), Self::wait)
        .add_systems(Update, Self::ready.run_if(in_state(UIState::Loading)));
    }
}
