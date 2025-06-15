mod level;
mod pause;

use super::GameScene;
use avian2d::prelude::*;
use bevy::prelude::*;
use level::LevelWaitChange;

#[derive(SubStates, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[source(GameScene = GameScene::InGame)]
#[states(scoped_entities)]
enum InGameState {
    #[default]
    Running,
    Paused,
}
impl InGameState {
    fn next(&self) -> Self {
        match self {
            Self::Running => Self::Paused,
            Self::Paused => Self::Running,
        }
    }
}

pub struct InGmaeScene;
impl InGmaeScene {
    fn update(
        input: Res<ButtonInput<KeyCode>>,
        in_game_state: Res<State<InGameState>>,
        mut in_game_next_state: ResMut<NextState<InGameState>>,
    ) {
        if input.just_pressed(KeyCode::Escape) {
            in_game_next_state.set(in_game_state.next());
        }
    }
}
impl Plugin for InGmaeScene {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<InGameState>()
            .add_plugins(level::Level)
            .add_plugins(pause::Paused)
            .add_systems(
                OnEnter(InGameState::Paused),
                |mut time: ResMut<Time<Physics>>, mut command: Commands| {
                    time.pause();
                    command.trigger(LevelWaitChange);
                },
            )
            .add_systems(
                OnEnter(InGameState::Running),
                |mut time: ResMut<Time<Physics>>, mut command: Commands| {
                    time.unpause();
                    command.trigger(LevelWaitChange);
                },
            )
            .add_systems(Update, Self::update.run_if(in_state(GameScene::InGame)));
    }
}
