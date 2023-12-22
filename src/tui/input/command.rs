use bevy::prelude::*;
use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};

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
    mut input_reader: EventReader<super::InputEvent>,
    mut command_input_state: ResMut<CommandInputState>,
) {
    for event in input_reader.read() {
        if let Event::Key(key_event) = event.0 {
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
