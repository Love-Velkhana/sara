mod game_over;
mod in_game;
mod start;
use bevy::prelude::*;
use bevy::window::WindowResolution;

#[derive(States, Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
#[states(scoped_entities)]
pub enum GameScene {
    #[default]
    Start,
    InGame,
    GameOver,
}
impl GameScene {
    fn next(&self) -> Self {
        match self {
            Self::Start => Self::InGame,
            Self::InGame => Self::GameOver,
            Self::GameOver => Self::Start,
        }
    }
}

pub struct ScenePlugins;
impl Plugin for ScenePlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        //todo 修改到合适的窗口大小
                        resolution: WindowResolution::new(1152.0, 648.0),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .init_state::<GameScene>()
        .add_plugins(start::StartScene)
        .add_plugins(in_game::InGmaeScene)
        .add_plugins(game_over::GameOverScene);
    }
}
