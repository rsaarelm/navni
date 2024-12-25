use std::time::SystemTime;

use navni::prelude::*;

fn main() {
    navni::run("update speed test", async {
        let mut t = SystemTime::now();
        let mut i = 0;

        let (w, h) = navni::char_resolution(0, 0);
        let mut buf = vec![CharCell::default(); (w * h) as usize];

        let (w, h) = (w as usize, h as usize);

        let mut mode = 0;

        loop {
            if mode > 0 {
                for x in 0..if mode == 3 { w } else { i % w } {
                    for y in 0..h {
                        buf[(x % w) + y * w] = CharCell::from('@');
                        if mode == 1 {
                            // Wall is built up but does not move, FPS should
                            // not go down as only the new rim is drawn.
                            buf[(x % w) + y * w].foreground =
                                X256Color((x % 16) as u8);
                        } else if mode >= 2 {
                            // Moving wall, whole screen needs to be redrawn,
                            // FPS may go down.
                            buf[(x % w) + y * w].foreground =
                                X256Color(((i + 9999 - x) % 16) as u8);
                        }
                    }
                }
            } else {
                for y in 0..h {
                    buf[(i % w) + y * w] = CharCell::from('@');
                    buf[(i % w) + y * w].foreground = X256Color::LIME;
                }
            }
            i += 1;

            let delta = SystemTime::now().duration_since(t).unwrap();
            t += delta;

            let fps = format!("FPS {}   ", 1000 / (delta.as_millis() + 1));
            for (i, c) in fps.chars().enumerate() {
                buf[i] = CharCell::from(c);
            }

            navni::draw_chars(w as u32, h as u32, &buf).await;
            for c in buf.iter_mut() {
                *c = CharCell::default();
            }

            if navni::keypress().key() == Key::Esc {
                break;
            }

            if navni::keypress().key() == Key::Char(' ') {
                mode = (mode + 1) % 4;
            }
        }
    });
}