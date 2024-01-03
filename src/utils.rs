use tracing::Level;
use tracing_appender;
use tracing_subscriber::{self, EnvFilter};

pub fn setup_global_tracing_subscriber(default_log_level: Level) {
    tracing_subscriber::fmt()
        .with_writer(tracing_appender::rolling::never(
            "logs",
            format!(
                "{}",
                chrono::offset::Utc::now().format("%d-%m-%Y_%H-%M-%S.log")
            ),
        ))
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(default_log_level.into())
                .from_env_lossy(),
        )
        .with_ansi(false)
        .init();
}
