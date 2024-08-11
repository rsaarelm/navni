use navni::prelude::*;

fn main() {
    const W: usize = 640;
    const H: usize = 360;

    let buf: Vec<Rgba> = (0..W * H)
        .map(|i| [Rgba::BLACK, Rgba::WHITE][(i + ((i / W) % 2)) % 2])
        .collect();

    navni::run("raster test", async move {
        loop {
            navni::draw_pixels(W as u32, H as u32, &buf).await;

            if navni::keypress().key() == Key::Esc {
                break;
            }
        }
    });
}
