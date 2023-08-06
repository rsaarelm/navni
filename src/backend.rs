//! Placeholder module in case no features were set.

use crate::prelude::*;

pub fn run<T: 'static>(
    config: &Config,
    game: T,
    scene: impl Scene<T> + 'static,
) -> ! {
    panic!("Please compile with --features=gui or --features=tty");
}
