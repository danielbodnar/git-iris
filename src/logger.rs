use parking_lot::Mutex;
use std::fs::OpenOptions;
use std::io::{self, Write};
use tracing_subscriber::{
    EnvFilter, Registry,
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

static LOGGING_ENABLED: std::sync::LazyLock<Mutex<bool>> =
    std::sync::LazyLock::new(|| Mutex::new(false));
static LOG_FILE: std::sync::LazyLock<Mutex<Option<std::fs::File>>> =
    std::sync::LazyLock::new(|| Mutex::new(None));
static LOG_TO_STDOUT: std::sync::LazyLock<Mutex<bool>> =
    std::sync::LazyLock::new(|| Mutex::new(false));
static VERBOSE_LOGGING: std::sync::LazyLock<Mutex<bool>> =
    std::sync::LazyLock::new(|| Mutex::new(false));

/// Custom writer that writes to both file and stdout/stderr
#[derive(Clone)]
struct UnifiedWriter;

impl Write for UnifiedWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        // Write to file if configured
        if let Some(file) = LOG_FILE.lock().as_mut() {
            let _ = file.write_all(buf);
            let _ = file.flush();
        }

        // Also write to stdout if enabled (for CLI debug mode)
        if *LOG_TO_STDOUT.lock() {
            let _ = io::stdout().write_all(buf);
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        if let Some(file) = LOG_FILE.lock().as_mut() {
            let _ = file.flush();
        }
        if *LOG_TO_STDOUT.lock() {
            let _ = io::stdout().flush();
        }
        Ok(())
    }
}

impl<'a> tracing_subscriber::fmt::MakeWriter<'a> for UnifiedWriter {
    type Writer = UnifiedWriter;

    fn make_writer(&'a self) -> Self::Writer {
        UnifiedWriter
    }
}

/// Initialize unified logging system using tracing.
///
/// `debug` enables debug-level output for git-iris and rig crates. When false,
/// only warnings and errors pass through the tracing filter. This must be called
/// after CLI flags are parsed so `--log` can raise the level.
///
/// # Errors
///
/// Returns an error when the tracing subscriber cannot be initialized.
pub fn init(debug: bool) -> Result<(), Box<dyn std::error::Error>> {
    use std::sync::{Once, OnceLock};
    static INIT: Once = Once::new();
    static INIT_RESULT: OnceLock<Result<(), String>> = OnceLock::new();

    INIT.call_once(|| {
        // Check if we should enable verbose logging from environment
        let verbose_from_env = std::env::var("GIT_IRIS_VERBOSE").is_ok()
            || std::env::var("RUST_LOG").is_ok_and(|v| v.contains("debug") || v.contains("trace"));

        let verbose = debug || verbose_from_env;

        if verbose {
            set_verbose_logging(true);
            set_log_to_stdout(true);
        }

        // Enable logging to file only by default (stdout requires explicit --log flag)
        enable_logging();

        // Set up tracing subscriber with unified writer (for Rig logs)
        let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            if verbose {
                "git_iris=debug,iris=debug,rig=info,warn".into()
            } else {
                // Silent by default - no debug spam
                "warn".into()
            }
        });

        let fmt_layer = fmt::Layer::new()
            .with_target(true)
            .with_level(true)
            .with_timer(fmt::time::ChronoUtc::rfc_3339())
            .with_span_events(FmtSpan::CLOSE)
            .with_writer(UnifiedWriter);

        let result = Registry::default()
            .with(env_filter)
            .with(fmt_layer)
            .try_init()
            .map_err(|e| format!("Failed to initialize logging: {e}"));

        let _ = INIT_RESULT.set(result);
    });

    match INIT_RESULT.get() {
        Some(Ok(())) => Ok(()),
        Some(Err(e)) => Err(e.clone().into()),
        None => Err("Initialization failed unexpectedly".into()),
    }
}

pub fn enable_logging() {
    let mut logging_enabled = LOGGING_ENABLED.lock();
    *logging_enabled = true;
}

pub fn disable_logging() {
    let mut logging_enabled = LOGGING_ENABLED.lock();
    *logging_enabled = false;
}

pub fn set_verbose_logging(enabled: bool) {
    let mut verbose_logging = VERBOSE_LOGGING.lock();
    *verbose_logging = enabled;

    // Note: Verbose logging changes will take effect on next application restart
    // or can be controlled via RUST_LOG environment variable before startup
}

/// Check if a log file is already configured
#[must_use]
pub fn has_log_file() -> bool {
    LOG_FILE.lock().is_some()
}

/// Configure the log file destination.
///
/// # Errors
///
/// Returns an error when the log file cannot be opened for append.
pub fn set_log_file(file_path: &str) -> std::io::Result<()> {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;

    let mut log_file = LOG_FILE.lock();
    *log_file = Some(file);
    Ok(())
}

pub fn set_log_to_stdout(enabled: bool) {
    let mut log_to_stdout = LOG_TO_STDOUT.lock();
    *log_to_stdout = enabled;
}

// All logging goes through tracing now
#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        tracing::debug!($($arg)*)
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        tracing::error!($($arg)*)
    };
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        tracing::info!($($arg)*)
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        tracing::warn!($($arg)*)
    };
}

// Tracing macros for enhanced logging (following Rig patterns)
#[macro_export]
macro_rules! trace_debug {
    (target: $target:expr, $($arg:tt)*) => {
        tracing::debug!(target: $target, $($arg)*)
    };
    ($($arg:tt)*) => {
        tracing::debug!($($arg)*)
    };
}

#[macro_export]
macro_rules! trace_info {
    (target: $target:expr, $($arg:tt)*) => {
        tracing::info!(target: $target, $($arg)*)
    };
    ($($arg:tt)*) => {
        tracing::info!($($arg)*)
    };
}

#[macro_export]
macro_rules! trace_warn {
    (target: $target:expr, $($arg:tt)*) => {
        tracing::warn!(target: $target, $($arg)*)
    };
    ($($arg:tt)*) => {
        tracing::warn!($($arg)*)
    };
}

#[macro_export]
macro_rules! trace_error {
    (target: $target:expr, $($arg:tt)*) => {
        tracing::error!(target: $target, $($arg)*)
    };
    ($($arg:tt)*) => {
        tracing::error!($($arg)*)
    };
}
