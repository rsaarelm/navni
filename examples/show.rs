use navni::prelude::*;
use std::env;

type GameData = image::RgbaImage;

fn show(b: &mut dyn Backend, _: u32, game: &mut GameData) {
    if b.keypress().key() == Key::Esc {
        b.quit();
    }

    {
        let rgba_slice: &[Rgba] = unsafe { game.as_raw().align_to::<Rgba>().1 };
        let buf = &rgba_slice[..(game.width() * game.height()) as usize];

        b.draw_pixels(game.width(), game.height(), buf);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let image =
        image::io::Reader::open(args.get(1).expect("Usage: tty-image [FILE]"))
            .expect("Failed to open image file")
            .decode()
            .expect("Failed to parse image file")
            .to_rgba8();

    run(&Default::default(), (image, show));
}
