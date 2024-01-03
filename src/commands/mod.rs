//! General and specific code for every Command in the game. Not to be confused with Bevy Commands.
use bevy::prelude::*;
use dyn_eq::DynEq;
use linkme::distributed_slice;
use std::fmt::{self, Debug};

mod events;
mod macros;

pub use events::*;
pub(crate) use macros::*;

/// Commands that are always available to run
#[distributed_slice]
pub static GLOBAL_COMMANDS: [&'static dyn Command];

pub struct CommandsPlugin;
impl Plugin for CommandsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CommandIssued>()
            .add_systems(Update, log_issued_commands);
    }
}

/// A command that can be ran by a player.
#[typetag::serde]
#[async_trait::async_trait]
pub trait Command: Send + Sync + dyn_clone::DynClone + DynEq {
    fn name(&self) -> &'static str;
    fn summary(&self) -> &'static str;

    async fn execute(
        &self,
        context: &CommandContext,
        args: String,
        world: &mut World,
    ) -> Result<(), CommandError>;
}
impl Debug for dyn Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}
dyn_clone::clone_trait_object!(Command);
dyn_eq::eq_trait_object!(Command);

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

pub fn get_global_command(name: &str) -> Option<&'static dyn Command> {
    GLOBAL_COMMANDS
        .iter()
        .find(|cmd| cmd.name() == name)
        .cloned()
}

#[cfg(feature = "test-command")]
define_global_command!(
    TestCommand,
    "test",
    "Temporary command for testing and building the commands system",
    |_, ctx: &CommandContext, args, _| {
        ctx.output_append(&format!("Hello {}", args));
        Ok(())
    }
);
