use std::path::PathBuf;

use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::EnvFilter;

pub fn init_logging() -> Option<WorkerGuard> {
    let log_dir = get_log_dir();
    let _ = std::fs::create_dir_all(&log_dir);

    let file_appender = tracing_appender::rolling::daily(&log_dir, "devhome-relocator.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .with_writer(non_blocking)
        .with_ansi(false)
        .init();

    Some(guard)
}

fn get_log_dir() -> PathBuf {
    std::env::var("LOCALAPPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("DevHomeRelocator")
        .join("logs")
}
