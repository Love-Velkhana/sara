use super::GameScene;
use crate::data::level::LevelPass;
use bevy::prelude::*;

#[derive(Component)]
struct GoBackStartButtonMarker;
type GoBackStartButtonQuery<'a, 'b> = Single<'a, &'b Interaction, With<GoBackStartButtonMarker>>;

pub struct GameOverScene;
impl GameOverScene {
    fn init(mut command: Commands, mut level_pass: EventReader<LevelPass>) {
        let message = if level_pass.read().last().unwrap().0 {
            "you win"
        } else {
            "game over"
        };
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
                    Node {
                        width: Val::Px(128.0),
                        height: Val::Px(64.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    children![(Text::new(message),)]
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
                    BorderColor(Color::srgb(0.0, 0.0, 0.0)),
                    BorderRadius::all(Val::Px(5.0)),
                    BackgroundColor(Color::srgb_u8(105, 106, 106)),
                    children![(
                        Text::new("go back start"),
                        TextFont {
                            font_size: 14.0,
                            ..Default::default()
                        }
                    )],
                    GoBackStartButtonMarker,
                )
            ],
            StateScoped(GameScene::GameOver),
        ));
    }
    fn update(
        mut next_state: ResMut<NextState<GameScene>>,
        go_back_start_query: GoBackStartButtonQuery,
    ) {
        if let Interaction::Pressed = *go_back_start_query {
            next_state.set(GameScene::GameOver.next());
        }
    }
}
impl Plugin for GameOverScene {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameScene::GameOver), Self::init)
            .add_systems(Update, Self::update.run_if(in_state(GameScene::GameOver)));
    }
}
