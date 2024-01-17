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
