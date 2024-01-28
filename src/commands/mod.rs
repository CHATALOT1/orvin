//! General and specific code for every Command in the game. Not to be confused with Bevy Commands.
use bevy::prelude::*;
use dyn_eq::DynEq;
use linkme::distributed_slice;
use std::fmt;

mod events;
mod global;

pub use events::*;

pub struct CommandsPlugin;
impl Plugin for CommandsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<IssueCommand>()
            .add_systems(Startup, global::setup_global_commands)
            .add_systems(Update, log_issued_commands);
    }
}

/// A command that can be ran by a player.
#[typetag::serde]
pub trait Command: Send + Sync + dyn_clone::DynClone + DynEq {
    fn execute(&self, context: &CommandContext) -> Result<(), CommandError>;
}
dyn_clone::clone_trait_object!(Command);
dyn_eq::eq_trait_object!(Command);

pub struct CommandContext {
    pub output: std::sync::RwLock<String>,
    pub args: String,
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

#[derive(Component)]
pub enum AvailableCommand {
    Static(&'static dyn Command),
}
