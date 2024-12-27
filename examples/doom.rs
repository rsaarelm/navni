//! DOOM for navni. Have a DOOM1.WAD from https://doomwiki.org/wiki/DOOM1.WAD
//! in the working directory when you run it. Use 'm' to fire since navni
//! doesn't support reading just ctrl key as a keypress. Use Ctrl-C to quit.
//!
//! Controls will work badly in terminals that don't support [kitty keyboard
//! protocol](https://sw.kovidgoyal.net/kitty/keyboard-protocol/) since they
//! won't tell us whether keys are being held down or not.

// BUG: Exiting via Doom's menu instead of pressing 'q' will skip navni's
// cleanup and leaves the terminal in a bad state when running in TTY mode.

use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
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
    screen: Arc<Mutex<Screen>>,
    update: Sender<()>,
    keys: Receiver<(Key, bool)>,
}

impl NavniDoom {
    pub fn new(
        max_w: u32,
        max_h: u32,
        screen: Arc<Mutex<Screen>>,
        update: Sender<()>,
        keys: Receiver<(Key, bool)>,
    ) -> Self {
        NavniDoom {
            max_w,
            max_h,
            screen,
            update,
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

        {
            let mut screen = self.screen.lock().unwrap();
            screen.buf = buf;
            screen.w = w;
            screen.h = h;
        }

        let _ = self.update.send(());
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
            Key::Ctrl => *keys::KEY_FIRE,
            Key::Char('w') => *keys::KEY_UP,
            Key::Char('s') => *keys::KEY_DOWN,
            Key::Char('a') => *keys::KEY_STRAFELEFT,
            Key::Char('d') => *keys::KEY_STRAFERIGHT,
            Key::Char('q') => *keys::KEY_LEFT,
            Key::Char('e') => *keys::KEY_RIGHT,
            // Alternate fire key in case terminal doesn't support reading
            // standalone Ctrl presses.
            Key::Char('m') => *keys::KEY_FIRE,
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
    let (tx_keys, rx_keys) = mpsc::channel();
    let (tx_update, rx_update) = mpsc::channel();
    let (max_w, max_h) = navni::pixel_resolution();

    let screen = Arc::new(Mutex::new(Screen {
        buf: vec![Rgba::default(); (max_w * max_h) as usize],
        w: max_w,
        h: max_h,
    }));

    // XXX: Resolution changes can't be communicated to NavniDoom after it's
    // been spawned.
    let doom = NavniDoom::new(max_w, max_h, screen.clone(), tx_update, rx_keys);
    thread::spawn(move || {
        init(doom);
    });

    let mut pressed_keys = Vec::new();

    while let Ok(_) = rx_update.recv() {
        {
            let screen = screen.lock().unwrap();
            navni::draw_pixels(screen.w, screen.h, &screen.buf).await;
        }

        if navni::keypress().is("C-c") {
            return;
        }

        // Send key releases.
        for i in (0..pressed_keys.len()).rev() {
            if !navni::is_down(pressed_keys[i]) {
                tx_keys.send((pressed_keys[i], false)).unwrap();
                pressed_keys.swap_remove(i);
            }
        }

        let pressed = navni::keypress();
        if pressed.is_some() {
            pressed_keys.push(pressed.key());
            tx_keys.send((pressed.key(), true)).unwrap();
        }
    }
}

fn main() {
    navni::run("Navni DOOM", amain());
}
