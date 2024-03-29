use bevy::prelude::*;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use std::io::{stdout, Stdout};

mod draw;
mod input;

type RatatuiTerminal = ratatui::Terminal<CrosstermBackend<Stdout>>;

pub struct TuiPlugin;
impl Plugin for TuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, draw::render_system)
            .add_plugins(input::InputPlugin);
    }
}

#[derive(Resource)]
pub struct Terminal(pub RatatuiTerminal);

pub fn setup(mut commands: Commands) {
    let mut stdo = stdout();
    enable_raw_mode().expect("failed to enable raw mode");
    execute!(stdo, EnterAlternateScreen).expect("unable to enter alternate screen");
    let mut term =
        RatatuiTerminal::new(CrosstermBackend::new(stdo)).expect("creating terminal failed");
    term.hide_cursor().expect("Unable to hide terminal cursor");

    commands.insert_resource(Terminal(term));
}

pub fn restore_terminal() {
    let mut stdo = stdout();
    disable_raw_mode().expect("Failed to disable raw mode.");
    execute!(stdo, LeaveAlternateScreen).expect("unable to switch to main screen");
}
