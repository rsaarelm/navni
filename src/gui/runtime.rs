use std::{
    collections::VecDeque,
    future::Future,
    pin::Pin,
    sync::{Mutex, OnceLock},
};

use miniquad::*;
use rustc_hash::FxHashSet as HashSet;

use crate::{
    FontSheet, Key, KeyTyped, MouseState, Rgba, X256Color, FRAME_DURATION,
};

pub static RUNTIME: OnceLock<Mutex<Runtime>> = OnceLock::new();

pub static mut FUTURE: Option<Pin<Box<dyn Future<Output = ()>>>> = None;

const BINDINGS_PIXEL_BUFFER_INDEX: usize = 0;
const BINDINGS_FONT_SHEET_INDEX: usize = 1;
const BINDINGS_TEXT_BUFFER_INDEX: usize = 2;
const BINDINGS_FOREGROUND_COLOR_INDEX: usize = 3;
const BINDINGS_BACKGROUND_COLOR_INDEX: usize = 4;

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

/// Dummy type to pass to miniquad, just accesses the Runtime singleton.
pub struct Handle;

impl EventHandler for Handle {
    fn update(&mut self) {
        // TODO: Should the future be updated here instead of in draw?
    }

    fn draw(&mut self) {
        let n_frames = with(|r| {
            let now = date::now();
            let elapsed = now - r.last_update;

            (elapsed / FRAME_DURATION.as_secs_f64()) as u32
        });

        // One or more frames have elapsed, tick the state machine.
        if n_frames > 0 {
            with(|r| {
                r.logical_frame_count = n_frames;
                r.last_update += n_frames as f64 * FRAME_DURATION.as_secs_f64();
            });

            // Poll on the application future, this moves application logic
            // forward to the point where it awaits for frame change.
            //
            // If the future completes, the application run has ended and
            // we should quit.
            if unsafe { crate::exec::poll(FUTURE.as_mut().unwrap()) }.is_some()
            {
                window::quit();
                return;
            }

            // Update input stack machines.
            with(|r| {
                r.keypress.pop_front();
                r.mouse_state.frame_update();
            });
        }
    }

    fn key_down_event(
        &mut self,
        keycode: KeyCode,
        keymods: KeyMods,
        repeat: bool,
    ) {
        with(|r| {
            if let Ok(typed) = KeyTyped::try_from((keycode, keymods, repeat)) {
                // Miniquad won't give char_event for non-printable keys, but we
                // want those too. Emit them here.
                if !matches!(typed.key(), Key::Char(_)) {
                    r.keypress.push_back(typed);
                }

                r.key_down.insert(typed.key().char_to_lowercase());
            }
        })
    }

    fn key_up_event(&mut self, keycode: KeyCode, keymods: KeyMods) {
        if let Ok(typed) = KeyTyped::try_from((keycode, keymods, false)) {
            with(|r| r.key_down.remove(&typed.key().char_to_lowercase()));
        }
    }

    fn char_event(
        &mut self,
        mut character: char,
        keymods: KeyMods,
        repeat: bool,
    ) {
        let c = character as u32;
        if c > 1 << 15 {
            // I think these are key release events or something?
            return;
        }

        if !keymods.ctrl && c < 32 {
            // Some unprintable keys like enter try to get through here...
            return;
        }

        if keymods.ctrl && c < 32 {
            // CTRL chars show up as nonprintable control codes on windows.

            // Adjust letters to lowercase or uppercase based on shift state,
            // the rest into characters from the uppercase letter column.
            let letter_base = if keymods.shift { '@' } else { '`' };
            let base = if c <= 26 { letter_base } else { '@' } as u32;
            character = char::from_u32(base + c).unwrap();
        }

        let mut mods = crate::KeyMods::from(keymods);
        // Shift must be false with printable keys.
        mods.shift = false;
        let typed = KeyTyped::new(Key::Char(character), mods, repeat);
        with(|r| r.keypress.push_back(typed));
    }

    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        with(|r| *r.mouse_state.cursor_pos_mut() = r.transform_mouse_pos(x, y));
    }

    fn mouse_wheel_event(&mut self, x: f32, y: f32) {
        let (u, v) = ((-x as i32).signum(), (-y as i32).signum());
        if u != 0 || v != 0 {
            with(|r| r.mouse_state.scroll(u, v));
        }
    }

    fn mouse_button_down_event(
        &mut self,
        button: miniquad::MouseButton,
        _x: f32,
        _y: f32,
    ) {
        if let Ok(button) = button.try_into() {
            with(|r| r.mouse_state.button_down(button));
        }
    }

    fn mouse_button_up_event(
        &mut self,
        button: miniquad::MouseButton,
        _x: f32,
        _y: f32,
    ) {
        if let Ok(button) = button.try_into() {
            with(|r| r.mouse_state.button_up(button));
        }
    }
}

struct Font {
    // Lookup table from codepoints to font sheet.
    //
    // All undefined codepoints will point to 0xff.
    char_lookup: Vec<u8>,
    font_size: (u32, u32),
}

pub struct Runtime {
    gl: GlContext,

    pixels_pipeline: Pipeline,
    chars_pipeline: Pipeline,
    bindings: Bindings,

    system_colors: [Rgba; 16],
    font: Option<Font>,

    pub(crate) last_update: f64,
    /// How many logical frames were covered in last frame.
    ///
    /// This goes up if the app is slow and drops frames.
    pub(crate) logical_frame_count: u32,

    pub(crate) key_down: HashSet<Key>,
    pub(crate) mouse_state: MouseState,
    pub(crate) keypress: VecDeque<KeyTyped>,

    mouse_offset: (i32, i32),
    mouse_scale: (i32, i32),
}

impl Runtime {
    pub fn new() -> Self {
        let mut gl = GlContext::new();

        #[rustfmt::skip]
        let vertices: [Vertex; 4] = [
            Vertex::new(-1.0, -1.0,  0.0,  1.0),
            Vertex::new( 1.0, -1.0,  1.0,  1.0),
            Vertex::new( 1.0,  1.0,  1.0,  0.0),
            Vertex::new(-1.0,  1.0,  0.0,  0.0),
        ];

        let vertex_buffer = gl.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Dynamic,
            BufferSource::slice(&vertices),
        );

        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
        let index_buffer = gl.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&indices),
        );

        // Placeholder until the font gets initialized.
        let font_image = create_texture::<u8>(&mut gl, 0, 0, &[]);

        // Pixel buffer pixels.
        let pixels = create_texture::<u8>(&mut gl, 0, 0, &[]);

        // Charcell buffer characters.
        let text = create_texture::<u8>(&mut gl, 0, 0, &[]);
        // Charcell buffer foreground colors.
        let foreground_color = create_texture::<u8>(&mut gl, 0, 0, &[]);
        // Charcell buffer background colors.
        let background_color = create_texture::<u8>(&mut gl, 0, 0, &[]);

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer,
            // This layout must match the BINDINGS_*_INDEX constants
            images: vec![
                pixels,
                font_image,
                text,
                foreground_color,
                background_color,
            ],
        };

        let pixels_shader = gl
            .new_shader(
                match gl.info().backend {
                    miniquad::Backend::OpenGl => ShaderSource::Glsl {
                        vertex: VERTEX_SHADER,
                        fragment: PIXEL_FRAGMENT_SHADER,
                    },
                    miniquad::Backend::Metal => unimplemented!(),
                },
                ShaderMeta {
                    images: vec!["pixels".to_string()],
                    uniforms: UniformBlockLayout {
                        uniforms: vec![UniformDesc::new(
                            "terminal_size",
                            UniformType::Float2,
                        )],
                    },
                },
            )
            .unwrap();

        let chars_shader = gl
            .new_shader(
                match gl.info().backend {
                    miniquad::Backend::OpenGl => ShaderSource::Glsl {
                        vertex: VERTEX_SHADER,
                        fragment: CHAR_FRAGMENT_SHADER,
                    },
                    miniquad::Backend::Metal => unimplemented!(),
                },
                ShaderMeta {
                    images: vec![
                        "pixels".to_string(),
                        "font_image".to_string(),
                        "text".to_string(),
                        "foreground_color".to_string(),
                        "background_color".to_string(),
                    ],
                    uniforms: UniformBlockLayout {
                        uniforms: vec![UniformDesc::new(
                            "terminal_size",
                            UniformType::Float2,
                        )],
                    },
                },
            )
            .unwrap();

        let pixels_pipeline = gl.new_pipeline(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float2),
                VertexAttribute::new("uv", VertexFormat::Float2),
            ],
            pixels_shader,
            Default::default(),
        );

        let chars_pipeline = gl.new_pipeline(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float2),
                VertexAttribute::new("uv", VertexFormat::Float2),
            ],
            chars_shader,
            Default::default(),
        );

        let last_update = date::now();

        Runtime {
            gl,
            pixels_pipeline,
            chars_pipeline,
            bindings,
            font: None,
            system_colors: std::array::from_fn(|i| X256Color(i as u8).into()),
            last_update,
            logical_frame_count: 1,
            key_down: Default::default(),
            mouse_state: Default::default(),
            keypress: Default::default(),
            mouse_offset: Default::default(),
            mouse_scale: (1, 1),
        }
    }

    fn get_font(&mut self) -> &Font {
        if self.font.is_none() {
            self.set_font(&FontSheet::default());
        }
        self.font.as_ref().unwrap()
    }

    pub fn set_font(&mut self, sheet: &FontSheet) {
        let font_size;

        let (font_image, font_chars) = {
            assert!(sheet.image.width() % 16 == 0 && sheet.image.height() % 16 == 0,
                    "Font sheet dimensions aren't a multiple of 16, sheet must be a 16x16 grid");
            font_size = (sheet.image.width() / 16, sheet.image.height() / 16);
            (
                create_texture(
                    &mut self.gl,
                    sheet.image.width(),
                    sheet.image.height(),
                    sheet.image.as_raw(),
                ),
                sheet.chars,
            )
        };

        let mut char_lookup = vec![0xff; 0x10000];
        for (i, &c) in font_chars.iter().enumerate() {
            char_lookup[c as usize] = i as u8;
        }

        self.bindings.images[BINDINGS_FONT_SHEET_INDEX] = font_image;
        self.font = Some(Font {
            char_lookup,
            font_size,
        });
    }

    pub fn set_palette(&mut self, palette: &[Rgba; 16]) {
        self.system_colors = *palette;
    }

    pub fn draw_pixels(&mut self, w: u32, h: u32, buffer: &[crate::Rgba]) {
        assert!(buffer.len() == (w * h) as usize);

        if w == 0 || h == 0 {
            return;
        }

        self.gl.texture_resize(
            self.bindings.images[BINDINGS_PIXEL_BUFFER_INDEX],
            w,
            h,
            Some(bytes(buffer)),
        );

        self.gl.begin_default_pass(Default::default());
        self.gl.apply_pipeline(&self.pixels_pipeline);
        self.gl.apply_bindings(&self.bindings);

        self.pixel_canvas_scale(w, h);

        self.gl.apply_uniforms(UniformsSource::table(&Uniforms {
            terminal_size: (0.0, 0.0),
        }));

        self.clear();
        self.gl.draw(0, 6, 1);
        self.gl.end_render_pass();
        self.gl.commit_frame();
    }

    pub fn draw_chars(&mut self, w: u32, h: u32, buffer: &[crate::CharCell]) {
        assert!(buffer.len() == (w * h) as usize);

        if w == 0 || h == 0 {
            return;
        }

        // Make sure font is initialized.
        self.get_font();

        // TODO: Make the channel-buffers reused members of Runtime so I
        // don't need to heap-allocate new ones every frame.
        //
        // If I want to really optimize this, maybe the X256 palette could be
        // embedded in the shader and the shader could operate directly on
        // CharCell data...

        let chars: Vec<u32> = {
            let lookup = &self.get_font().char_lookup;
            buffer.iter().map(|a| lookup[a.c as usize] as u32).collect()
        };
        let fore: Vec<Rgba> = buffer
            .iter()
            .map(|a| self.convert_color(a.foreground))
            .collect();
        let back: Vec<Rgba> = buffer
            .iter()
            .map(|a| self.convert_color(a.background))
            .collect();

        self.gl.texture_resize(
            self.bindings.images[BINDINGS_TEXT_BUFFER_INDEX],
            w,
            h,
            Some(bytes(&chars)),
        );
        self.gl.texture_resize(
            self.bindings.images[BINDINGS_FOREGROUND_COLOR_INDEX],
            w,
            h,
            Some(bytes(&fore)),
        );
        self.gl.texture_resize(
            self.bindings.images[BINDINGS_BACKGROUND_COLOR_INDEX],
            w,
            h,
            Some(bytes(&back)),
        );

        self.gl.begin_default_pass(Default::default());
        self.gl.apply_pipeline(&self.chars_pipeline);
        self.gl.apply_bindings(&self.bindings);

        self.char_canvas_scale(w, h);

        self.gl.apply_uniforms(UniformsSource::table(&Uniforms {
            terminal_size: (w as f32, h as f32),
        }));

        self.clear();
        self.gl.draw(0, 6, 1);
        self.gl.end_render_pass();
        self.gl.commit_frame();
    }

    pub fn pixel_resolution(&self) -> (u32, u32) {
        let (w, h) = window::screen_size();
        (w as u32, h as u32)
    }

    pub fn char_resolution(&mut self, max_w: u32, max_h: u32) -> (u32, u32) {
        let (w, h) = self.pixel_resolution();
        let size = self.get_font().font_size;

        let mut n = 1; // Scale factor.
        if max_w > 0 {
            while (w / n) / size.0 > max_w {
                n += 1;
            }
        }

        if max_h > 0 {
            while (h / n) / size.1 > max_h {
                n += 1;
            }
        }

        ((w / n) / size.0, (h / n) / size.1)
    }

    fn pixel_canvas_scale(
        &mut self,
        buffer_w: u32,
        buffer_h: u32,
    ) -> (f32, f32) {
        let (w, h) = window::screen_size();

        let (buffer_w, buffer_h) = (buffer_w as f32, buffer_h as f32);
        let (mut x, mut y) = (buffer_w, buffer_h);

        let mut s = 1;
        while (x + buffer_w) <= w && (y + buffer_h) <= h {
            x += buffer_w;
            y += buffer_h;
            s += 1;
        }

        self.mouse_scale = (s, s);
        self.mouse_offset = ((w - x) as i32 / 2, (h - y) as i32 / 2);

        let x0 = -x / w;
        let y0 = -y / h;

        // Add pixel artifact preventing fudge factors if window dimensions
        // are odd.
        let x1 = (x - if w as u32 % 2 == 1 { 1.0 } else { 0.0 }) / w;
        let y1 = (y - if h as u32 % 2 == 1 { 1.0 } else { 0.0 }) / h;

        let vertices: [Vertex; 4] = [
            Vertex::new(x0, y0, 0.0, 1.0),
            Vertex::new(x1, y0, 1.0, 1.0),
            Vertex::new(x1, y1, 1.0, 0.0),
            Vertex::new(x0, y1, 0.0, 0.0),
        ];

        self.gl.buffer_update(
            self.bindings.vertex_buffers[0],
            BufferSource::slice(&vertices),
        );

        (x / w, y / h)
    }

    fn clear(&mut self) {
        let c = self.system_colors[0];
        let c = (
            c.r as f32 / 255.0,
            c.g as f32 / 255.0,
            c.b as f32 / 255.0,
            c.a as f32 / 255.0,
        );
        self.gl.clear(Some(c), None, None);
    }

    fn char_canvas_scale(
        &mut self,
        buffer_w: u32,
        buffer_h: u32,
    ) -> (f32, f32) {
        let size = self.get_font().font_size;
        let (buffer_w, buffer_h) = (buffer_w * size.0, buffer_h * size.1);

        let ret = self.pixel_canvas_scale(buffer_w, buffer_h);
        self.mouse_scale.0 *= size.0 as i32;
        self.mouse_scale.1 *= size.1 as i32;
        ret
    }

    fn transform_mouse_pos(&self, x: f32, y: f32) -> [i32; 2] {
        [
            (x as i32 - self.mouse_offset.0) / self.mouse_scale.0,
            (y as i32 - self.mouse_offset.1) / self.mouse_scale.1,
        ]
    }

    fn convert_color(&self, c: X256Color) -> Rgba {
        if c.0 < 16 {
            self.system_colors[c.0 as usize]
        } else {
            c.into()
        }
    }
}

fn create_texture<T>(
    gl: &mut GlContext,
    w: u32,
    h: u32,
    data: &[T],
) -> TextureId {
    let ret = gl.new_texture_from_data_and_format(
        bytes(data),
        TextureParams {
            format: TextureFormat::RGBA8,
            min_filter: FilterMode::Nearest,
            mag_filter: FilterMode::Nearest,
            width: w as _,
            height: h as _,
            ..Default::default()
        },
    );
    ret
}

fn bytes<T>(v: &[T]) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(
            v.as_ptr() as *const u8,
            std::mem::size_of_val(v),
        )
    }
}

#[repr(C)]
struct Vec2 {
    x: f32,
    y: f32,
}

#[repr(C)]
struct Vertex {
    pos: Vec2,
    uv: Vec2,
}

impl Vertex {
    pub fn new(x: f32, y: f32, u: f32, v: f32) -> Self {
        Vertex {
            pos: Vec2 { x, y },
            uv: Vec2 { x: u, y: v },
        }
    }
}

pub const VERTEX_SHADER: &str = r#"\
#version 100
attribute vec2 pos;
attribute vec2 uv;

varying lowp vec2 texcoord;
void main() {
    gl_Position = vec4(pos, 0, 1);
    texcoord = uv;
}"#;

pub const PIXEL_FRAGMENT_SHADER: &str = r#"\
#version 100
varying lowp vec2 texcoord;
uniform sampler2D pixels;

void main() {
    gl_FragColor = texture2D(pixels, texcoord);
}"#;

pub const CHAR_FRAGMENT_SHADER: &str = r#"\
#version 100
varying lowp vec2 texcoord;
uniform sampler2D font_image;
uniform sampler2D text;
uniform sampler2D foreground_color;
uniform sampler2D background_color;

uniform lowp vec2 terminal_size;

void main() {
    // Reinterpret color channel in text texture as ASCII byte.
    lowp vec4 ch = texture2D(text, texcoord);
    int chr = int(ch.x * 255.0 + 0.5);

    // Top left corner of this character in font sheet.
    lowp float row = float(chr / 16) / 16.0;
    lowp float col = mod(float(chr), 16.0) / 16.0;

    // Pull texel for these coordinates from the character in the font
    // sheet.
    lowp vec4 texel = texture2D(
        font_image,
        vec2(
            col + mod(texcoord.x * terminal_size.x / 16.0, (1.0 / 16.0)),
            row + mod(texcoord.y * terminal_size.y / 16.0, (1.0 / 16.0))));

    // Modulate with background and foreground colors.
    gl_FragColor =
        texel * texture2D(foreground_color, texcoord) +
        (1.0 - texel) * texture2D(background_color, texcoord);
}"#;

#[repr(C)]
pub struct Uniforms {
    pub terminal_size: (f32, f32),
}
