use crate::{CharCell, Key, KeyTyped, MouseState, Rgba};

pub trait Backend {
    /// Draw a pixel buffer of a given size to backend.
    ///
    /// Backends may draw buffers magnified if they are much smaller than the
    /// display area.
    fn draw_pixels(&mut self, w: u32, h: u32, buffer: &[Rgba]);

    /// Draw a character buffer of a given size to backend.
    ///
    /// Backends may draw buffers magnified if they are much smaller than the
    /// display area.
    fn draw_chars(&mut self, w: u32, h: u32, buffer: &[CharCell]);

    /// Return pixel resolution of backend.
    fn pixel_resolution(&self) -> (u32, u32);

    /// Return char cell resolution of backend, depends on font size.
    fn char_resolution(&self) -> (u32, u32);

    fn is_gui(&self) -> bool {
        false
    }

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
