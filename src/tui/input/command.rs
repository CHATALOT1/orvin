use super::events::{InvalidCommandSubmitted, SubmitCommandText};
use crate::commands::{AvailableCommand, IssueCommand};
use bevy::prelude::*;
use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};

#[derive(Resource, Default)]
pub(in crate::tui) struct CommandInputState {
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

// TODO: Up arrow for previous commands
pub(super) fn handle_command_input(
    mut input_reader: EventReader<super::InputEvent>,
    mut command_input_state: ResMut<CommandInputState>,
    mut submit_command: EventWriter<SubmitCommandText>,
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
                    (KeyCode::Enter, _) => {
                        // If the current input is all whitespace, ignore this.
                        if command_input_state.content.split_whitespace().next() == None {
                            continue;
                        };

                        // Submit the current content and reset the input state
                        submit_command.send(SubmitCommandText(command_input_state.content.clone()));
                        *command_input_state = CommandInputState::default();
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

pub(super) fn handle_submitted_commands(
    mut submitted: EventReader<SubmitCommandText>,
    commands: Query<(Entity, &Name), Has<AvailableCommand>>,
    mut issue_command: EventWriter<IssueCommand>,
    mut submit_invalid_command: EventWriter<InvalidCommandSubmitted>,
) {
    for event in submitted.read() {
        let mut args = event.0.split_whitespace();

        // This should never be None, even if the string is empty
        let Some(submitted_name) = args.next() else {
            warn!("A string containing only whitespace was submitted as a command");
            continue;
        };

        if let Some((cmd_entity, _)) = commands
            .iter()
            .find(|(_, name)| name.as_str() == submitted_name)
        {
            issue_command.send(IssueCommand {
                command: cmd_entity,
                text: event.0.clone(),
            });
        } else {
            debug!("Invalid command text \"{}\" submitted", event.0);
            submit_invalid_command.send(InvalidCommandSubmitted);
        }
    }
}
