use crate::{CharCell, Key, KeyTyped, MouseState, Rgba};

/// Navni backend interfaces, access to input and buffer drawing.
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

    /// Signal that the application should close after the current update.
    fn quit(&mut self);
}

/// Interface for the navni application code.
pub trait App {
    /// Called once every frame.
    ///
    /// If the frame rate is lagging, `n_updates` will be larger than one and
    /// indicates you should do multiple updates on the logical state of the
    /// application for this frame update to keep animation rate steady.
    fn update(&mut self, b: &mut dyn Backend, n_updates: u32);
}

/// Implementation for a stateless app.
impl<F: FnMut(&mut dyn Backend, u32)> App for F {
    fn update(&mut self, b: &mut dyn Backend, n_updates: u32) {
        self(b, n_updates);
    }
}

/// Implementation of an app with an update function and persistent state
/// data, expressed as tuple `(state, update_fn)`.
impl<F: FnMut(&mut dyn Backend, u32, &mut T), T> App for (T, F) {
    fn update(&mut self, b: &mut dyn Backend, n_updates: u32) {
        (self.1)(b, n_updates, &mut self.0);
    }
}
