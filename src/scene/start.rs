use super::GameScene;
use crate::data::level::LevelInit;
use bevy::prelude::*;

#[derive(Component)]
struct PlayButtonMarker;
type PlayButtonQuery<'a, 'b> = Single<'a, &'b Interaction, With<PlayButtonMarker>>;

#[derive(Component)]
struct LoadButtonMarker;
type LoadButtonQuery<'a, 'b> = Single<'a, &'b Interaction, With<LoadButtonMarker>>;

#[derive(Component)]
struct ConfigButtonMarker;
type ConfigButtonQuery<'a, 'b> = Single<'a, &'b Interaction, With<ConfigButtonMarker>>;

#[derive(Component)]
struct ExitButtonMarker;
type ExitButtonQuery<'a, 'b> = Single<'a, &'b Interaction, With<ExitButtonMarker>>;

pub struct StartScene;
impl StartScene {
    fn init(mut command: Commands) {
        command.spawn((
            Camera2d,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(32.0),
                ..Default::default()
            },
            children![
                (
                    Button,
                    Node {
                        width: Val::Px(128.0),
                        height: Val::Px(64.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        ..Default::default()
                    },
                    BackgroundColor(Color::srgb_u8(105, 106, 106)),
                    BorderColor(Color::srgb(0.0, 0.0, 0.0)),
                    BorderRadius::all(Val::Px(5.0)),
                    children![Text::new("play game")],
                    PlayButtonMarker,
                ),
                (
                    Button,
                    Node {
                        width: Val::Px(128.0),
                        height: Val::Px(64.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        ..Default::default()
                    },
                    BackgroundColor(Color::srgb_u8(105, 106, 106)),
                    BorderColor(Color::srgb(0.0, 0.0, 0.0)),
                    BorderRadius::all(Val::Px(5.0)),
                    children![Text::new("load game")],
                    LoadButtonMarker,
                ),
                (
                    Button,
                    Node {
                        width: Val::Px(128.0),
                        height: Val::Px(64.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        ..Default::default()
                    },
                    BackgroundColor(Color::srgb_u8(105, 106, 106)),
                    BorderColor(Color::srgb(0.0, 0.0, 0.0)),
                    BorderRadius::all(Val::Px(5.0)),
                    children![Text::new("setting")],
                    ConfigButtonMarker,
                ),
                (
                    Button,
                    Node {
                        width: Val::Px(128.0),
                        height: Val::Px(64.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        ..Default::default()
                    },
                    BackgroundColor(Color::srgb_u8(105, 106, 106)),
                    BorderColor(Color::srgb(0.0, 0.0, 0.0)),
                    BorderRadius::all(Val::Px(5.0)),
                    children![Text::new("exit game")],
                    ExitButtonMarker,
                ),
            ],
            StateScoped(GameScene::Start),
        ));
    }

    fn update(
        mut next_state: ResMut<NextState<GameScene>>,
        mut event_writer: EventWriter<LevelInit>,
        play_button_query: PlayButtonQuery,
        load_button_query: LoadButtonQuery,
        config_button_query: ConfigButtonQuery,
        exit_button_query: ExitButtonQuery,
    ) {
        if let Interaction::Pressed = *play_button_query {
            event_writer.write(LevelInit(0));
            next_state.set(GameScene::Start.next());
        }
        if let Interaction::Pressed = *load_button_query {
            info!("load button clicked")
        }
        if let Interaction::Pressed = *config_button_query {
            info!("config button clicked")
        }
        if let Interaction::Pressed = *exit_button_query {
            std::process::exit(0);
            //或许有更优雅的需求
        }
    }
}
impl Plugin for StartScene {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameScene::Start), Self::init)
            .add_systems(Update, Self::update.run_if(in_state(GameScene::Start)));
    }
}
