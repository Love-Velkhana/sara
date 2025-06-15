use crate::{tile::prelude::*, utils::prelude::*};
use bevy::{asset::RenderAssetUsages, prelude::*};

#[derive(SubStates, Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
#[source(super::TilesState = super::TilesState::Running)]
enum UIState {
    #[default]
    Prepare,
    Loading,
    Running,
}

/*
use std::collections::HashMap;
#[derive(Component)]
struct RawData(HashMap<usize, TileDescriptor>);
*/

#[derive(Component)]
struct LoadButtonMarker;

#[derive(Component)]
struct SaveButtonMarker;

pub struct UIPlugin;
impl UIPlugin {
    /*
    const SAVE_BUTTON_LAB: &'static str = "save";
    const LOAD_BUTTON_LAB: &'static str = "load";
    const BUTTON_SIZE: (f32, f32) = (128.0, 64.0);
    const BUTTON_GAP: f32 = 32.0;
    */

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

    fn init(
        mut command: Commands,
        window: Single<&Window>,
        mut meshes: ResMut<Assets<Mesh>>,
        level_resource: Res<LevelResource>,
        asset: Res<Assets<LevelAsset>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        mut next_state: ResMut<NextState<AsepriteSystemState>>,
    ) {
        let level_asset = asset.get(&level_resource.data_handle).unwrap();

        const GRID_SPACING: f32 = 32.0;
        let mut mesh = Mesh::new(
            bevy::render::mesh::PrimitiveTopology::LineList,
            RenderAssetUsages::RENDER_WORLD,
        );

        let mut positions = Vec::new();
        let mut colors = Vec::new();
        for col in 0..=level_asset.cols {
            let x = col as f32 * GRID_SPACING;
            positions.push([x, 0.0, 0.0]);
            positions.push([x, (level_asset.rows as f32) * GRID_SPACING, 0.0]);
            colors.push([1.0, 0.0, 0.0, 1.0]);
            colors.push([1.0, 0.0, 0.0, 1.0]);
        }

        for row in 0..=level_asset.rows {
            let y = row as f32 * GRID_SPACING;
            positions.push([0.0, y, 0.0]);
            positions.push([(level_asset.cols as f32) * GRID_SPACING, y, 0.0]);
            colors.push([1.0, 0.0, 0.0, 1.0]);
            colors.push([1.0, 0.0, 0.0, 1.0]);
        }

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);

        command.spawn((
            Mesh2d(meshes.add(mesh)),
            MeshMaterial2d(materials.add(ColorMaterial::default())),
        ));

        /*
        let data = level_asset
            .data
            .iter()
            .map(|iter| ((iter.tile_pos.0 + iter.tile_pos.1 * 32.0) as usize, *iter))
            .collect();

        command.spawn(RawData(data));
        */
        command.spawn((
            Camera2d,
            Transform::from_translation(Vec3::new(
                window.resolution.width() / 2.0,
                window.resolution.height() / 2.0,
                3.0,
            )),
        ));
        /*
        command.spawn((
            Node {
                width: Val::Px(Self::BUTTON_SIZE.0 * 2.0 + Self::BUTTON_GAP),
                height: Val::Px(Self::BUTTON_SIZE.1),
                margin: UiRect::all(Val::Px(Self::BUTTON_GAP)),
                justify_content: JustifyContent::Center,
                column_gap: Val::Px(Self::BUTTON_GAP),
                ..Default::default()
            },
            children![
                (
                    Button,
                    Node {
                        width: Val::Px(Self::BUTTON_SIZE.0),
                        height: Val::Px(Self::BUTTON_SIZE.1),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        ..Default::default()
                    },
                    BorderRadius::all(Val::Px(5.0)),
                    BorderColor(Color::srgb_u8(0, 0, 0)),
                    BackgroundColor(Color::srgb_u8(105, 106, 106)),
                    LoadButtonMarker,
                    children![Text::new(Self::LOAD_BUTTON_LAB)]
                ),
                (
                    Button,
                    Node {
                        width: Val::Px(Self::BUTTON_SIZE.0),
                        height: Val::Px(Self::BUTTON_SIZE.1),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,

                        border: UiRect::all(Val::Px(2.0)),
                        ..Default::default()
                    },
                    BorderRadius::all(Val::Px(5.0)),
                    BorderColor(Color::srgb_u8(0, 0, 0)),
                    BackgroundColor(Color::srgb_u8(105, 106, 106)),
                    children![Text::new(Self::SAVE_BUTTON_LAB)],
                    SaveButtonMarker,
                )
            ],
        ));
        */

        for descriptor in &level_asset.data {
            let translation = Vec3::new(descriptor.tile_pos.0, descriptor.tile_pos.1, 0.0);
            match descriptor.tile_typ {
                TileType::Pass => {
                    command.spawn(PassBox::new(
                        translation,
                        descriptor.rotation,
                        &level_resource,
                    ));
                }
                TileType::Wall => {
                    command.spawn(Floor::new(
                        translation,
                        descriptor.rotation,
                        &level_resource,
                    ));
                }
                TileType::Trap => {
                    command.spawn(HitBox::new(
                        translation,
                        descriptor.rotation,
                        &level_resource,
                    ));
                }
            }
        }
        next_state.set(AsepriteSystemState::Running);
    }

    fn update() {}
}
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_sub_state::<UIState>()
        .add_systems(OnEnter(UIState::Prepare), Self::load)
        .add_systems(Update, Self::ready.run_if(in_state(UIState::Loading)))
        .add_systems(OnEnter(UIState::Running), Self::init)
        .add_systems(Update, Self::update.run_if(in_state(UIState::Running)));
    }
}
