use super::*;

/// Issue a (potentially invalid) command
#[derive(Event)]
pub enum CommandIssued {
    Command {
        command: &'static dyn Command,
        args: String,
    },
    Invalid {
        text: String,
    },
}

pub fn log_issued_commands(mut reader: EventReader<CommandIssued>) {
    for event in reader.read() {
        match event {
            CommandIssued::Command { command, args } => {
                debug!("Command '{:?}' with args '{}' issued.", command, args);
            }
            CommandIssued::Invalid { text } => {
                debug!("Invalid Command '{}' issued.", text);
            }
        }
    }
}
