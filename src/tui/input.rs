use bevy::{
    app::AppExit,
    prelude::{Event as BevyEvent, *},
};
use crossterm::event::{poll, read, Event as CrosstermEvent, KeyCode, KeyEventKind, KeyModifiers};
use std::time::Duration;

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InputEvent>()
            .add_systems(First, send_events)
            .init_resource::<CommandInputState>()
            .add_systems(Update, (handle_quit, handle_command_input));
    }
}

#[derive(BevyEvent)]
pub struct InputEvent(pub CrosstermEvent);

pub fn send_events(mut writer: EventWriter<InputEvent>) {
    while poll(Duration::from_secs(0)).expect("crossterm Event poll should at least be Ok") {
        writer.send(InputEvent(
            read().expect("crossterm Event read should at least be Ok"),
        ));
    }
}

pub fn handle_quit(
    mut input_reader: EventReader<InputEvent>,
    mut exit_writer: EventWriter<AppExit>,
) {
    for event in input_reader.read() {
        if let CrosstermEvent::Key(key_event) = event.0 {
            if (key_event.modifiers == KeyModifiers::CONTROL
                || key_event.modifiers == KeyModifiers::ALT)
                && (key_event.code == KeyCode::Char('c') || key_event.code == KeyCode::Char('q'))
            {
                exit_writer.send(AppExit);
            }
        }
    }
}

#[derive(Resource, Default)]
pub struct CommandInputState {
    pub content: String,
    pub cursor_pos: usize,
}

impl CommandInputState {
    fn shift_cursor(&mut self, vector: isize) {
        self.cursor_pos = match self.cursor_pos.checked_add_signed(vector) {
            Some(val) => val.clamp(0, self.content.chars().count()),
            None => self.cursor_pos,
        }
    }
}

pub fn handle_command_input(
    mut input_reader: EventReader<InputEvent>,
    mut command_input_state: ResMut<CommandInputState>,
) {
    for event in input_reader.read() {
        if let CrosstermEvent::Key(key_event) = event.0 {
            if key_event.kind != KeyEventKind::Release {
                match (key_event.code, key_event.modifiers) {
                    (KeyCode::Char(c), KeyModifiers::NONE | KeyModifiers::SHIFT) => {
                        let pos = command_input_state.cursor_pos.clone();
                        command_input_state.content.insert(pos, c);
                        command_input_state.shift_cursor(1);
                    }
                    (KeyCode::Backspace, _) | (KeyCode::Char('h'), KeyModifiers::CONTROL) => {
                        let pos = command_input_state.cursor_pos.clone();
                        if pos > 0 {
                            command_input_state.content.remove(pos - 1);
                            command_input_state.shift_cursor(-1);
                        }
                    }
                    (KeyCode::Delete, _) | (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
                        let pos = command_input_state.cursor_pos.clone();
                        if pos < command_input_state.content.chars().count() {
                            command_input_state.content.remove(pos);
                        }
                    }
                    (KeyCode::Left, _) | (KeyCode::Char('b'), KeyModifiers::CONTROL) => {
                        command_input_state.shift_cursor(-1);
                    }
                    (KeyCode::Right, _) | (KeyCode::Char('f'), KeyModifiers::CONTROL) => {
                        command_input_state.shift_cursor(1);
                    }
                    (KeyCode::Home, _) | (KeyCode::Char('a'), KeyModifiers::CONTROL) => {
                        command_input_state.cursor_pos = 0;
                    }
                    (KeyCode::End, _) | (KeyCode::Char('e'), KeyModifiers::CONTROL) => {
                        command_input_state.cursor_pos = command_input_state.content.chars().count()
                    }
                    _ => {}
                }
            }
        }
    }
}
