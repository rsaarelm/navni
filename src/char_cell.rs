use crate::{Rgba, X256Color};

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

impl From<Rgba> for CharCell {
    fn from(value: Rgba) -> Self {
        CharCell::new('█', value, X256Color::BACKGROUND)
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

    /// Create default-colored `CharCell` with given char.
    pub fn c(c: char) -> Self {
        CharCell::new(c, X256Color::FOREGROUND, X256Color::BACKGROUND)
    }

    /// Set foreground color of cell.
    pub fn col(mut self, foreground: impl Into<X256Color>) -> Self {
        self.foreground = foreground.into();
        self
    }

    /// Swap background and foreground colors of cell.
    pub fn inv(mut self) -> Self {
        std::mem::swap(&mut self.foreground, &mut self.background);
        self
    }
}
