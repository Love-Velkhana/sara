use super::InGameState;
use bevy::prelude::*;

#[derive(Component)]
struct ContinueButtonMarker;
type ContinueButtonQuery<'a, 'b> = Single<'a, &'b Interaction, With<ContinueButtonMarker>>;

pub struct Paused;
impl Paused {
    fn init(mut command: Commands) {
        command.spawn((
            Camera2d,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            BackgroundColor(Color::srgba_u8(0, 0, 0, 120)),
            children![(
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
                    Text::new("continue"),
                    TextFont {
                        font_size: 14.0,
                        ..Default::default()
                    }
                )],
                ContinueButtonMarker,
            )],
            StateScoped(InGameState::Paused),
        ));
    }

    fn update(
        mut next_state: ResMut<NextState<InGameState>>,
        continue_button_query: ContinueButtonQuery,
    ) {
        if let Interaction::Pressed = *continue_button_query {
            next_state.set(InGameState::Paused.next());
            info!("paused to running")
        }
    }
}
impl Plugin for Paused {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(InGameState::Paused), Self::init)
            .add_systems(Update, Self::update.run_if(in_state(InGameState::Paused)));
    }
}
