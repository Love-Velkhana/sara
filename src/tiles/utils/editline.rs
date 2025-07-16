use bevy::{ecs::system::IntoObserverSystem, input::keyboard::KeyboardInput, prelude::*};

#[derive(Event)]
pub struct EditFinished;

#[derive(Component, Default)]
pub struct CursorPosition(pub usize);

#[derive(Component, Default)]
#[require(Text, CursorPosition)]
pub struct EditableText;

#[derive(Component)]
pub struct EditableTextEntity(pub Entity);
impl Default for EditableTextEntity {
    fn default() -> Self {
        Self(Entity::PLACEHOLDER)
    }
}

#[derive(Component)]
#[require(Node, Button, Outline, EditableTextEntity)]
pub struct EditLine;
impl EditLine {
    pub const DEFAULT_OUTLINE: Outline = Outline::new(Val::Px(1.0), Val::ZERO, Color::BLACK);
    pub const SELECTED_OUTLINE: Outline = Outline::new(Val::Px(1.0), Val::ZERO, Color::WHITE);
}

#[derive(Component)]
#[component(storage = "SparseSet")]
struct CurrentEditable;

pub struct EditLinePlugin;
impl EditLinePlugin {
    pub fn spawn_edit<E, B, M, K>(
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
        command.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(33.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            children![(
                Text::new(label_text),
                TextFont {
                    font_size: 16.0,
                    ..Default::default()
                }
            )],
        ));
        let editable_text_id = command
            .commands()
            .spawn((
                Text::new(text_val),
                CursorPosition(text_val.len()),
                EditableText,
                marker,
            ))
            .id();
        command
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(67.0),
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
            .observe(observer);
    }

    fn handle_input(
        mut command: Commands,
        mut text_query: Query<&mut Text, With<EditableText>>,
        mut char_input_events: EventReader<KeyboardInput>,
        mut cursor_position_query: Query<&mut CursorPosition, With<EditableText>>,
        entry: Option<Single<(Entity, &EditableTextEntity), With<CurrentEditable>>>,
    ) {
        let (entity, editable_text_entity) = if let Some(entry_s) = entry {
            (entry_s.0, entry_s.1)
        } else {
            return;
        };
        let mut edit_text = text_query.get_mut(editable_text_entity.0).unwrap();
        let mut cursor_position = cursor_position_query
            .get_mut(editable_text_entity.0)
            .unwrap();
        for char_input_event in char_input_events.read() {
            if !char_input_event.state.is_pressed() {
                continue;
            }
            if KeyCode::ArrowLeft == char_input_event.key_code {
                if cursor_position.0 > 0 {
                    cursor_position.0 -= 1;
                }
                continue;
            }
            if KeyCode::ArrowRight == char_input_event.key_code {
                if cursor_position.0 < edit_text.len() {
                    cursor_position.0 += 1;
                }
                continue;
            }
            if KeyCode::Delete == char_input_event.key_code
                || KeyCode::Backspace == char_input_event.key_code
            {
                if cursor_position.0 > 0 {
                    edit_text.0.remove(cursor_position.0 - 1);
                    cursor_position.0 -= 1;
                }
                continue;
            }
            if KeyCode::Enter == char_input_event.key_code {
                command.entity(entity).insert(EditLine::DEFAULT_OUTLINE);
                command.entity(entity).remove::<CurrentEditable>();
                command.trigger_targets(EditFinished, entity);
                return;
            }
            if let Some(ref text) = char_input_event.text {
                edit_text.0.insert_str(cursor_position.0, text.as_str());
                cursor_position.0 += text.len();
            }
        }
    }

    fn switch_entry(
        mut command: Commands,
        text_query: Query<&Text, With<EditableText>>,
        current_entry: Option<Single<Entity, With<CurrentEditable>>>,
        next_entrys: Query<
            (Entity, &Interaction, &EditableTextEntity),
            (With<EditLine>, Changed<Interaction>),
        >,
        mut cursor_position_query: Query<&mut CursorPosition, With<EditableText>>,
    ) {
        for (entity, interaction, editable_text_entity) in next_entrys {
            if Interaction::Pressed != *interaction {
                continue;
            }
            cursor_position_query
                .get_mut(editable_text_entity.0)
                .unwrap()
                .0 = text_query.get(editable_text_entity.0).unwrap().len();
            if let Some(current_entity) = current_entry {
                command.entity(*current_entity).remove::<CurrentEditable>();
                command
                    .entity(*current_entity)
                    .insert(EditLine::DEFAULT_OUTLINE);
                if *current_entity != entity {
                    command.trigger_targets(EditFinished, *current_entity);
                }
            }
            command.entity(entity).insert(CurrentEditable);
            command.entity(entity).insert(EditLine::SELECTED_OUTLINE);
            return;
        }
    }
}
impl Plugin for EditLinePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EditFinished>()
            .add_systems(Update, Self::handle_input)
            .add_systems(Update, Self::switch_entry);
    }
}
