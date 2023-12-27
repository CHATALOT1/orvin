//! General and specific code for every Command in the game. Not to be confused with Bevy Commands.
use bevy::prelude::*;
use linkme::distributed_slice;
use std::fmt;

mod events;

pub use events::*;

#[distributed_slice]
pub static GENERAL_COMMANDS: [Command];

pub struct CommandsPlugin;
impl Plugin for CommandsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CommandIssued>()
            .add_systems(Update, log_issued_commands);
    }
}

pub struct Command {
    pub name: &'static str,
    pub summary: &'static str,
    pub execute:
        fn(context: &CommandContext, args: String, world: &mut World) -> Result<(), CommandError>,
}

pub struct CommandContext {
    pub output: std::sync::RwLock<String>,
}
impl CommandContext {
    pub(self) fn output_append(&self, text: &str) {
        self.output.write().unwrap().push_str(text);
    }
}

pub enum CommandError {
    FmtError(fmt::Error),
}
impl From<fmt::Error> for CommandError {
    fn from(value: fmt::Error) -> Self {
        Self::FmtError(value)
    }
}

pub fn get_command(name: &str) -> Option<&'static Command> {
    GENERAL_COMMANDS
        .iter()
        .filter(|cmd| cmd.name == name)
        .next()
}

#[cfg(feature = "test-command")]
#[distributed_slice(GENERAL_COMMANDS)]
pub static TEST_COMMAND: Command = Command {
    name: "test",
    summary: "Temporary command for testing and building the commands system",
    execute: |context, args, _| {
        context.output_append(&format!("Hello {}", args));
        Ok(())
    },
};
