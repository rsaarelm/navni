use navni::prelude::*;

fn show(b: &mut dyn Backend, _: u32) {
    const W: usize = 80;
    const H: usize = 24;

    let mut buf: Vec<CharCell> = vec![Default::default(); W * H];

    // Demonstrate the key down check function.
    if b.is_down(Key::Char('w')) || b.is_down(Key::Up) {
        buf[1 + 0 * W] =
            CharCell::new('^', X256Color::LIME, X256Color::BACKGROUND);
    }
    if b.is_down(Key::Char('a')) || b.is_down(Key::Left) {
        buf[0 + 1 * W] =
            CharCell::new('<', X256Color::LIME, X256Color::BACKGROUND);
    }
    if b.is_down(Key::Char('s')) || b.is_down(Key::Down) {
        buf[1 + 1 * W] =
            CharCell::new('v', X256Color::LIME, X256Color::BACKGROUND);
    }
    if b.is_down(Key::Char('d')) || b.is_down(Key::Right) {
        buf[2 + 1 * W] =
            CharCell::new('>', X256Color::LIME, X256Color::BACKGROUND);
    }

    // Draw colorful stuff.
    for y in 0..16 {
        for x in 0..16 {
            let c = CharCell::new('@', X256Color(x as u8), X256Color(y as u8));
            buf[x + 2 + (y + 2) * W] = c;
        }
    }

    for y in 0..16 {
        for x in 0..16 {
            let c = CharCell::new(
                navni::CODEPAGE_437[x + y * 16],
                X256Color((x + 16 * y) as u8),
                X256Color::BACKGROUND,
            );
            buf[x + 20 + (y + 2) * W] = c;
        }
    }

    for y in 0..16 {
        for x in 0..16 {
            let c = CharCell::new(
                '@',
                X256Color::FOREGROUND,
                X256Color((x + 16 * y) as u8),
            );
            buf[x + 38 + (y + 2) * W] = c;
        }
    }

    for y in 0..16 {
        for x in 0..16 {
            let c = CharCell::new(
                '@',
                X256Color::BACKGROUND,
                X256Color((x + 16 * y) as u8),
            );
            buf[x + 56 + (y + 2) * W] = c;
        }
    }

    match b.mouse_state() {
        MouseState::Drag(pos, _, MouseButton::Left) => {
            if pos[0] >= 0
                && pos[1] >= 0
                && pos[0] < W as i32
                && pos[1] < H as i32
            {
                buf[pos[0] as usize + W * pos[1] as usize] = CharCell::new(
                    ' ',
                    X256Color::FOREGROUND,
                    X256Color::FUCHSIA,
                );
            }
        }
        MouseState::Release(_, _, MouseButton::Right) => {
            b.quit();
        }
        _ => {}
    }

    b.draw_chars(W as u32, H as u32, &buf);

    if b.keypress().key() == Key::Esc {
        b.quit();
    }
}

fn main() {
    run(
        &Config {
            application_name: "Navni demo".to_owned(),
            system_color_palette: Some(LIGHT_PALETTE),
            ..Default::default()
        },
        show,
    );
}

const LIGHT_PALETTE: [Rgba; 16] = [
    Rgba::new(0xaa, 0xaa, 0xaa, 0xff), // white
    Rgba::new(0x66, 0x00, 0x00, 0xff), // maroon
    Rgba::new(0x00, 0x66, 0x00, 0xff), // green
    Rgba::new(0x66, 0x33, 0x00, 0xff), // brown
    Rgba::new(0x00, 0x00, 0x88, 0xff), // navy
    Rgba::new(0x66, 0x00, 0x66, 0xff), // purple
    Rgba::new(0x00, 0x66, 0x66, 0xff), // teal
    Rgba::new(0x33, 0x33, 0x33, 0xff), // gray
    Rgba::new(0x77, 0x77, 0x77, 0xff), // silver
    Rgba::new(0xaa, 0x00, 0x00, 0xff), // red
    Rgba::new(0x00, 0xaa, 0x00, 0xff), // lime
    Rgba::new(0xaa, 0x55, 0x00, 0xff), // yellow
    Rgba::new(0x00, 0x00, 0xaa, 0xff), // blue
    Rgba::new(0xaa, 0x00, 0xaa, 0xff), // fuchsia
    Rgba::new(0x00, 0x99, 0x99, 0xff), // aqua
    Rgba::new(0x00, 0x00, 0x00, 0xff), // black
];
