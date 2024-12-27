use crate::{Key, KeyMods};
use crossterm::event;

impl TryFrom<event::KeyEvent> for crate::KeyTyped {
    type Error = ();

    fn try_from(
        event::KeyEvent {
            code,
            modifiers,
            kind,
            ..
        }: crossterm::event::KeyEvent,
    ) -> Result<Self, Self::Error> {
        let key = Key::try_from(code)?;
        let mut mods = KeyMods {
            shift: modifiers.intersects(event::KeyModifiers::SHIFT),
            ctrl: modifiers.intersects(event::KeyModifiers::CONTROL),
            alt: modifiers.intersects(event::KeyModifiers::ALT),
            logo: modifiers.intersects(event::KeyModifiers::SUPER),
        };
        if key.is_printable() {
            mods.shift = false;
        }

        Ok(crate::KeyTyped::new(
            key,
            mods,
            kind == event::KeyEventKind::Repeat,
        ))
    }
}

impl TryFrom<crossterm::event::KeyCode> for crate::Key {
    type Error = ();

    fn try_from(
        key_code: crossterm::event::KeyCode,
    ) -> Result<Self, Self::Error> {
        use crossterm::event::{KeyCode as K, ModifierKeyCode as M};
        use Key::*;

        match key_code {
            K::Backspace => Ok(Backspace),
            K::Enter => Ok(Enter),
            K::Left => Ok(Left),
            K::Right => Ok(Right),
            K::Up => Ok(Up),
            K::Down => Ok(Down),
            K::Home => Ok(Home),
            K::End => Ok(End),
            K::PageUp => Ok(PageUp),
            K::PageDown => Ok(PageDown),
            K::Tab | K::BackTab => Ok(Tab),
            K::Delete => Ok(Delete),
            K::Insert => Ok(Insert),
            K::F(n) => Ok(F(n)),
            K::Char(c) => Ok(Char(c)),
            K::Null => Err(()),
            K::Esc => Ok(Esc),
            K::CapsLock => Err(()),
            K::NumLock => Err(()),
            K::ScrollLock => Err(()),
            K::PrintScreen => Err(()),
            K::Pause => Err(()),
            K::Menu => Err(()),
            K::KeypadBegin => Err(()),
            K::Media(_) => Err(()),
            K::Modifier(M::LeftShift | M::RightShift) => Ok(Shift),
            K::Modifier(M::LeftControl | M::RightControl) => Ok(Ctrl),
            K::Modifier(M::LeftAlt | M::RightAlt) => Ok(Alt),
            K::Modifier(M::LeftSuper | M::RightSuper) => Ok(Icon),
            K::Modifier(_) => Err(()),
        }
    }
}

impl From<crossterm::event::MouseButton> for crate::MouseButton {
    fn from(button: crossterm::event::MouseButton) -> crate::MouseButton {
        match button {
            crossterm::event::MouseButton::Left => crate::MouseButton::Left,
            crossterm::event::MouseButton::Right => crate::MouseButton::Right,
            crossterm::event::MouseButton::Middle => crate::MouseButton::Middle,
        }
    }
}
