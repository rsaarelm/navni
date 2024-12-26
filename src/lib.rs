use std::future::Future;

#[cfg(feature = "gui")]
mod gui;
#[cfg(feature = "gui")]
use gui as backend;

#[cfg(feature = "tty")]
mod tty;
#[cfg(feature = "tty")]
use tty as backend;

#[cfg(not(any(feature = "gui", feature = "tty")))]
mod backend;

mod char_cell;
pub use char_cell::CharCell;

mod color;
pub use color::{Rgba, X256Color};

mod config;
pub use config::{FontSheet, DEFAULT_FONT_CHARS};

mod exec;
pub use exec::FrameFuture;

mod frame_counter;
pub use frame_counter::FrameCounter;

#[cfg_attr(target_arch = "wasm32", path = "wasm_directory.rs")]
#[cfg_attr(not(target_arch = "wasm32"), path = "fs_directory.rs")]
mod directory;
/// Simple writable data directory abstraction that works under WASM.
pub use directory::Directory;

mod event;
pub use event::{Key, KeyMods, KeyTyped, MouseButton, MouseState};

pub mod logger;

pub mod prelude;

// Interface wrapper
//
// Having this here creates a compile-time check that each backend
// implementation contains all the functions and lets the functions be
// documented in one place.

/// Entry point for a navni application.
///
/// Start running the given async main function initialized with the given
/// configuration. The application will terminate when the async function
/// exits.
pub fn run(window_title: &str, amain: impl Future<Output = ()> + 'static) {
    backend::run(window_title, amain);
}

/// Set a custom bitmap font sheet.
///
/// Has no effect on TTY backends, they always use the font provided by the
/// operating system terminal.
pub fn set_font(sheet: &FontSheet) {
    backend::set_font(sheet);
}

/// Set the system color palette.
///
/// Has no effect on TTY backends, they always use the palette provided by the
/// operating system terminal.
pub fn set_palette(palette: &[Rgba; 16]) {
    backend::set_palette(palette);
}

/// Draw a pixel buffer of a given size to the window.
///
/// Backends may draw buffers magnified if they are much smaller than the
/// window.
///
/// This function's result must be awaited to make the backend progress to the
/// next frame.
pub fn draw_pixels(w: u32, h: u32, buffer: &[Rgba]) -> FrameFuture {
    backend::draw_pixels(w, h, buffer)
}

/// Draw a character buffer of a given size to the window.
///
/// Backends may draw buffers magnified if they are much smaller than the
/// window.
///
/// This function's result must be awaited to make the backend progress to the
/// next frame.
pub fn draw_chars(w: u32, h: u32, buffer: &[CharCell]) -> FrameFuture {
    backend::draw_chars(w, h, buffer)
}

/// Return pixel resolution of the window.
pub fn pixel_resolution() -> (u32, u32) {
    backend::pixel_resolution()
}

/// Return char cell resolution of the window, depends on font size.
///
/// On GUI backends, if it looks like the terminal would get more than the
/// given maximum of characters along either dimension, the terminal will try
/// to zoom up the characters to fit within the bounds. This is to prevent the
/// characters from becoming tiny on high-DPI displays. TTY backends will
/// ignore `max_w` and `max_h` and always report the actual terminal
/// dimensions, since navni cannot affect font size on a TTY.
///
/// Zero values of `max_w` and `max_h` cause no adjustment to be done along
/// that dimension.
pub fn char_resolution(max_w: u32, max_h: u32) -> (u32, u32) {
    backend::char_resolution(max_w, max_h)
}

/// Return current time in seconds starting from an unspecified epoch.
pub fn now() -> f64 {
    backend::now()
}

/// Return if given key is currently held down.
///
/// Letter keys are represented by lowercase printable letters no matter
/// what shift status is. This method is not supported on TTY backends and
/// always returns false there.
pub fn is_down(key: Key) -> bool {
    backend::is_down(key)
}

/// Return keypress from last frame.
///
/// Only one keypress per frame is supported, should be fast enough for
/// any reasonable framerate.
pub fn keypress() -> KeyTyped {
    backend::keypress()
}

/// Return mouse action state from last frame.
pub fn mouse_state() -> MouseState {
    backend::mouse_state()
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum BackendType {
    Tty,
    Gui,
}

pub fn backend_type() -> BackendType {
    backend::backend_type()
}
