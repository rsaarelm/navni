use std::{
    collections::VecDeque,
    io::Write,
    time::{Duration, SystemTime},
};

use crate::{
    App, Backend, CharCell, Config, Key, KeyTyped, MouseState, Rgba, X256Color,
    FRAME_DURATION, MAX_UPDATES_PER_FRAME,
};
use crossterm::{cursor, event, queue, style, terminal, QueueableCommand};
use signal_hook::{consts::SIGTERM, iterator::Signals};

// Used for hacking up the fake key held detector. This part is fragile, we
// want it to be as low as possible to get better time resolution, but it
// can't be lower than the actual system key repeat delay or continuous
// keypresses will stutter. The values were figured by trial and error of
// seeing how low I can make them on Linux before the signal starts dropping
// while the key is held. They might not work right on all platforms.
//
// Supporting keys held down on TTY side isn't an entirely serious feature.
// It's recommended to stick to games whose input is based on typed keys if
// the TTY target is important.
const KEY_REPEAT_START: Duration = Duration::from_millis(700);
const KEY_REPEAT_CONTINUE: Duration = Duration::from_millis(50);

/// Run an application with the given starting scene and game data using a
/// TTY backend.
pub fn run(_config: &Config, mut app: impl App + 'static) -> ! {
    let mut backend = TtyBackend::new();

    while !backend.quit_requested {
        let dt = backend.elapsed_since_last_update();
        let n = (dt / FRAME_DURATION.as_secs_f64()) as u32;

        app.update(&mut backend, n.min(MAX_UPDATES_PER_FRAME));
        backend.keypress.pop_front();
        backend.mouse_state.frame_update();
        backend.process_events();

        backend.last_update += (n as f64) * FRAME_DURATION.as_secs_f64();
    }

    drop(backend);

    std::process::exit(0);
}

struct TtyBackend {
    keypress: VecDeque<KeyTyped>,
    last_update: f64,
    prev_buffer: (u32, u32, Vec<CharCell>),
    size: (u32, u32),
    mouse_state: MouseState,
    // Store last frame's projection that needs to be applied to mouse
    // position.
    mouse_transform: MouseTransform,

    // Used for fake pressed key tracking.
    last_key: (Key, f64),

    quit_requested: bool,
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

impl Drop for TtyBackend {
    fn drop(&mut self) {
        cleanup();
    }
}

impl TtyBackend {
    fn new() -> Self {
        let mut stdout = std::io::stdout();
        queue!(
            stdout,
            event::EnableMouseCapture,
            event::EnableFocusChange,
            terminal::EnterAlternateScreen,
            cursor::Hide
        )
        .unwrap();
        terminal::enable_raw_mode().unwrap();
        stdout.flush().unwrap();

        // Watcher thread, call cleanup on SIGTERM.
        std::thread::spawn(move || {
            for _ in Signals::new([SIGTERM])
                .expect("Failed to register SIGTERM handler")
                .forever()
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

        TtyBackend {
            keypress: Default::default(),
            last_update: now(),
            prev_buffer: Default::default(),
            size,
            mouse_state: Default::default(),
            mouse_transform: Default::default(),
            last_key: (Key::None, 0.0),
            quit_requested: false,
            focus_lost: false,
        }
    }

    fn resize(&mut self, w: u32, h: u32) {
        self.prev_buffer = Default::default();
        self.size = (w, h);
    }

    fn process_event(&mut self, event: event::Event) {
        self.wake_up();
        match event {
            event::Event::Key(k) => {
                if let Ok(k) = KeyTyped::try_from(k) {
                    self.keypress.push_back(k);
                    let key = k.key().char_to_lowercase();
                    if self.is_down(key) {
                        // Repeat press, low timeout
                        self.last_key = (
                            key,
                            self.last_update
                                + KEY_REPEAT_CONTINUE.as_secs_f64(),
                        );
                    } else {
                        // New press, start with the long repeat start delay.
                        self.last_key = (
                            key,
                            self.last_update + KEY_REPEAT_START.as_secs_f64(),
                        );
                    }
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

    fn process_events(&mut self) {
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

    fn elapsed_since_last_update(&self) -> f64 {
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

impl Backend for TtyBackend {
    fn draw_pixels(&mut self, w: u32, h: u32, buffer: &[Rgba]) {
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

    fn draw_chars(&mut self, w: u32, h: u32, buffer: &[CharCell]) {
        assert!(buffer.len() == (w * h) as usize);

        self.mouse_transform.scale = [1, 1];

        let mut stdout = std::io::stdout();

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

                // TODO 2023-06-24 Write crossterm render in an errorable function instead of having all the unwraps scattered about
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

                    stdout.queue(style::ResetColor).unwrap();
                    if is_inverse {
                        stdout
                            .queue(style::SetAttribute(
                                style::Attribute::Reverse,
                            ))
                            .unwrap();
                    } else if cell.background != X256Color::BACKGROUND {
                        stdout
                            .queue(style::SetBackgroundColor(
                                style::Color::AnsiValue(cell.background.0),
                            ))
                            .unwrap();
                    }
                    if foreground != X256Color::FOREGROUND
                        && foreground != X256Color::BOLD_FOREGROUND
                    {
                        stdout
                            .queue(style::SetForegroundColor(
                                style::Color::AnsiValue(foreground.0),
                            ))
                            .unwrap();
                    }
                    if foreground.0 >= 8 && foreground.0 < 16 && !is_inverse {
                        stdout
                            .queue(style::SetAttribute(style::Attribute::Bold))
                            .unwrap();
                    }

                    stdout.flush().unwrap();
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

        if made_changes {
            stdout.flush().unwrap();
        }

        self.prev_buffer = (w, h, buffer.to_vec());
    }

    fn pixel_resolution(&self) -> (u32, u32) {
        // Block pseudopixel size multipliers.
        (self.size.0, self.size.1 * 2)
    }

    fn char_resolution(&self) -> (u32, u32) {
        self.size
    }

    fn now(&self) -> f64 {
        self.last_update
    }

    fn is_down(&self, key: Key) -> bool {
        self.last_key.0 == key && self.last_update < self.last_key.1
    }

    fn keypress(&self) -> KeyTyped {
        self.keypress.front().copied().unwrap_or_default()
    }

    fn mouse_state(&self) -> MouseState {
        self.mouse_state
    }

    fn quit(&mut self) {
        self.quit_requested = true;
    }
}

fn cleanup() {
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
