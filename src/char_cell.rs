use crate::X256Color;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct CharCell {
    /// Char in UTF-16.
    pub c: u16,
    /// Cell foreground color, `X256Color::FOREGROUND` by default.
    pub foreground: X256Color,
    /// Cell background color, `X256Color::BACKGROUND` by default.
    pub background: X256Color,
}

impl Default for CharCell {
    fn default() -> Self {
        CharCell {
            c: 0,
            foreground: X256Color::FOREGROUND,
            background: X256Color::BACKGROUND,
        }
    }
}

impl From<char> for CharCell {
    fn from(c: char) -> Self {
        CharCell::new(c, X256Color::FOREGROUND, X256Color::BACKGROUND)
    }
}

impl CharCell {
    pub fn new(
        c: char,
        foreground: impl Into<X256Color>,
        background: impl Into<X256Color>,
    ) -> Self {
        let mut b = [0; 2];
        let c = if c.encode_utf16(&mut b).len() == 2 {
            // Error, char doesn't fit in a single UTF-16 cell.
            // 0xfffd = unicode replacement char
            0xfffd
        } else {
            b[0]
        };
        let foreground = foreground.into();
        let background = background.into();

        CharCell {
            c,
            foreground,
            background,
        }
    }
}