//! Graphical desktop application backend.
use std::{future::Future, sync::Mutex};

use crate::{FontSheet, FrameFuture, KeyTyped, prelude::*};

use self::runtime::Handle;

mod event;
mod runtime;

pub fn run(window_title: &str, amain: impl Future<Output = ()> + 'static) {
    let config = miniquad::conf::Conf {
        window_title: window_title.to_owned(),
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    };

    unsafe {
        runtime::FUTURE = Some(Box::pin(amain));
    }

    miniquad::start(config, move || {
        runtime::RUNTIME
            .set(Mutex::new(runtime::Runtime::new()))
            .map_err(|_| panic!("backend initialized twice"))
            .unwrap();

        Box::new(Handle)
    });
}

pub fn set_font(sheet: &FontSheet) {
    runtime::with(|r| r.set_font(sheet));
}

pub fn set_palette(palette: &[Rgba; 16]) {
    runtime::with(|r| r.set_palette(palette));
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

pub fn char_resolution(max_w: u32, max_h: u32) -> (u32, u32) {
    runtime::with(|r| r.char_resolution(max_w, max_h))
}

pub fn now() -> f64 {
    miniquad::date::now()
}

pub fn sleep(seconds: f64) {
    // WASM Does not have a sleep function, so busy-loop instead.
    #[cfg(target_arch = "wasm32")]
    {
        let start = now();
        while now() - start < seconds {}
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        std::thread::sleep(std::time::Duration::from_secs_f64(seconds));
    }
}

pub fn is_down(key: Key) -> bool {
    runtime::with(|r| r.key_down.contains(&key))
}

pub fn keypress() -> KeyTyped {
    runtime::with(|r| r.keypress.front().copied().unwrap_or_default())
}

pub fn mouse_state() -> MouseState {
    runtime::with(|r| r.mouse_state)
}

pub fn backend_type() -> BackendType {
    BackendType::Gui
}
