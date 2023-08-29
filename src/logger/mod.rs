//! Runner for a platform-appropriate logging backend.
//!
//! TTY programs use stdout for logging so they need to be debugged via
//! syslog.

// TODO: WASM logging
//#[cfg_attr(
//    all(feature = "gui", target_arch = "wasm32"),
//    path = "wasm_logger.rs"
//)]
// TODO: Windows logging
//#[cfg_attr(
//    all(feature = "gui", target_os = "windows"),
//    path = "win_logger.rs"
//)]
#[cfg_attr(
    all(
        feature = "gui",
        not(target_os = "windows"),
        not(target_arch = "wasm32")
    ),
    path = "env_logger.rs"
)]
#[cfg_attr(
    all(feature = "tty", not(feature = "gui"), target_os = "linux"),
    path = "syslog_logger.rs"
)]
mod backend;
pub use backend::start;
