impl TryFrom<miniquad::MouseButton> for crate::MouseButton {
    type Error = ();

    fn try_from(value: miniquad::MouseButton) -> Result<Self, Self::Error> {
        use crate::MouseButton::*;
        match value {
            miniquad::MouseButton::Right => Ok(Right),
            miniquad::MouseButton::Left => Ok(Left),
            miniquad::MouseButton::Middle => Ok(Middle),
            miniquad::MouseButton::Unknown => Err(()),
        }
    }
}

impl TryFrom<(miniquad::KeyCode, miniquad::KeyMods)> for crate::KeyTyped {
    type Error = ();

    fn try_from(
        (k, m): (miniquad::KeyCode, miniquad::KeyMods),
    ) -> Result<Self, Self::Error> {
        use crate::{Key, KeyTyped};

        let key = Key::try_from(k)?;
        let mut mods = crate::KeyMods::from((k, m));
        if key.is_printable() {
            mods.shift = false;
        }

        Ok(KeyTyped::new(key, mods))
    }
}

impl From<miniquad::KeyMods> for crate::KeyMods {
    fn from(mods: miniquad::KeyMods) -> Self {
        crate::KeyMods {
            shift: mods.shift,
            ctrl: mods.ctrl,
            alt: mods.alt,
            logo: mods.logo,
        }
    }
}

// A modifier key being pressed down doesn't trip the mod flag yet, so we need
// to handle both here.
impl From<(miniquad::KeyCode, miniquad::KeyMods)> for crate::KeyMods {
    fn from((keycode, mods): (miniquad::KeyCode, miniquad::KeyMods)) -> Self {
        use miniquad::KeyCode;

        let mut ret = crate::KeyMods::from(mods);

        match keycode {
            KeyCode::LeftShift | KeyCode::RightShift => ret.shift = true,
            KeyCode::LeftControl | KeyCode::RightControl => ret.ctrl = true,
            KeyCode::LeftAlt | KeyCode::RightAlt => ret.alt = true,
            KeyCode::LeftSuper | KeyCode::RightSuper => ret.logo = true,
            _ => {}
        }

        ret
    }
}

impl TryFrom<miniquad::KeyCode> for crate::Key {
    type Error = ();

    fn try_from(key_code: miniquad::KeyCode) -> Result<Self, Self::Error> {
        use crate::Key::*;
        use miniquad::KeyCode;

        match key_code {
            KeyCode::Space => Ok(Char(' ')),
            KeyCode::Apostrophe => Ok(Char('\'')),
            KeyCode::Comma => Ok(Char(',')),
            KeyCode::Minus => Ok(Char('-')),
            KeyCode::Period => Ok(Char('.')),
            KeyCode::Slash => Ok(Char('/')),
            KeyCode::Key0 => Ok(Char('0')),
            KeyCode::Key1 => Ok(Char('1')),
            KeyCode::Key2 => Ok(Char('2')),
            KeyCode::Key3 => Ok(Char('3')),
            KeyCode::Key4 => Ok(Char('4')),
            KeyCode::Key5 => Ok(Char('5')),
            KeyCode::Key6 => Ok(Char('6')),
            KeyCode::Key7 => Ok(Char('7')),
            KeyCode::Key8 => Ok(Char('8')),
            KeyCode::Key9 => Ok(Char('9')),
            KeyCode::Semicolon => Ok(Char(';')),
            KeyCode::Equal => Ok(Char('=')),
            KeyCode::A => Ok(Char('a')),
            KeyCode::B => Ok(Char('b')),
            KeyCode::C => Ok(Char('c')),
            KeyCode::D => Ok(Char('d')),
            KeyCode::E => Ok(Char('e')),
            KeyCode::F => Ok(Char('f')),
            KeyCode::G => Ok(Char('g')),
            KeyCode::H => Ok(Char('h')),
            KeyCode::I => Ok(Char('i')),
            KeyCode::J => Ok(Char('j')),
            KeyCode::K => Ok(Char('k')),
            KeyCode::L => Ok(Char('l')),
            KeyCode::M => Ok(Char('m')),
            KeyCode::N => Ok(Char('n')),
            KeyCode::O => Ok(Char('o')),
            KeyCode::P => Ok(Char('p')),
            KeyCode::Q => Ok(Char('q')),
            KeyCode::R => Ok(Char('r')),
            KeyCode::S => Ok(Char('s')),
            KeyCode::T => Ok(Char('t')),
            KeyCode::U => Ok(Char('u')),
            KeyCode::V => Ok(Char('v')),
            KeyCode::W => Ok(Char('w')),
            KeyCode::X => Ok(Char('x')),
            KeyCode::Y => Ok(Char('y')),
            KeyCode::Z => Ok(Char('z')),
            KeyCode::LeftBracket => Ok(Char('[')),
            KeyCode::Backslash => Ok(Char('\\')),
            KeyCode::RightBracket => Ok(Char(']')),
            KeyCode::GraveAccent => Ok(Char('`')),
            KeyCode::World1 => Err(()),
            KeyCode::World2 => Err(()),
            KeyCode::Escape => Ok(Esc),
            KeyCode::Enter => Ok(Enter),
            KeyCode::Tab => Ok(Tab),
            KeyCode::Backspace => Ok(Backspace),
            KeyCode::Insert => Ok(Insert),
            KeyCode::Delete => Ok(Delete),
            KeyCode::Right => Ok(Right),
            KeyCode::Left => Ok(Left),
            KeyCode::Down => Ok(Down),
            KeyCode::Up => Ok(Up),
            KeyCode::PageUp => Ok(PageUp),
            KeyCode::PageDown => Ok(PageDown),
            KeyCode::Home => Ok(Home),
            KeyCode::End => Ok(End),
            KeyCode::CapsLock => Err(()),
            KeyCode::ScrollLock => Err(()),
            KeyCode::NumLock => Err(()),
            KeyCode::PrintScreen => Err(()),
            KeyCode::Pause => Err(()),
            KeyCode::F1 => Ok(F(1)),
            KeyCode::F2 => Ok(F(2)),
            KeyCode::F3 => Ok(F(3)),
            KeyCode::F4 => Ok(F(4)),
            KeyCode::F5 => Ok(F(5)),
            KeyCode::F6 => Ok(F(6)),
            KeyCode::F7 => Ok(F(7)),
            KeyCode::F8 => Ok(F(8)),
            KeyCode::F9 => Ok(F(9)),
            KeyCode::F10 => Ok(F(10)),
            KeyCode::F11 => Ok(F(11)),
            KeyCode::F12 => Ok(F(12)),
            KeyCode::F13 => Ok(F(13)),
            KeyCode::F14 => Ok(F(14)),
            KeyCode::F15 => Ok(F(15)),
            KeyCode::F16 => Ok(F(16)),
            KeyCode::F17 => Ok(F(17)),
            KeyCode::F18 => Ok(F(18)),
            KeyCode::F19 => Ok(F(19)),
            KeyCode::F20 => Ok(F(20)),
            KeyCode::F21 => Ok(F(21)),
            KeyCode::F22 => Ok(F(22)),
            KeyCode::F23 => Ok(F(23)),
            KeyCode::F24 => Ok(F(24)),
            KeyCode::F25 => Ok(F(25)),
            KeyCode::Kp0 => Ok(Char('0')),
            KeyCode::Kp1 => Ok(Char('1')),
            KeyCode::Kp2 => Ok(Char('2')),
            KeyCode::Kp3 => Ok(Char('3')),
            KeyCode::Kp4 => Ok(Char('4')),
            KeyCode::Kp5 => Ok(Char('5')),
            KeyCode::Kp6 => Ok(Char('6')),
            KeyCode::Kp7 => Ok(Char('7')),
            KeyCode::Kp8 => Ok(Char('8')),
            KeyCode::Kp9 => Ok(Char('9')),
            KeyCode::KpDecimal => Ok(Char('.')),
            KeyCode::KpDivide => Ok(Char('/')),
            KeyCode::KpMultiply => Ok(Char('*')),
            KeyCode::KpSubtract => Ok(Char('-')),
            KeyCode::KpAdd => Ok(Char('+')),
            KeyCode::KpEnter => Ok(Enter),
            KeyCode::KpEqual => Ok(Char('=')),
            KeyCode::LeftShift => Ok(Shift),
            KeyCode::LeftControl => Ok(Ctrl),
            KeyCode::LeftAlt => Ok(Alt),
            KeyCode::LeftSuper => Ok(Icon),
            KeyCode::RightShift => Ok(Shift),
            KeyCode::RightControl => Ok(Ctrl),
            KeyCode::RightAlt => Ok(Alt),
            KeyCode::RightSuper => Ok(Icon),
            KeyCode::Menu => Err(()),
            KeyCode::Unknown => Err(()),
        }
    }
}
