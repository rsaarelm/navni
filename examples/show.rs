use navni::prelude::*;
use std::env;

type GameData = image::RgbaImage;

fn show(
    game: &mut GameData,
    b: &mut dyn Backend,
    _: u32,
) -> Option<StackOp<GameData>> {
    match b.mouse_state() {
        MouseState::Pressed(
            pos,
            MousePress {
                button: MouseButton::Left,
                ..
            },
        ) => {
            if area(game.width() as i32, game.height() as i32).contains(pos) {
                game.put_pixel(
                    pos[0] as u32,
                    pos[1] as u32,
                    image::Rgba([0xff, 0, 0xff, 0xff]),
                );
            }
        }
        MouseState::Pressed(
            _,
            MousePress {
                button: MouseButton::Right,
                ..
            },
        ) => {
            return Some(StackOp::Pop);
        }
        _ => {}
    }

    if b.keypress().key() == Key::Esc {
        return Some(StackOp::Pop);
    }

    {
        let rgba_slice: &[Rgba] = unsafe { game.as_raw().align_to::<Rgba>().1 };
        let buf = &rgba_slice[..(game.width() * game.height()) as usize];

        b.draw_pixels(game.width(), game.height(), buf);
    }

    None
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let image =
        image::io::Reader::open(args.get(1).expect("Usage: tty-image [FILE]"))
            .expect("Failed to open image file")
            .decode()
            .expect("Failed to parse image file")
            .to_rgba8();

    run(&Default::default(), image, show);
}
