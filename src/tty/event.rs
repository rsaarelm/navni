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
        use crossterm::event::KeyCode;
        use Key::*;

        match key_code {
            KeyCode::Backspace => Ok(Backspace),
            KeyCode::Enter => Ok(Enter),
            KeyCode::Left => Ok(Left),
            KeyCode::Right => Ok(Right),
            KeyCode::Up => Ok(Up),
            KeyCode::Down => Ok(Down),
            KeyCode::Home => Ok(Home),
            KeyCode::End => Ok(End),
            KeyCode::PageUp => Ok(PageUp),
            KeyCode::PageDown => Ok(PageDown),
            KeyCode::Tab => Ok(Tab),
            KeyCode::BackTab => Err(()), // XXX: What's this anyway?
            KeyCode::Delete => Ok(Delete),
            KeyCode::Insert => Ok(Insert),
            KeyCode::F(n) => Ok(F(n)),
            KeyCode::Char(c) => Ok(Char(c)),
            KeyCode::Null => Err(()),
            KeyCode::Esc => Ok(Esc),
            KeyCode::CapsLock => Err(()),
            KeyCode::NumLock => Err(()),
            KeyCode::ScrollLock => Err(()),
            KeyCode::PrintScreen => Err(()),
            KeyCode::Pause => Err(()),
            KeyCode::Menu => Err(()),
            KeyCode::KeypadBegin => Err(()),
            KeyCode::Media(_) => Err(()),
            KeyCode::Modifier(_) => Err(()),
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
