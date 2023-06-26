use crate::{Key, KeyTyped, MouseState, Rgba, X256Color};

pub trait Backend {
    /// Draw a pixel buffer of a given size to backend.
    ///
    /// The buffer may be magnified if it's much smaller than the backend
    /// pixel resolution.
    fn draw_pixels(&mut self, w: u32, h: u32, buffer: &[Rgba]);

    /// Draw a character buffer of a given size to backend.
    ///
    /// The buffer may be magnified if it's much smaller than the backend
    /// char grid resolution.
    fn draw_chars(&mut self, w: u32, h: u32, buffer: &[CharCell]);

    /// Return pixel resolution of backend.
    fn pixel_resolution(&self) -> (u32, u32);

    /// Return char cell resolution of backend, depends on font size.
    fn char_resolution(&self) -> (u32, u32);

    /// Return current time in seconds starting from an unspecified epoch.
    fn now(&self) -> f64;

    /// Return if given key is currently held down.
    ///
    /// Letter keys are represented by lowercase printable letters no matter
    /// what shift status is. This method does not work well on TTY backends.
    fn is_down(&self, key: Key) -> bool;

    /// Return keypress from last frame.
    ///
    /// Only one keypress per frame is supported, should be fast enough for
    /// any reasonable framerate.
    fn keypress(&self) -> KeyTyped;

    /// Return mouse action state from last frame.
    fn mouse_state(&self) -> MouseState;
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct CharCell {
    /// Char in UTF-16.
    pub c: u16,
    pub foreground: X256Color,
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
