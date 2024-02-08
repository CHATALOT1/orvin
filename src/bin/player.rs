use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use orvin::{commands, tui, utils};
use std::time::Duration;
use tracing::Level;
use tracing_subscriber::prelude::*;

fn main() {
    tracing_subscriber::registry()
        .with(utils::tracing_file_layer(Level::DEBUG))
        .init();

    info!("Starting Player Client");

    utils::setup_panic_hook();

    // Set up and run app
    App::new()
        .add_plugins((
            MinimalPlugins.set(ScheduleRunnerPlugin {
                run_mode: bevy::app::RunMode::Loop {
                    // We add a tiny wait time to prevent unnecessary resource consumption.
                    wait: Some(Duration::from_secs_f32(0.0005)),
                },
            }),
            tui::TuiPlugin,
            commands::CommandsPlugin,
        ))
        .run();

    // Restore terminal before program execution finished
    tui::restore_terminal();
    info!("Program exiting...");
}
