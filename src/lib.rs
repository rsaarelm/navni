use std::time::Duration;

pub const FRAME_DURATION: Duration = Duration::from_micros(1_000_000 / 60);

/// Never ask more than this many updates to prevent death spirals if
/// something in the update slows things down very badly.
pub const MAX_UPDATES_PER_FRAME: u32 = 30;

#[cfg_attr(feature = "gui", path = "miniquad/mod.rs")]
#[cfg_attr(feature = "tty", path = "crossterm/mod.rs")]
mod backend;
pub use backend::run;

mod color;
pub use color::{Rgba, X256Color};

mod config;
pub use config::{Config, FontSheet, CODEPAGE_437};

mod event;
pub use event::{Key, KeyMods, KeyTyped, MouseButton, MousePress, MouseState};

mod interface;
pub use interface::{Backend, CharCell};

pub mod logger;

pub mod prelude;

mod scene;
pub use scene::{Scene, StackOp};
