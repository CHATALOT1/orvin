use bevy::{
    app::AppExit,
    prelude::{Event as BevyEvent, *},
};
use crossterm::event::{poll, read, Event as CrosstermEvent, KeyCode, KeyModifiers};
use std::time::Duration;

pub mod command;
pub mod events;

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InputEvent>()
            .add_event::<events::SubmitCommandText>()
            .add_event::<events::InvalidCommandSubmitted>()
            .init_resource::<command::CommandInputState>()
            .add_systems(PreUpdate, send_events)
            .add_systems(
                Update,
                (
                    handle_quit,
                    command::handle_command_input,
                    command::handle_submitted_commands.after(command::handle_command_input),
                ),
            );
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
