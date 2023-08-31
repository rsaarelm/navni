//! Placeholder module in case no features were set.

use crate::prelude::*;

#[allow(dead_code)]
pub fn run(_config: &Config, _app: impl App + 'static) -> ! {
    panic!("Please compile with --features=gui or --features=tty");
}
