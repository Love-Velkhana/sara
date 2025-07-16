use super::*;
use bevy::{
    render::{camera::Viewport, view::RenderLayers},
    window::WindowResized,
};
use strum::IntoEnumIterator;

#[derive(Default, Component)]
struct ToolsMarker;

#[derive(Event)]
struct UIButtonDown;

#[derive(Component)]
#[require(
    Node = ToolsPlugin::button_base_node(),
    Button,
    BorderRadius = BorderRadius::all(Val::Px(6.0)),
    ToolsMarker,
)]
struct SaveButton;

#[derive(Component)]
#[require(
    Node = ToolsPlugin::button_base_node(),
    Button,
    BorderRadius = BorderRadius::all(Val::Px(6.0)),
    ToolsMarker
)]
struct LoadButton;

#[derive(Component)]
#[require(
    Node = Node{
        width : Val::Px(32.0),
        height : Val::Px(32.0),
        ..Default::default()
    },
    Button,
    ImageNode,
    ToolsMarker,
)]
struct TileTypeButton;

#[derive(Component)]
#[require(
    Node = Node{
        width : Val::Px(32.0),
        height : Val::Px(32.0),
        ..Default::default()
    },
    Button,
    ImageNode,
    ToolsMarker,
)]
struct TrackingButton;

#[derive(Component)]
#[require(EditableText)]
struct RotationEditLineText;

#[derive(Component)]
#[require(EditableText)]
struct GridColsEditLineText;

#[derive(Component)]
#[require(EditableText)]
struct GridRowsEditLineText;

#[derive(Component)]
#[require(EditableText)]
struct NextLevelEditLineText;

#[derive(Event)]
pub struct UpdateEditLine;

pub(super) struct ToolsPlugin;
impl ToolsPlugin {
    const SVAE_BUTTON_LAB: &'static str = "save";
    const LOAD_BUTTON_LAB: &'static str = "load";

    const DEFAULT_OUTLINE: Outline = Outline::new(Val::Px(1.0), Val::ZERO, Color::WHITE);
    const SELECTED_OUTLINE: Outline =
        Outline::new(Val::Px(1.0), Val::ZERO, Color::srgb(0.0, 1.0, 0.0));

    #[inline(always)]
    fn line_base_node() -> Node {
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(48.0),
            column_gap: Val::Percent(2.0),
            ..Default::default()
        }
    }

    #[inline(always)]
    fn button_base_node() -> Node {
        Node {
            width: Val::Percent(49.0),
            height: Val::Percent(100.0),
            margin: UiRect::all(Val::Percent(1.0)),
            border: UiRect::all(Val::Px(2.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        }
    }

    fn init(mut command: Commands, level_resource: Res<LevelStaticResource>) {
        let tool_camera_id = Self::create_camera(&mut command);
        let mut id = Entity::PLACEHOLDER;
        command
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    border: UiRect::all(Val::Px(2.0)),
                    row_gap: Val::Percent(2.0),
                    justify_content: JustifyContent::Start,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                BorderColor(Color::srgb_u8(0, 255, 255)),
                UiTargetCamera(tool_camera_id),
            ))
            .with_children(|parent| {
                Self::create_buttons(parent);
                Self::craete_editlines(parent);
                id = Self::create_elements(parent, &level_resource);
            });
        command.insert_resource(Selected {
            id,
            typ: TileType::Wall,
            rotation: 0.0,
        });
        command.trigger_targets(UIButtonDown, id);
    }

    fn create_camera(command: &mut Commands) -> Entity {
        command
            .spawn((
                Camera2d,
                Camera {
                    order: UIPlugin::TOOL_VIEWPORT_ORDER,
                    viewport: Some(Viewport::default()),
                    ..Default::default()
                },
                RenderLayers::layer(UIPlugin::TOOL_VIEWPORT_ORDER as _),
                ToolsMarker,
            ))
            .id()
    }

    fn create_buttons(command: &mut ChildSpawnerCommands) {
        command
            .spawn(Self::line_base_node())
            .with_children(|command| {
                command
                    .spawn((
                        SaveButton,
                        BorderColor(Color::BLACK),
                        children![Text::new(Self::SVAE_BUTTON_LAB)],
                    ))
                    .observe(
                        |_: Trigger<UIButtonDown>, mut next_state: ResMut<NextState<UIState>>| {
                            next_state.set(UIState::Waiting);
                        },
                    );
                command
                    .spawn((
                        LoadButton,
                        BorderColor(Color::BLACK),
                        children![Text::new(Self::LOAD_BUTTON_LAB)],
                    ))
                    .observe(
                        |_: Trigger<UIButtonDown>, mut next_state: ResMut<NextState<UIState>>| {
                            next_state.set(UIState::Prepare);
                        },
                    );
            });
    }

    fn craete_editlines(command: &mut ChildSpawnerCommands) {
        command
            .spawn(Self::line_base_node())
            .with_children(|parent| {
                Self::create_editline_node(
                    parent,
                    GridColsEditLineText,
                    "cols",
                    "0",
                    |_: Trigger<EditFinished>,
                     mut command: Commands,
                     mut map_data: ResMut<MapData>,
                     text: Single<&Text, With<GridColsEditLineText>>| {
                        let mut cols = if let Ok(val) = text.0.parse::<usize>() {
                            val
                        } else {
                            return;
                        };
                        if cols == map_data.cols {
                            return;
                        }
                        std::mem::swap(&mut map_data.cols, &mut cols);
                        if cols < map_data.cols {
                            command.trigger(tiles::GridCreateEvent);
                        } else {
                            command.trigger(tiles::ParseTilesEvent);
                        }
                    },
                );
                Self::create_editline_node(
                    parent,
                    GridRowsEditLineText,
                    "rows",
                    "0",
                    |_: Trigger<EditFinished>,
                     mut command: Commands,
                     mut map_data: ResMut<MapData>,
                     text: Single<&Text, With<GridRowsEditLineText>>| {
                        let mut rows = if let Ok(val) = text.0.parse::<usize>() {
                            val
                        } else {
                            return;
                        };
                        if rows == map_data.rows {
                            return;
                        }
                        std::mem::swap(&mut map_data.rows, &mut rows);
                        if rows < map_data.rows {
                            command.trigger(tiles::GridCreateEvent);
                        } else {
                            command.trigger(tiles::ParseTilesEvent);
                        }
                    },
                );
            });
        command
            .spawn(Self::line_base_node())
            .with_children(|parent| {
                Self::create_editline_node(
                    parent,
                    NextLevelEditLineText,
                    "next",
                    "none",
                    |_: Trigger<EditFinished>,
                     text: Single<&Text, With<NextLevelEditLineText>>,
                     mut map_data: ResMut<MapData>|
                     -> Result {
                        Ok(map_data.as_mut().next = text.parse::<usize>().ok())
                    },
                );

                Self::create_editline_node(
                    parent,
                    RotationEditLineText,
                    "rotation",
                    "0",
                    |_: Trigger<EditFinished>,
                     text: Single<&Text, With<RotationEditLineText>>,
                     mut selected: ResMut<Selected>|
                     -> Result {
                        Ok(selected.as_mut().rotation = text.parse::<f32>()?.to_radians())
                    },
                );
            });
    }

    fn create_editline_node<E, B, M, K>(
        command: &mut ChildSpawnerCommands,
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
            .spawn(Node {
                width: Val::Percent(49.0),
                height: Val::Percent(100.0),
                margin: UiRect::all(Val::Percent(1.0)),
                border: UiRect::all(Val::Px(2.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(2.0),
                ..Default::default()
            })
            .with_children(|parent| {
                EditLinePlugin::spawn_edit(parent, marker, label_text, text_val, observer);
            });
    }

    fn update_editlines(
        _: Trigger<UpdateEditLine>,
        map_data: Res<MapData>,
        mut paramset: ParamSet<(
            Single<&mut Text, With<GridColsEditLineText>>,
            Single<&mut Text, With<GridRowsEditLineText>>,
            Single<&mut Text, With<NextLevelEditLineText>>,
        )>,
    ) {
        paramset.p0().0 = map_data.cols.to_string();
        paramset.p1().0 = map_data.rows.to_string();
        paramset.p2().0 = if let Some(id) = map_data.next {
            id.to_string()
        } else {
            String::from("none")
        };
    }

    fn create_elements(
        command: &mut ChildSpawnerCommands,
        tiles_resource: &Res<LevelStaticResource>,
    ) -> Entity {
        let mut id = Entity::PLACEHOLDER;
        command
            .spawn(Node {
                width: Val::Percent(100.0),
                height: Val::Auto,
                ..Default::default()
            })
            .with_children(|parent| {
                id = Self::create_tracking(parent);
                for tile_type in TileType::iter() {
                    Self::create_choice(parent, tile_type, &tiles_resource);
                }
            });
        id
    }

    fn create_tracking(command: &mut ChildSpawnerCommands) -> Entity {
        command
            .spawn((
                TrackingButton,
                ImageNode {
                    color: Color::WHITE,
                    ..Default::default()
                },
                Self::DEFAULT_OUTLINE,
            ))
            .observe(
                |trigger: Trigger<UIButtonDown>,
                 mut command: Commands,
                 mut selected: ResMut<Selected>,
                 mut editor_next_state: ResMut<NextState<EditorState>>| {
                    command.entity(selected.id).insert(Self::DEFAULT_OUTLINE);
                    command
                        .entity(trigger.target())
                        .insert(Self::SELECTED_OUTLINE);
                    selected.as_mut().id = trigger.target();
                    editor_next_state.set(EditorState::Tracking);
                },
            )
            .id()
    }

    fn create_choice(
        command: &mut ChildSpawnerCommands,
        tile_type: TileType,
        tiles_resource: &Res<LevelStaticResource>,
    ) {
        command
            .spawn((
                TileTypeButton,
                ImageNode {
                    image: tiles_resource.texture_handle.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: tiles_resource.layout_handle.clone(),
                        index: tile_type.texture_atlas_index(),
                    }),
                    ..Default::default()
                },
                Self::DEFAULT_OUTLINE,
            ))
            .observe(
                move |trigger: Trigger<UIButtonDown>,
                      mut command: Commands,
                      mut selected: ResMut<Selected>,
                      mut editor_next_state: ResMut<NextState<EditorState>>| {
                    command.entity(selected.id).insert(Self::DEFAULT_OUTLINE);
                    command
                        .entity(trigger.target())
                        .insert(Self::SELECTED_OUTLINE);
                    selected.as_mut().id = trigger.target();
                    selected.as_mut().typ = tile_type;
                    editor_next_state.set(EditorState::Selected);
                },
            );
    }

    fn resize(
        window: Single<&Window>,
        mut window_event: EventReader<WindowResized>,
        mut camera: Single<&mut Camera, With<ToolsMarker>>,
    ) {
        for _ in window_event.read() {
            if let Some(ref mut viewport) = camera.viewport {
                viewport.physical_position = UVec2::new(
                    (window.width() * UIPlugin::TILE_VIEWPORT_VAL.x * window.scale_factor()) as _,
                    0,
                );
                viewport.physical_size =
                    (window.size() * window.scale_factor() * UIPlugin::TOOL_VIEWPORT_VAL)
                        .as_uvec2();
            }
        }
    }

    fn handle_clicked(
        mut command: Commands,
        interactions: Query<(&Interaction, Entity), (Changed<Interaction>, With<ToolsMarker>)>,
    ) {
        for interaction in interactions {
            if let Interaction::Pressed = interaction.0 {
                command.trigger_targets(UIButtonDown, interaction.1);
            }
        }
    }
}
impl Plugin for ToolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UIButtonDown>()
            .add_event::<UpdateEditLine>()
            .add_observer(Self::update_editlines)
            .add_systems(OnEnter(AppState::Running), Self::init)
            .add_systems(Update, Self::resize.run_if(in_state(AppState::Running)))
            .add_systems(
                Update,
                Self::handle_clicked.run_if(in_state(UIState::Running)),
            );
    }
}
