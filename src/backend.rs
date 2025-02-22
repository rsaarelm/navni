use std::future::Future;

use crate::{FontSheet, FrameFuture, KeyTyped, prelude::*};

pub fn run(_window_title: &str, _amain: impl Future<Output = ()> + 'static) {
    panic!("Please compile with --features=gui or --features=tty");
}

pub fn set_font(_sheet: &FontSheet) {}

pub fn set_palette(_palette: &[Rgba; 16]) {}

pub fn draw_pixels(_w: u32, _h: u32, _buffer: &[Rgba]) -> FrameFuture {
    unimplemented!()
}

pub fn draw_chars(_w: u32, _h: u32, _buffer: &[CharCell]) -> FrameFuture {
    unimplemented!()
}

pub fn pixel_resolution() -> (u32, u32) {
    unimplemented!()
}

pub fn char_resolution(_max_w: u32, _max_h: u32) -> (u32, u32) {
    unimplemented!()
}

pub fn now() -> f64 {
    unimplemented!()
}

pub fn sleep(_seconds: f64) {
    unimplemented!()
}

pub fn is_down(_key: Key) -> bool {
    unimplemented!()
}

pub fn keypress() -> KeyTyped {
    unimplemented!()
}

pub fn mouse_state() -> MouseState {
    unimplemented!()
}

pub fn backend_type() -> BackendType {
    unimplemented!()
}
