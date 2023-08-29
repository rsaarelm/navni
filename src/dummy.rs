//! Placeholder module in case no features were set.

use crate::prelude::*;

#[allow(dead_code)]
pub fn run<T: 'static>(
    _config: &Config,
    _game: T,
    _scene: impl Scene<T> + 'static,
) -> ! {
    panic!("Please compile with --features=gui or --features=tty");
}
