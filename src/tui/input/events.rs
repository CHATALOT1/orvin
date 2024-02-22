use bevy::prelude::*;

/// Submit command text (including arguments), may or may not be valid to issue.
#[derive(Event, Debug)]
pub(super) struct SubmitCommandText(pub String);

/// Denote that an invalid command has been submitted for feedback purposes
#[derive(Event)]
pub(in crate::tui) struct InvalidCommandSubmitted;
