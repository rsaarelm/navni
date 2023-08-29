use std::time::Duration;

pub const FRAME_DURATION: Duration = Duration::from_micros(1_000_000 / 60);

/// Never ask more than this many updates to prevent death spirals if
/// something in the update slows things down very badly.
pub const MAX_UPDATES_PER_FRAME: u32 = 30;

#[cfg(feature = "gui")]
pub mod gui;

#[cfg(feature = "tty")]
pub mod tty;

#[cfg(all(not(feature = "tty"), not(feature = "gui")))]
pub use dummy::run;
#[cfg(feature = "gui")]
pub use gui::run;
#[cfg(all(feature = "tty", not(feature = "gui")))]
pub use tty::run;

mod char_cell;
pub use char_cell::CharCell;

mod color;
pub use color::{Rgba, X256Color};

mod config;
pub use config::{Config, FontSheet, CODEPAGE_437};

mod dummy;

mod event;
pub use event::{Key, KeyMods, KeyTyped, MouseButton, MouseState};

mod interface;
pub use interface::Backend;

pub mod logger;

pub mod prelude;

mod scene;
pub use scene::{Scene, StackOp};
