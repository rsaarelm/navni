use std::{
    collections::VecDeque,
    future::Future,
    io::Write,
    pin::Pin,
    sync::{Mutex, OnceLock},
    time::{Duration, SystemTime},
};

use crossterm::{cursor, event, queue, style, terminal};
use rustc_hash::FxHashSet as HashSet;
use signal_hook::{consts::SIGTERM, iterator::Signals};

use crate::{
    CharCell, Key, KeyTyped, MouseState, Rgba, X256Color, FRAME_DURATION,
};

pub static RUNTIME: OnceLock<Mutex<Runtime>> = OnceLock::new();

pub static mut FUTURE: Option<Pin<Box<dyn Future<Output = ()>>>> = None;

pub fn with<F, T>(mut f: F) -> T
where
    F: FnMut(&mut Runtime) -> T,
{
    let mut gui = RUNTIME
        .get()
        .expect("backend not initialized")
        .lock()
        .unwrap();
    f(&mut gui)
}

pub struct Runtime {
    pub(crate) keypress: VecDeque<KeyTyped>,
    pub(crate) last_update: f64,
    pub(crate) logical_frame_count: u32,
    prev_buffer: (u32, u32, Vec<CharCell>),
    size: (u32, u32),
    pub(crate) mouse_state: MouseState,
    // Store last frame's projection that needs to be applied to mouse
    // position.
    mouse_transform: MouseTransform,

    focus_lost: bool,
}

struct MouseTransform {
    offset: [i32; 2],
    scale: [i32; 2],
}

impl Default for MouseTransform {
    fn default() -> Self {
        MouseTransform {
            offset: [0, 0],
            scale: [1, 1],
        }
    }
}

impl Drop for Runtime {
    fn drop(&mut self) {
        cleanup();
    }
}

impl Runtime {
    pub fn new() -> Self {
        let mut stdout = std::io::stdout();
        queue!(
            stdout,
            event::EnableMouseCapture,
            event::EnableFocusChange,
            terminal::EnterAlternateScreen,
            cursor::Hide,
        )
        .unwrap();
        terminal::enable_raw_mode().unwrap();
        stdout.flush().unwrap();

        // Watcher thread, call cleanup on SIGTERM.
        std::thread::spawn(move || {
            if let Some(_) = Signals::new([SIGTERM])
                .expect("Failed to register SIGTERM handler")
                .forever()
                .next()
            {
                cleanup();
                std::process::exit(1);
            }
        });

        // Also cleanup on panic.
        std::panic::set_hook(Box::new(|p| {
            cleanup();
            println!("{}", p);
        }));

        let size = if let Ok((width, height)) = terminal::size() {
            (width as u32, height as u32)
        } else {
            (80, 24)
        };

        Runtime {
            keypress: Default::default(),
            last_update: now(),
            logical_frame_count: 1,
            prev_buffer: Default::default(),
            size,
            mouse_state: Default::default(),
            mouse_transform: Default::default(),
            focus_lost: false,
        }
    }

    pub fn draw_pixels(&mut self, w: u32, h: u32, buffer: &[Rgba]) {
        let cells: Vec<CharCell> = (0..w * h / 2)
            .map(|i| {
                let (x, y) = (i % w, (i / w) * 2);
                CharCell::new(
                    '▀',
                    buffer[(x + y * w) as usize],
                    buffer[(x + (y + 1) * w) as usize],
                )
            })
            .collect();

        self.draw_chars(w, h / 2, &cells);

        // Set scaling to account for the fake y axis doubling. Do the setting
        // here because draw_chars will also set inv_scale.
        self.mouse_transform.scale = [1, 2];
    }

    pub fn draw_chars(&mut self, w: u32, h: u32, buffer: &[CharCell]) {
        assert!(buffer.len() == (w * h) as usize);

        self.mouse_transform.scale = [1, 1];

        let mut stdout = std::io::stdout();
        queue!(stdout, terminal::BeginSynchronizedUpdate).unwrap();

        if self.prev_buffer.0 != w || self.prev_buffer.1 != h {
            // Clear the screen after a resize.
            queue!(stdout, terminal::Clear(terminal::ClearType::All),).unwrap();
        }

        // Center the buffer if it's smaller than the screen.
        let x_offset = if w < self.size.0 {
            (self.size.0 - w) / 2
        } else {
            0
        };

        let y_offset = if h < self.size.1 {
            (self.size.1 - h) / 2
        } else {
            0
        };

        // Adjust mouse pos for the small buffer.
        self.mouse_transform.offset = [x_offset as i32, y_offset as i32];

        let mut prev_cell = CharCell {
            c: 0xffff,
            ..Default::default()
        };

        let mut made_changes = false;

        for y in 0..h.min(self.size.1) {
            let mut need_goto = true;
            for x in 0..w.min(self.size.0) {
                // Skip drawing cells that didn't change from previous frame.
                if self.prev_buffer.0 == w
                    && self.prev_buffer.1 == h
                    && self.prev_buffer.2[(x + y * w) as usize]
                        == buffer[(x + y * w) as usize]
                {
                    need_goto = true;
                    continue;
                }

                made_changes = true;

                // TODO Write crossterm render in an errorable function instead of having all the unwraps scattered about
                if need_goto {
                    queue!(
                        stdout,
                        cursor::MoveTo(
                            (x + x_offset) as u16,
                            (y + y_offset) as u16
                        )
                    )
                    .unwrap();
                    need_goto = false;
                }

                let cell = buffer[(x + y * w) as usize];

                let color_changed = prev_cell.c == 0xffff
                    || cell.foreground != prev_cell.foreground
                    || cell.background != prev_cell.background;
                prev_cell = cell;

                if color_changed {
                    // Determine terminal ops from color
                    // * System background color as cell foreground marks
                    //   inverse display.
                    // * System foreground as foregound marks no color.
                    // * System colors 8-15 are styled bold.
                    let is_inverse = cell.foreground == X256Color::BACKGROUND
                        && cell.background != X256Color::BACKGROUND;
                    let foreground = if is_inverse {
                        cell.background
                    } else {
                        cell.foreground
                    };

                    queue!(stdout, style::ResetColor).unwrap();
                    if is_inverse {
                        queue!(
                            stdout,
                            style::SetAttribute(style::Attribute::Reverse,)
                        )
                        .unwrap();
                    } else if cell.background != X256Color::BACKGROUND {
                        queue!(
                            stdout,
                            style::SetBackgroundColor(style::Color::AnsiValue(
                                cell.background.0
                            ),)
                        )
                        .unwrap();
                    }
                    if foreground != X256Color::FOREGROUND
                        && foreground != X256Color::BOLD_FOREGROUND
                    {
                        queue!(
                            stdout,
                            style::SetForegroundColor(style::Color::AnsiValue(
                                foreground.0
                            ),)
                        )
                        .unwrap();
                    }
                    if foreground.0 >= 8 && foreground.0 < 16 && !is_inverse {
                        queue!(
                            stdout,
                            style::SetAttribute(style::Attribute::Bold,)
                        )
                        .unwrap();
                    }
                }

                if let Some(c) = char::from_u32(cell.c as u32) {
                    let c = match c {
                        '\0' => ' ',
                        c => c,
                    };
                    print!("{c}");
                }
            }
        }

        queue!(stdout, terminal::EndSynchronizedUpdate).unwrap();
        if made_changes {
            stdout.flush().unwrap();
        }

        self.prev_buffer = (w, h, buffer.to_vec());
    }

    pub fn pixel_resolution(&self) -> (u32, u32) {
        // Block pseudopixel size multipliers.
        (self.size.0, self.size.1 * 2)
    }

    pub fn char_resolution(&self) -> (u32, u32) {
        self.size
    }

    pub fn is_down(&self, _key: Key) -> bool {
        // TTY doesn't support this, just assume all pressed keys are
        // immediately released.
        false
    }

    fn resize(&mut self, w: u32, h: u32) {
        self.prev_buffer = Default::default();
        self.size = (w, h);
    }

    pub fn process_event(&mut self, event: event::Event) {
        self.wake_up();
        match event {
            event::Event::Key(k) => {
                if let Ok(k) = KeyTyped::try_from(k) {
                    self.keypress.push_back(k);
                }
            }
            event::Event::Mouse(event::MouseEvent {
                kind,
                column,
                row,
                ..
            }) => {
                let (x, y) = (column as i32, row as i32);
                match kind {
                    event::MouseEventKind::Down(button) => {
                        self.mouse_state.button_down(button.into());
                    }
                    event::MouseEventKind::Up(button) => {
                        self.mouse_state.button_up(button.into());
                    }
                    event::MouseEventKind::Drag(_) => {
                        *self.mouse_state.cursor_pos_mut() =
                            self.transform_mouse_pos([x, y]);
                    }
                    event::MouseEventKind::Moved => {
                        *self.mouse_state.cursor_pos_mut() =
                            self.transform_mouse_pos([x, y]);
                    }
                    event::MouseEventKind::ScrollDown => {
                        self.mouse_state.scroll(0, 1);
                    }
                    event::MouseEventKind::ScrollUp => {
                        self.mouse_state.scroll(0, -1);
                    }
                    event::MouseEventKind::ScrollLeft => {
                        self.mouse_state.scroll(-1, 0);
                    }
                    event::MouseEventKind::ScrollRight => {
                        self.mouse_state.scroll(1, 0);
                    }
                }
            }
            event::Event::Resize(w, h) => {
                // Record new canvas size.
                self.resize(w as _, h as _);
            }
            event::Event::FocusGained => {
                // No-op, entering the event handler wakes you up.
            }
            event::Event::FocusLost => {
                // Go to sleep.
                self.focus_lost = true;
            }
            event::Event::Paste(_) => {}
        }
    }

    pub fn process_events(&mut self) {
        // TODO Better error handling when processing crossterm events

        // Process immediately available events.
        // If focus is currently lost, don't poll for events but just enter
        // the blocking event read until an event comes in and wakes the
        // program.
        while self.focus_lost
            || event::poll(Duration::from_secs(0)).unwrap_or(false)
        {
            self.process_event(event::read().unwrap());
        }

        // If there is time left in the frame, sleep and wait for events.
        let mut elapsed = self.elapsed_since_last_update();
        let dt = FRAME_DURATION.as_secs_f64();
        while elapsed < dt {
            if event::poll(Duration::from_secs_f64(dt - elapsed))
                .unwrap_or(false)
            {
                self.process_event(event::read().unwrap());
            } else {
                break;
            }
            elapsed = self.elapsed_since_last_update();
        }
    }

    pub fn elapsed_since_last_update(&self) -> f64 {
        now() - self.last_update
    }

    fn transform_mouse_pos(&self, [x, y]: [i32; 2]) -> [i32; 2] {
        let [ox, oy] = self.mouse_transform.offset;
        let [sx, sy] = self.mouse_transform.scale;
        [(x - ox) * sx, (y - oy) * sy]
    }

    fn wake_up(&mut self) {
        if self.focus_lost {
            self.focus_lost = false;
            self.last_update = now();
        }
    }
}

pub fn cleanup() {
    let mut stdout = std::io::stdout();
    queue!(
        stdout,
        style::ResetColor,
        cursor::Show,
        terminal::LeaveAlternateScreen,
        event::DisableFocusChange,
        event::DisableMouseCapture
    )
    .unwrap();
    terminal::disable_raw_mode().unwrap();
    stdout.flush().unwrap();
}

fn now() -> f64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
}
