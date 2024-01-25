use super::*;

/// Issue a valid command
#[derive(Event)]
pub struct IssueCommand {
    pub command: Entity,
    pub text: String,
}

pub fn log_issued_commands(mut events: EventReader<IssueCommand>) {
    for event in events.read() {
        debug!(
            "Command issued with text \"{}\": {:?}",
            event.text, event.command
        );
    }
}
