use navni::prelude::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let image =
        image::ImageReader::open(args.get(1).expect("Usage: tty-image [FILE]"))
            .expect("Failed to open image file")
            .decode()
            .expect("Failed to parse image file")
            .to_rgba8();

    navni::run("image viewer", async move {
        let rgba_slice: &[Rgba] =
            unsafe { image.as_raw().align_to::<Rgba>().1 };
        let buf = &rgba_slice[..(image.width() * image.height()) as usize];
        loop {
            navni::draw_pixels(image.width(), image.height(), buf).await;
            if navni::keypress().key() == Key::Esc {
                break;
            }
        }
    });
}
