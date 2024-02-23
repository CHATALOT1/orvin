use crate::tui;
use bevy::prelude::*;
use tracing::{Level, Subscriber};
use tracing_appender;
use tracing_subscriber::{self, fmt, registry::LookupSpan, EnvFilter, Layer};

pub fn tracing_file_layer<S: Subscriber + for<'span> LookupSpan<'span> + 'static>(
    default_log_level: Level,
) -> impl Layer<S> {
    fmt::layer()
        .with_writer(tracing_appender::rolling::never(
            "logs",
            format!(
                "{}",
                chrono::offset::Utc::now().format("%d-%m-%Y_%H-%M-%S.log")
            ),
        ))
        .with_ansi(false)
        .with_filter(
            EnvFilter::builder()
                .with_default_directive(default_log_level.into())
                .from_env_lossy(),
        )
}

/// Set up panic hook to restore terminal and log the panic
pub fn setup_panic_hook() {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        error!("Panicked! Restoring terminal. Panic details logged below.");
        tui::restore_terminal();
        error!("{}", panic_info);
        original_hook(panic_info);
    }));
}
