use navni::prelude::*;

fn fractal(_: &mut (), b: &mut dyn Backend, _: u32) -> Option<StackOp<()>> {
    let (w, h) = b.pixel_resolution();
    let (w, h) = (w.min(640), h.min(360));

    let mut buf = vec![Rgba::default(); (w * h) as usize];

    for v in 0..h {
        for u in 0..w {
            let x0 = -2.0 + ((u as f32 / w as f32) * 2.47);
            let y0 = -1.12 + ((v as f32 / h as f32) * 2.24);
            let (mut x, mut y) = (0.0, 0.0);

            let mut c = 0;

            while x * x + y * y <= 4.0 {
                if c == 767 {
                    c = 0;
                    break;
                }

                let a = x * x - y * y + x0;
                y = 2.0 * x * y + y0;
                x = a;

                c += 1;
            }

            let i = (u + w * v) as usize;
            if c < 256 {
                buf[i] = Rgba::new(c as u8, 0, 0, 0xff);
            } else if c < 512 {
                buf[i] = Rgba::new(0xff, (c - 256) as u8, 0, 0xff);
            } else {
                buf[i] = Rgba::new(0xff, 0xff, (c - 512) as u8, 0xff);
            }
        }
    }

    b.draw_pixels(w as u32, h as u32, &buf);

    if b.keypress().key() == Key::Esc {
        return Some(StackOp::Pop);
    }

    None
}

fn main() {
    run(&Default::default(), (), fractal);
}
