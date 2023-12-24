use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use std::time::{Duration, UNIX_EPOCH};
use tracing_appender;
use tracing_subscriber;

mod net;
mod tui;

fn main() {
    // Set up tracing subscriber
    tracing_subscriber::fmt()
        .with_writer(tracing_appender::rolling::never(
            "logs",
            format!(
                "{:#?}.log",
                std::time::SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("System time should not be before the Unix Epoch")
            ),
        ))
        .init();

    // Set up panic hook to restore terminal
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        tui::restore_terminal();
        original_hook(panic_info);
    }));

    // Set up and run app
    App::new()
        .add_plugins((
            MinimalPlugins.set(ScheduleRunnerPlugin {
                run_mode: bevy::app::RunMode::Loop {
                    wait: Some(Duration::from_secs_f32(0.0005)),
                },
            }),
            tui::TuiPlugin,
        ))
        .run();

    // Restore terminal before program execution finished
    tui::restore_terminal();
}
