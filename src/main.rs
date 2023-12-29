use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use std::time::Duration;
use tracing::level_filters::LevelFilter;

mod commands;
mod net;
mod tui;
mod utils;

fn main() {
    utils::setup_global_tracing_subscriber(LevelFilter::DEBUG);

    // Set up panic hook to restore terminal
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        error!("Panicked! Restoring terminal.");
        tui::restore_terminal();
        original_hook(panic_info);
    }));

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
