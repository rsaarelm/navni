//! TTY terminal backend.
use std::{future::Future, sync::Mutex};

use crate::{prelude::*, FontSheet, FrameFuture, KeyTyped};

mod event;
mod runtime;

pub fn run(_window_title: &str, amain: impl Future<Output = ()> + 'static) {
    unsafe {
        runtime::FUTURE = Some(Box::pin(amain));
    }

    runtime::RUNTIME
        .set(Mutex::new(runtime::Runtime::new()))
        .map_err(|_| panic!("backend initialized twice"))
        .unwrap();

    loop {
        // Poll on the application future, this moves application logic
        // forward to the point where it awaits for frame change.
        //
        // If the future completes, the application run has ended and
        // we should quit.
        if unsafe { crate::exec::poll(runtime::FUTURE.as_mut().unwrap()) }
            .is_some()
        {
            break;
        }

        runtime::with(|r| {
            r.keypress.pop_front();
            r.mouse_state.frame_update();
            r.process_events();
        });
    }

    runtime::cleanup();
}

pub fn set_font(_sheet: &FontSheet) {
    // No-op on TTY
}

pub fn set_palette(_palette: &[Rgba; 16]) {
    // No-op on TTY
}

pub fn draw_pixels(w: u32, h: u32, buffer: &[crate::Rgba]) -> FrameFuture {
    runtime::with(|r| r.draw_pixels(w, h, buffer));
    FrameFuture::default()
}

pub fn draw_chars(w: u32, h: u32, buffer: &[crate::CharCell]) -> FrameFuture {
    runtime::with(|r| r.draw_chars(w, h, buffer));
    FrameFuture::default()
}

pub fn pixel_resolution() -> (u32, u32) {
    runtime::with(|r| r.pixel_resolution())
}

pub fn char_resolution(_max_w: u32, _max_h: u32) -> (u32, u32) {
    runtime::with(|r| r.char_resolution())
}

pub fn now() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};

    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
}

pub fn is_down(key: Key) -> bool {
    runtime::with(|r| r.is_down(key))
}

pub fn keypress() -> KeyTyped {
    runtime::with(|r| r.keypress.front().copied().unwrap_or_default())
}

pub fn mouse_state() -> MouseState {
    runtime::with(|r| r.mouse_state)
}

pub fn backend_type() -> BackendType {
    BackendType::Tty
}
