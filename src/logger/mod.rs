//!
//! Runner for a platform-appropriate logging backend.
//!
//! TTY programs can't use logging to stdout so they need to be debugged with
//! the env logger.

// TODO: WASM logging
#[cfg_attr(
    all(feature = "gui", target_arch = "wasm32"),
    path = "null_logger.rs"
)]
// TODO: Windows logging
#[cfg_attr(
    all(feature = "gui", target_os = "windows"),
    path = "null_logger.rs"
)]
#[cfg_attr(
    all(
        feature = "gui",
        not(target_os = "windows"),
        not(target_arch = "wasm32")
    ),
    path = "env_logger.rs"
)]
#[cfg_attr(
    all(feature = "tty", target_os = "linux"),
    path = "syslog_logger.rs"
)]
mod backend;
pub use backend::start;
