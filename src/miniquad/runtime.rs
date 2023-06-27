use std::collections::VecDeque;

use crate::{
    scene::SceneStack, Backend, Config, FontSheet, Key, KeyTyped, MouseState,
    Rgba, Scene, X256Color, FRAME_DURATION, MAX_UPDATES_PER_FRAME,
};
use miniquad::*;
use rustc_hash::FxHashSet as HashSet;

pub fn run<T: 'static>(
    config: &Config,
    game: T,
    scene: impl Scene<T> + 'static,
) -> ! {
    let config = config.clone();
    let mut mq_config = conf::Conf::default();
    mq_config.window_title = config.window_title.clone();

    miniquad::start(mq_config, move || {
        Box::new(Runtime::new(&config, game, SceneStack::new(scene)))
    });
    std::process::exit(0);
}

struct Runtime<T> {
    gui: GuiBackend,

    game: T,
    stack: SceneStack<T>,
}

impl<T> Runtime<T> {
    fn new(config: &Config, game: T, stack: SceneStack<T>) -> Self {
        let gui = GuiBackend::new(config);

        Runtime { gui, game, stack }
    }
}

impl<T> EventHandler for Runtime<T> {
    fn update(&mut self) {
        if self.stack.is_empty() {
            window::quit();
        }
    }

    fn draw(&mut self) {
        let now = date::now();
        let n = ((now - self.gui.last_update) / FRAME_DURATION.as_secs_f64())
            as u32;

        self.stack.update(
            &mut self.game,
            &mut self.gui,
            n.min(MAX_UPDATES_PER_FRAME),
        );

        self.gui.last_update += n as f64 * FRAME_DURATION.as_secs_f64();
        if n > 0 {
            self.gui.keypress.pop_front();
            self.gui.mouse_state.frame_update();
        }
    }

    fn key_down_event(
        &mut self,
        keycode: KeyCode,
        keymods: KeyMods,
        _repeat: bool,
    ) {
        if let Ok(typed) = KeyTyped::try_from((keycode, keymods)) {
            // Miniquad won't give char_event for non-printable keys, but we
            // want those too. Emit them here.
            if !matches!(typed.key(), Key::Char(_)) {
                self.gui.keypress.push_back(typed);
            }

            self.gui.key_down.insert(typed.key().char_to_lowercase());
        }
    }

    fn key_up_event(&mut self, keycode: KeyCode, keymods: KeyMods) {
        if let Ok(typed) = KeyTyped::try_from((keycode, keymods)) {
            self.gui.key_down.remove(&typed.key().char_to_lowercase());
        }
    }

    fn char_event(
        &mut self,
        mut character: char,
        keymods: KeyMods,
        _repeat: bool,
    ) {
        let c = character as u32;
        if c > 1 << 15 {
            // I think these are key release events or something?
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
        let typed = KeyTyped::new(Key::Char(character), mods);
        self.gui.keypress.push_back(typed);
    }

    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        let (x, y) = self.gui.transform_mouse_pos(x, y);
        // TODO: Transform coordinates based on whatever was rendered last
        // frame.
        *self.gui.mouse_state.cursor_pos_mut() = (x, y).into();
    }

    fn mouse_wheel_event(&mut self, _x: f32, y: f32) {
        let z = -y as i32;
        self.gui.mouse_state.scroll(if z < 0 { -1 } else { 1 });
    }

    fn mouse_button_down_event(
        &mut self,
        button: miniquad::MouseButton,
        _x: f32,
        _y: f32,
    ) {
        if let Ok(button) = button.try_into() {
            self.gui.mouse_state.button_down(button);
        }
    }

    fn mouse_button_up_event(
        &mut self,
        button: miniquad::MouseButton,
        _x: f32,
        _y: f32,
    ) {
        if let Ok(button) = button.try_into() {
            self.gui.mouse_state.button_up(button);
        }
    }
}

struct GuiBackend {
    gl: GlContext,

    pixels_pipeline: Pipeline,
    chars_pipeline: Pipeline,
    bindings: Bindings,

    // Lookup table from codepoints to font sheet.
    //
    // All undefined codepoints will point to 0xff.
    char_lookup: Vec<u8>,
    font_size: (u32, u32),
    system_colors: Option<[Rgba; 16]>,

    last_update: f64,

    key_down: HashSet<Key>,
    mouse_state: MouseState,
    keypress: VecDeque<KeyTyped>,

    mouse_offset: (i32, i32),
    mouse_scale: (i32, i32),
}

impl GuiBackend {
    fn new(config: &Config) -> Self {
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
            BufferUsage::Immutable,
            BufferSource::slice(&vertices),
        );

        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
        let index_buffer = gl.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&indices),
        );

        let font_size;

        let (font_image, font_chars) = match &config.font_sheet {
            Some(sheet) => {
                assert!(sheet.image.width() % 16 == 0 && sheet.image.height() % 16 == 0,
                    "Font sheet dimensions aren't a multiple of 16, sheet must be a 16x16 grid");
                font_size =
                    (sheet.image.width() / 16, sheet.image.height() / 16);
                (
                    create_texture(
                        &mut gl,
                        sheet.image.width(),
                        sheet.image.height(),
                        sheet.image.as_raw(),
                    ),
                    sheet.chars,
                )
            }
            None => {
                let sheet = FontSheet::default();
                assert!(sheet.image.width() % 16 == 0 && sheet.image.height() % 16 == 0,
                    "Font sheet dimensions aren't a multiple of 16, sheet must be a 16x16 grid");
                font_size =
                    (sheet.image.width() / 16, sheet.image.height() / 16);
                (
                    create_texture(
                        &mut gl,
                        sheet.image.width(),
                        sheet.image.height(),
                        sheet.image.as_raw(),
                    ),
                    sheet.chars,
                )
            }
        };

        let mut char_lookup = vec![0xff; 0x10000];
        for (i, &c) in font_chars.iter().enumerate() {
            char_lookup[c as u32 as usize] = i as u8;
        }

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
                ShaderSource {
                    glsl_vertex: Some(VERTEX_SHADER),
                    glsl_fragment: Some(PIXEL_FRAGMENT_SHADER),
                    metal_shader: None,
                },
                ShaderMeta {
                    images: vec!["pixels".to_string()],
                    uniforms: UniformBlockLayout {
                        uniforms: vec![
                            UniformDesc::new(
                                "terminal_size",
                                UniformType::Float2,
                            ),
                            UniformDesc::new(
                                "canvas_scale",
                                UniformType::Float2,
                            ),
                        ],
                    },
                },
            )
            .unwrap();

        let chars_shader = gl
            .new_shader(
                ShaderSource {
                    glsl_vertex: Some(VERTEX_SHADER),
                    glsl_fragment: Some(CHAR_FRAGMENT_SHADER),
                    metal_shader: None,
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
                        uniforms: vec![
                            UniformDesc::new(
                                "terminal_size",
                                UniformType::Float2,
                            ),
                            UniformDesc::new(
                                "canvas_scale",
                                UniformType::Float2,
                            ),
                        ],
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
        );

        let chars_pipeline = gl.new_pipeline(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float2),
                VertexAttribute::new("uv", VertexFormat::Float2),
            ],
            chars_shader,
        );

        let last_update = date::now();

        GuiBackend {
            gl,
            pixels_pipeline,
            chars_pipeline,
            bindings,
            char_lookup,
            font_size,
            system_colors: config.system_color_palette,
            last_update,
            key_down: Default::default(),
            mouse_state: Default::default(),
            keypress: Default::default(),
            mouse_offset: Default::default(),
            mouse_scale: (1, 1),
        }
    }

    fn pixel_canvas_scale(
        &mut self,
        buffer_w: u32,
        buffer_h: u32,
    ) -> (f32, f32) {
        let (w, h) = window::screen_size();
        // Snap window dimensions to even integers, otherwise there may be pixel
        // artifacts.
        let (w, h) =
            ((w as u32 & !1).max(2) as f32, (h as u32 & !1).max(2) as f32);

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

        (x / w, y / h)
    }

    fn char_canvas_scale(
        &mut self,
        buffer_w: u32,
        buffer_h: u32,
    ) -> (f32, f32) {
        let (buffer_w, buffer_h) =
            (buffer_w * self.font_size.0, buffer_h * self.font_size.1);

        let ret = self.pixel_canvas_scale(buffer_w, buffer_h);
        self.mouse_scale.0 *= self.font_size.0 as i32;
        self.mouse_scale.1 *= self.font_size.1 as i32;
        ret
    }

    fn transform_mouse_pos(&self, x: f32, y: f32) -> (i32, i32) {
        (
            (x as i32 - self.mouse_offset.0) / self.mouse_scale.0,
            (y as i32 - self.mouse_offset.1) / self.mouse_scale.1,
        )
    }

    fn convert_color(&self, c: X256Color) -> Rgba {
        if c.0 < 16 {
            if let Some(pal) = self.system_colors {
                return pal[c.0 as usize];
            }
        }

        c.into()
    }
}

impl Backend for GuiBackend {
    fn draw_pixels(&mut self, w: u32, h: u32, buffer: &[crate::Rgba]) {
        assert!(buffer.len() == (w * h) as usize);

        self.gl.texture_resize(
            self.bindings.images[0],
            w,
            h,
            Some(bytes(buffer)),
        );

        self.gl.begin_default_pass(Default::default());
        self.gl.apply_pipeline(&self.pixels_pipeline);
        self.gl.apply_bindings(&self.bindings);

        let canvas_scale = self.pixel_canvas_scale(w, h);

        self.gl.apply_uniforms(UniformsSource::table(&Uniforms {
            terminal_size: (0.0, 0.0),
            canvas_scale,
        }));

        self.gl.draw(0, 6, 1);
        self.gl.end_render_pass();
        self.gl.commit_frame();
    }

    fn draw_chars(&mut self, w: u32, h: u32, buffer: &[crate::CharCell]) {
        assert!(buffer.len() == (w * h) as usize);

        // TODO: Make the channel-buffers reused members of GuiBackend so I
        // don't need to heap-allocate new ones every frame.
        //
        // If I want to really optimize this, maybe the X256 palette could be
        // embedded in the shader and the shader could operate directly on
        // CharCell data...

        let chars: Vec<u32> = buffer
            .iter()
            .map(|a| self.char_lookup[a.c as usize] as u32)
            .collect();
        let fore: Vec<Rgba> = buffer
            .iter()
            .map(|a| self.convert_color(a.foreground))
            .collect();
        let back: Vec<Rgba> = buffer
            .iter()
            .map(|a| self.convert_color(a.background))
            .collect();

        self.gl.texture_resize(
            self.bindings.images[2],
            w,
            h,
            Some(bytes(&chars)),
        );
        self.gl.texture_resize(
            self.bindings.images[3],
            w,
            h,
            Some(bytes(&fore)),
        );
        self.gl.texture_resize(
            self.bindings.images[4],
            w,
            h,
            Some(bytes(&back)),
        );

        self.gl.begin_default_pass(Default::default());
        self.gl.apply_pipeline(&self.chars_pipeline);
        self.gl.apply_bindings(&self.bindings);

        let canvas_scale = self.char_canvas_scale(w, h);

        self.gl.apply_uniforms(UniformsSource::table(&Uniforms {
            terminal_size: (w as f32, h as f32),
            canvas_scale,
        }));

        self.gl.draw(0, 6, 1);
        self.gl.end_render_pass();
        self.gl.commit_frame();
    }

    fn pixel_resolution(&self) -> (u32, u32) {
        let (w, h) = window::screen_size();
        (w as u32, h as u32)
    }

    fn char_resolution(&self) -> (u32, u32) {
        let (w, h) = self.pixel_resolution();
        (w / self.font_size.0, h / self.font_size.1)
    }

    fn now(&self) -> f64 {
        self.last_update
    }

    fn is_down(&self, key: crate::Key) -> bool {
        self.key_down.contains(&key)
    }

    fn keypress(&self) -> crate::KeyTyped {
        self.keypress.front().copied().unwrap_or_default()
    }

    fn mouse_state(&self) -> crate::MouseState {
        self.mouse_state
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
            filter: FilterMode::Nearest,
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

uniform lowp vec2 canvas_scale;

varying lowp vec2 texcoord;
void main() {
    gl_Position = vec4(pos * canvas_scale, 0, 1);
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
    int chr = int(ch.x * 256.0);

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
    pub canvas_scale: (f32, f32),
}
