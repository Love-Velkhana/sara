use bevy::prelude::*;

#[derive(Component)]
struct LoadButtonMarker;

#[derive(Component)]
struct SaveButtonMarker;

const SAVE_BUTTON_LAB: &'static str = "save";
const LOAD_BUTTON_LAB: &'static str = "load";
const BUTTON_SIZE: (f32, f32) = (128.0, 64.0);
const BUTTON_GAP: f32 = 32.0;

pub fn init(mut command: Commands) {
    command.spawn(Camera2d);
    command.spawn((
        Node {
            width: Val::Px(BUTTON_SIZE.0 * 2.0 + BUTTON_GAP),
            height: Val::Px(BUTTON_SIZE.1),
            margin: UiRect::all(Val::Px(BUTTON_GAP)),
            justify_content: JustifyContent::Center,
            column_gap: Val::Px(BUTTON_GAP),
            ..Default::default()
        },
        children![
            (
                Button,
                Node {
                    width: Val::Px(BUTTON_SIZE.0),
                    height: Val::Px(BUTTON_SIZE.1),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(2.0)),
                    ..Default::default()
                },
                BorderRadius::all(Val::Px(5.0)),
                BorderColor(Color::srgb_u8(0, 0, 0)),
                BackgroundColor(Color::srgb_u8(105, 106, 106)),
                LoadButtonMarker,
                children![Text::new(LOAD_BUTTON_LAB)]
            ),
            (
                Button,
                Node {
                    width: Val::Px(BUTTON_SIZE.0),
                    height: Val::Px(BUTTON_SIZE.1),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,

                    border: UiRect::all(Val::Px(2.0)),
                    ..Default::default()
                },
                BorderRadius::all(Val::Px(5.0)),
                BorderColor(Color::srgb_u8(0, 0, 0)),
                BackgroundColor(Color::srgb_u8(105, 106, 106)),
                children![Text::new(SAVE_BUTTON_LAB)],
                SaveButtonMarker,
            )
        ],
    ));
}

pub fn update() {
    
}
