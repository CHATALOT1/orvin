use bevy::prelude::*;

mod net;
mod tui;

fn main() {
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
            commands::CommandsPlugin,
        ))
        .run();

    // Restore terminal before program execution finished
    tui::restore_terminal();
}
