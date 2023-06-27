use navni::{
    area, Backend, CharCell, Config, Key, MouseButton, MousePress, MouseState,
    Rgba, StackOp, X256Color,
};

fn show(_: &mut (), b: &mut dyn Backend, _: u32) -> Option<StackOp<()>> {
    let a = area(80, 24);
    let mut buf: Vec<CharCell> = vec![Default::default(); a.volume() as usize];

    // Demonstrate the key down check function.
    if b.is_down(Key::Char('w')) || b.is_down(Key::Up) {
        buf[a.idx([1, 0])] =
            CharCell::new('^', X256Color::LIME, X256Color::BACKGROUND);
    }
    if b.is_down(Key::Char('a')) || b.is_down(Key::Left) {
        buf[a.idx([0, 1])] =
            CharCell::new('<', X256Color::LIME, X256Color::BACKGROUND);
    }
    if b.is_down(Key::Char('s')) || b.is_down(Key::Down) {
        buf[a.idx([1, 1])] =
            CharCell::new('v', X256Color::LIME, X256Color::BACKGROUND);
    }
    if b.is_down(Key::Char('d')) || b.is_down(Key::Right) {
        buf[a.idx([2, 1])] =
            CharCell::new('>', X256Color::LIME, X256Color::BACKGROUND);
    }

    if b.is_down(Key::Shift) {
        buf[a.idx([1, 4])] =
            CharCell::new('S', X256Color::BLUE, X256Color::BACKGROUND);
    }
    if b.is_down(Key::Ctrl) {
        buf[a.idx([1, 5])] =
            CharCell::new('C', X256Color::BLUE, X256Color::BACKGROUND);
    }
    if b.is_down(Key::Alt) {
        buf[a.idx([1, 6])] =
            CharCell::new('A', X256Color::BLUE, X256Color::BACKGROUND);
    }
    if b.is_down(Key::Icon) {
        buf[a.idx([1, 7])] =
            CharCell::new('I', X256Color::BLUE, X256Color::BACKGROUND);
    }

    // Draw colorful stuff.
    for [x, y] in area(16, 16).into_iter() {
        let c = CharCell::new('@', X256Color(x as u8), X256Color(y as u8));
        buf[a.idx([x + 2, y + 2])] = c;
    }

    for [x, y] in area(16, 16).into_iter() {
        let c = CharCell::new(
            (b' ' + ((x as u8 + y as u8 * 16) % 0x60)) as char,
            X256Color((x + 16 * y) as u8),
            X256Color::BACKGROUND,
        );
        buf[a.idx([x + 20, y + 2])] = c;
    }

    for [x, y] in area(16, 16).into_iter() {
        let c = CharCell::new(
            '@',
            X256Color::FOREGROUND,
            X256Color((x + 16 * y) as u8),
        );
        buf[a.idx([x + 38, y + 2])] = c;
    }

    for [x, y] in area(16, 16).into_iter() {
        let c = CharCell::new(
            '@',
            X256Color::BACKGROUND,
            X256Color((x + 16 * y) as u8),
        );
        buf[a.idx([x + 56, y + 2])] = c;
    }

    match b.mouse_state() {
        MouseState::Pressed(
            pos,
            MousePress {
                button: MouseButton::Left,
                ..
            },
        ) => {
            if a.contains(pos) {
                buf[a.idx(pos)] = CharCell::new(
                    ' ',
                    X256Color::FOREGROUND,
                    X256Color::FUCHSIA,
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

    b.draw_chars(a.width() as u32, a.height() as u32, &buf);

    if b.keypress().key() == Key::Esc {
        return Some(StackOp::Pop);
    }

    None
}

fn main() {
    navni::run(
        &Config {
            window_title: "navni demo".to_owned(),
            system_color_palette: Some(LIGHT_PALETTE),
            ..Default::default()
        },
        (),
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
