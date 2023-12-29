use std::time::UNIX_EPOCH;
use tracing::level_filters::LevelFilter;
use tracing_appender;
use tracing_subscriber::{self, EnvFilter};

pub fn setup_global_tracing_subscriber(default_log_level: LevelFilter) {
    tracing_subscriber::fmt()
        .with_writer(tracing_appender::rolling::daily(
            "logs",
            "orvin.log", // format!(
                         //     "{:#?}.log",
                         //     std::time::SystemTime::now()
                         //         .duration_since(UNIX_EPOCH)
                         //         .expect("System time should not be before the Unix Epoch")
                         // ),
        ))
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(default_log_level.into())
                .from_env_lossy(),
        )
        .with_ansi(false)
        .init();
}
