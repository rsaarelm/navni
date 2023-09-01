use std::{fmt, str::FromStr};

use anyhow::anyhow;
use serde_with::{DeserializeFromStr, SerializeDisplay};

/// Key event value.
///
/// NB. shift modifier is always set to false when `key` is `Char(_)`, since
/// shift is redundant with the capitalization of the printable character and
/// adding the flag makes is difficult to interoperate with Navni backends
/// that do not capture the shift modifier.
#[derive(
    Copy,
    Clone,
    Default,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Debug,
    SerializeDisplay,
    DeserializeFromStr,
)]
pub struct KeyTyped {
    key: Key,
    mods: KeyMods,
}

impl From<KeyMods> for KeyTyped {
    fn from(mods: KeyMods) -> Self {
        KeyTyped {
            mods,
            ..Default::default()
        }
    }
}

impl fmt::Display for KeyTyped {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.mods.logo {
            write!(f, "D-")?;
        }
        if self.mods.alt {
            write!(f, "A-")?;
        }
        if self.mods.ctrl {
            write!(f, "C-")?;
        }
        // Explicitly print out shift for non-printable keys.
        // Otherwise it can be inferred from the char being capitalized.
        if self.mods.shift && !self.key.is_printable() {
            write!(f, "S-")?;
        }
        write!(f, "{}", self.key)
    }
}

impl FromStr for KeyTyped {
    type Err = anyhow::Error;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        let mut ret = KeyTyped::default();

        loop {
            if s.starts_with("D-") {
                ret.mods.logo = true;
                s = &s[2..];
            // Support the old-timey "META" designation too.
            } else if s.starts_with("A-") || s.starts_with("M-") {
                ret.mods.alt = true;
                s = &s[2..];
            } else if s.starts_with("C-") {
                ret.mods.ctrl = true;
                s = &s[2..];
            } else if s.starts_with("S-") {
                ret.mods.shift = true;
                s = &s[2..];
            } else {
                // Out of prefixes, parse the actual key or go bust.
                ret.key = s.parse()?;
                if ret.key.is_printable() && ret.mods.shift {
                    // Printable keys shouldn't have shift status specified.
                    return Err(anyhow!("Shift modifier on printable key"));
                }
                break;
            }
        }

        Ok(ret)
    }
}

impl KeyTyped {
    pub fn new(key: Key, mods: KeyMods) -> Self {
        if matches!(key, Key::Char(_)) && mods.shift {
            panic!("KeyTyped::new: shift flag on printable key");
        }

        KeyTyped { key, mods }
    }

    pub fn key(&self) -> Key {
        self.key
    }

    pub fn mods(&self) -> KeyMods {
        self.mods
    }

    pub fn is(&self, code: &str) -> bool {
        *self == code.parse().unwrap()
    }

    /// Convenience method, true if a non-modifier key was pressed.
    pub fn is_some(&self) -> bool {
        self.key != Key::None
    }
}

#[derive(Copy, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct KeyMods {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub logo: bool,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
pub enum Key {
    #[default]
    None,
    Up,
    Down,
    Left,
    Right,
    Tab,
    Enter,
    Esc,
    Backspace,
    Delete,
    Insert,
    Home,
    End,
    PageUp,
    PageDown,
    Shift,
    Ctrl,
    Alt,
    Icon,
    /// Function key
    F(u8),
    /// Normal printable character
    Char(char),
}

impl Key {
    pub fn is_printable(&self) -> bool {
        matches!(self, Key::Char(_))
    }

    /// Turn capital letters into lowercase ones so the key value can stand in
    /// for a physical keyboard key.
    pub fn char_to_lowercase(&self) -> Key {
        match self {
            Key::Char(mut c) => {
                c.make_ascii_lowercase();
                Key::Char(c)
            }
            a => *a,
        }
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Key::None => write!(f, "none"),
            Key::Up => write!(f, "Up"),
            Key::Down => write!(f, "Down"),
            Key::Left => write!(f, "Left"),
            Key::Right => write!(f, "Right"),
            Key::Tab => write!(f, "Tab"),
            Key::Enter => write!(f, "Ret"),
            Key::Esc => write!(f, "Esc"),
            Key::Backspace => write!(f, "Bksp"),
            Key::Delete => write!(f, "Del"),
            Key::Insert => write!(f, "Ins"),
            Key::Home => write!(f, "Home"),
            Key::End => write!(f, "End"),
            Key::PageUp => write!(f, "PgUp"),
            Key::PageDown => write!(f, "PgDn"),
            Key::Shift => write!(f, "Shift"),
            Key::Ctrl => write!(f, "Ctrl"),
            Key::Alt => write!(f, "Alt"),
            Key::Icon => write!(f, "Icon"),
            Key::F(n) => write!(f, "F{n}"),
            // NB. Space is printable but not serializable
            Key::Char(' ') => write!(f, "Sp"),
            Key::Char(c) => write!(f, "{c}"),
        }
    }
}

impl FromStr for Key {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 1 {
            let c = s.as_bytes()[0];
            if (33..127).contains(&c) {
                Ok(Key::Char(s.chars().next().unwrap()))
            } else {
                Err(anyhow!("Bad key {s:?}"))
            }
        } else {
            match s {
                "none" => Ok(Key::None),
                "Up" => Ok(Key::Up),
                "Down" => Ok(Key::Down),
                "Left" => Ok(Key::Left),
                "Right" => Ok(Key::Right),
                "Tab" => Ok(Key::Tab),
                "Ret" => Ok(Key::Enter),
                "Esc" => Ok(Key::Esc),
                "Bksp" => Ok(Key::Backspace),
                "Del" => Ok(Key::Delete),
                "Ins" => Ok(Key::Insert),
                "Home" => Ok(Key::Home),
                "End" => Ok(Key::End),
                "PgUp" => Ok(Key::PageUp),
                "PgDn" => Ok(Key::PageDown),
                "Shift" => Ok(Key::Shift),
                "Ctrl" => Ok(Key::Ctrl),
                "Alt" => Ok(Key::Alt),
                "Icon" => Ok(Key::Icon),
                "F1" => Ok(Key::F(1)),
                "F2" => Ok(Key::F(2)),
                "F3" => Ok(Key::F(3)),
                "F4" => Ok(Key::F(4)),
                "F5" => Ok(Key::F(5)),
                "F6" => Ok(Key::F(6)),
                "F7" => Ok(Key::F(7)),
                "F8" => Ok(Key::F(8)),
                "F9" => Ok(Key::F(9)),
                "F10" => Ok(Key::F(10)),
                "F11" => Ok(Key::F(11)),
                "F12" => Ok(Key::F(12)),
                "Sp" => Ok(Key::Char(' ')),
                _ => Err(anyhow!("Bad key {s:?}")),
            }
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}

/// Complex mouse state for IMGUI
#[derive(Copy, Clone, Debug)]
pub enum MouseState {
    /// Mouse hovering over position with buttons unpressed.
    Hover([i32; 2]),
    /// `Drag(p, s, b)` Dragged from `s` to `p` with button `b down.
    Drag([i32; 2], [i32; 2], MouseButton),
    /// `Release(p, s, b)` Button `b` released over `p` after drag from `s`.
    Release([i32; 2], [i32; 2], MouseButton),
    /// Scroll wheel scrolled at position.
    Scroll([i32; 2], i32),
}
use MouseState::*;

impl Default for MouseState {
    fn default() -> Self {
        Hover(Default::default())
    }
}

impl From<MouseState> for [i32; 2] {
    fn from(value: MouseState) -> Self {
        value.cursor_pos()
    }
}

#[allow(dead_code)]
impl MouseState {
    /// Return current mouse cursor position.
    pub fn cursor_pos(&self) -> [i32; 2] {
        match self {
            Hover(p) => *p,
            Drag(p, _, _) => *p,
            Release(p, _, _) => *p,
            Scroll(p, _) => *p,
        }
    }

    pub(crate) fn cursor_pos_mut(&mut self) -> &mut [i32; 2] {
        match self {
            Hover(p) => p,
            Drag(p, _, _) => p,
            Release(p, _, _) => p,
            Scroll(p, _) => p,
        }
    }

    /// Return position where mouse button was first pressed down for `Drag`
    /// and `Release` states.
    pub fn press_pos(&self) -> Option<[i32; 2]> {
        match self {
            Hover(_) => None,
            Drag(_, s, _) => Some(*s),
            Release(_, s, _) => Some(*s),
            Scroll(_, _) => None,
        }
    }

    pub fn scroll_delta(&self) -> i32 {
        if let Scroll(_, z) = self {
            *z
        } else {
            0
        }
    }

    pub(crate) fn button_down(&mut self, button: MouseButton) {
        match self {
            Hover(p) => *self = Drag(*p, *p, button),
            // Ignore new button presses when in pressed state.
            Drag(_, _, _) => {}
            // XXX: Also ignore new presses in release state because we don't
            // want to clobber the release state before the frame is over.
            // This can lead to actually missing input events if the user is
            // releasing and pressing buttons in very quick succession.
            Release(_, _, _) => {}
            Scroll(p, _) => *self = Drag(*p, *p, button),
        }
    }

    pub(crate) fn button_up(&mut self, button: MouseButton) {
        match self {
            Drag(p, s, b) if button == *b => *self = Release(*p, *s, *b),
            _ => {}
        }
    }

    pub(crate) fn scroll(&mut self, z: i32) {
        debug_assert!(z == -1 || z == 1);
        let p = self.cursor_pos();
        *self = Scroll(p, z);
    }

    /// Update called every frame, exits transient `Release` and `Scroll`
    /// states.
    pub(crate) fn frame_update(&mut self) {
        if let Release(p, _, _) | Scroll(p, _) = self {
            *self = Hover(*p);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use quickcheck::{Arbitrary, Gen};
    use quickcheck_macros::quickcheck;

    impl Arbitrary for Key {
        fn arbitrary(g: &mut Gen) -> Key {
            use Key::*;

            // Printable ASCII chars 50 % of the time.
            if bool::arbitrary(g) {
                let n = ((u32::arbitrary(g) % 95) + 32) as u8;
                return Key::Char(n as char);
            }

            // Special keys otherwise.
            let vals = &[
                None,
                Up,
                Down,
                Left,
                Right,
                Tab,
                Enter,
                Esc,
                Backspace,
                Delete,
                Insert,
                Home,
                End,
                PageUp,
                PageDown,
                F(1),
                F(2),
                F(3),
                F(4),
                F(5),
                F(6),
                F(7),
                F(8),
                F(9),
                F(10),
                F(11),
                F(12),
            ];
            *g.choose(vals).unwrap()
        }
    }

    impl Arbitrary for KeyTyped {
        fn arbitrary(g: &mut Gen) -> KeyTyped {
            let key: Key = Key::arbitrary(g);

            let shift = if key.is_printable() {
                false
            } else {
                bool::arbitrary(g)
            };

            KeyTyped::new(
                key,
                KeyMods {
                    shift,
                    ctrl: bool::arbitrary(g),
                    alt: bool::arbitrary(g),
                    logo: bool::arbitrary(g),
                },
            )
        }
    }

    #[quickcheck]
    fn key_typed_parse(typed: KeyTyped) -> bool {
        let s = typed.to_string();
        s.parse::<KeyTyped>().unwrap() == typed
    }
}
