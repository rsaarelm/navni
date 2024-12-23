//! DOOM for navni. Have a DOOM1.WAD from https://doomwiki.org/wiki/DOOM1.WAD
//! in the working directory when you run it.
//!
//! Controls currently don't work very well in TTY mode because it can't
//! really track whether keys are held down or not.

use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use doomgeneric::{
    game::{init, DoomGeneric},
    input::{keys, KeyData},
};
use navni::prelude::*;

struct Screen {
    buf: Vec<Rgba>,
    w: u32,
    h: u32,
}

struct NavniDoom {
    max_w: u32,
    max_h: u32,
    screens: Sender<Screen>,
    keys: Receiver<(Key, bool)>,
}

impl NavniDoom {
    pub fn new(
        max_w: u32,
        max_h: u32,
        screens: Sender<Screen>,
        keys: Receiver<(Key, bool)>,
    ) -> Self {
        NavniDoom {
            max_w,
            max_h,
            screens,
            keys,
        }
    }
}

impl DoomGeneric for NavniDoom {
    fn draw_frame(&mut self, screen_buffer: &[u32], xres: usize, yres: usize) {
        let w = self.max_w.min(xres as u32);
        let h = self.max_h.min(yres as u32);

        let mut buf = vec![Rgba::default(); (w * h) as usize];

        for y in 0..h {
            for x in 0..w {
                let u = x * (xres as u32) / w;
                let v = y * (yres as u32) / h;
                let c = screen_buffer[u as usize + v as usize * xres];

                let (a, r, g, b) =
                    ((c >> 24) as u8, (c >> 16) as u8, (c >> 8) as u8, c as u8);

                buf[(x + y * w) as usize] = Rgba::new(r, g, b, a);
            }
        }

        self.screens.send(Screen { buf, w, h }).unwrap();
    }

    fn get_key(&mut self) -> Option<KeyData> {
        let Ok((key, pressed)) = self.keys.try_recv() else {
            return None;
        };

        let k = match key {
            Key::Up => *keys::KEY_UP,
            Key::Down => *keys::KEY_DOWN,
            Key::Left => *keys::KEY_LEFT,
            Key::Right => *keys::KEY_RIGHT,
            Key::Esc => keys::KEY_ESCAPE,
            Key::Enter => keys::KEY_ENTER,
            // XXX: Not bothering to figure out ctrl-as-key...
            Key::Char('z') => *keys::KEY_FIRE,
            Key::Char(' ') => *keys::KEY_USE,
            Key::Char(c) => c as u8,
            _ => 0,
        };

        if k != 0 {
            Some(KeyData { pressed, key: k })
        } else {
            None
        }
    }

    fn set_window_title(&mut self, _title: &str) {}
}

async fn amain() {
    // Register for currently pressed-down key, send a release event if it's
    // released.
    let mut pressed = Key::None;

    let (tx_screens, rx_screens) = mpsc::channel();
    let (tx_keys, rx_keys) = mpsc::channel();
    let (max_w, max_h) = navni::pixel_resolution();
    // XXX: Resolution changes can't be communicated to NavniDoom after it's
    // been spawned.
    let doom = NavniDoom::new(max_w, max_h, tx_screens, rx_keys);
    thread::spawn(move || {
        init(doom);
    });

    while let Ok(screen) = rx_screens.recv() {
        navni::draw_pixels(screen.w, screen.h, &screen.buf).await;

        if pressed == Key::Char('q') {
            return;
        }

        if pressed != Key::None {
            if !navni::is_down(pressed) {
                tx_keys.send((pressed, false)).unwrap();
                pressed = Key::None;
            }
        }

        let key = navni::keypress().key();

        if key != pressed && key != Key::None {
            if pressed != Key::None {
                tx_keys.send((pressed, false)).unwrap();
            }
            pressed = key;

            tx_keys.send((key, true)).unwrap();
        }
    }
}

fn main() {
    navni::run("Navni DOOM", amain());
}
