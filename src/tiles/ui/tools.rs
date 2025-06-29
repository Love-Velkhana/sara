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

pub(super) struct ToolsPlugin;
impl ToolsPlugin {
    const SVAE_BUTTON_LAB: &'static str = "save";
    const LOAD_BUTTON_LAB: &'static str = "load";

    const DEFAULT_OUTLINE: Outline = Outline::new(Val::Px(1.0), Val::ZERO, Color::WHITE);
    const SELECTED_OUTLINE: Outline =
        Outline::new(Val::Px(1.0), Val::ZERO, Color::srgb(0.0, 1.0, 0.0));

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

    fn init(mut command: Commands, tiles_resource: Res<LevelResource>) {
        let tool_camera_id = Self::create_camera_with_viewport(&mut command);
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
                Self::create_save_load(parent);
                parent
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
                Self::create_rotation(parent);
            });
        command.insert_resource(Selected {
            id,
            typ: TileType::Wall,
            rotation: 0.0,
        });
        command.trigger_targets(UIButtonDown, id);
    }

    fn create_camera_with_viewport(command: &mut Commands) -> Entity {
        command
            .spawn((
                Camera2d,
                Camera {
                    order: UIPlugin::TOOL_VIEWPORT_ORDER,
                    viewport: Some(Viewport::default()),
                    ..Default::default()
                },
                RenderLayers::layer(UIPlugin::TILE_VIEWPORT_ORDER as _),
                ToolsMarker,
            ))
            .id()
    }

    fn create_save_load(command: &mut ChildSpawnerCommands) {
        command
            .spawn((Node {
                width: Val::Percent(100.0),
                height: Val::Px(48.0),
                column_gap: Val::Percent(2.0),
                ..Default::default()
            },))
            .with_children(|command| {
                command
                    .spawn((
                        SaveButton,
                        BorderColor(Color::BLACK),
                        children![Text::new(Self::SVAE_BUTTON_LAB)],
                    ))
                    .observe(Self::save);
                command
                    .spawn((
                        LoadButton,
                        BorderColor(Color::BLACK),
                        children![Text::new(Self::LOAD_BUTTON_LAB)],
                    ))
                    .observe(Self::load);
            });
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
        tiles_resource: &Res<LevelResource>,
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

    fn create_rotation(command: &mut ChildSpawnerCommands) {
        command.spawn((
            Node {
                width: Val::Auto,
                height: Val::Auto,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            children![Text::new("rotation")],
        ));
        let editable_text_id = command
            .commands()
            .spawn((
                Text::new("0"),
                CursorPosition(0),
                EditableText,
                RotationEditLineText,
            ))
            .id();
        command
            .spawn((
                Node {
                    width: Val::Percent(50.0),
                    height: Val::Px(48.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                EditableTextEntity(editable_text_id),
                BorderRadius::all(Val::Px(4.0)),
                EditLine::DEFAULT_OUTLINE,
                EditLine,
            ))
            .add_child(editable_text_id)
            .observe(
                |_: Trigger<EditFinished>,
                 text: Single<&Text, With<RotationEditLineText>>,
                 mut selected: ResMut<Selected>|
                 -> Result {
                    Ok(selected.as_mut().rotation = text.parse::<f32>()?.to_radians())
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

    fn save(_: Trigger<UIButtonDown>, map_data: Res<MapData>) -> Result {
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
        Ok(std::fs::write(
            String::from("assets/") + &LevelResource::data_path(map_data.id),
            &bincode::encode_to_vec(&map, bincode::config::standard())?,
        )?)
    }

    fn load(_: Trigger<UIButtonDown>) {
        info!("load button down")
    }
}
impl Plugin for ToolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UIButtonDown>()
            .add_systems(OnEnter(UIState::Running), Self::init)
            .add_systems(
                Update,
                (Self::resize, Self::handle_clicked).run_if(in_state(UIState::Running)),
            );
    }
}
