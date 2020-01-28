use miniquad::*;
use crate::vec::*;

pub mod image;
pub mod vec;
pub mod text;
pub mod measure;

pub use miniquad::MouseButton;
pub use miniquad::Touch;
pub use miniquad::KeyCode;
pub use miniquad::KeyMods;
pub use miniquad::date::now;
pub use rusttype::Font;

pub enum ButtonState {
    Down,
    DownRepeat,
    Up,
}

#[derive(Debug)]
pub enum MouseWheelVertical {
    Nothing,
    RotateUp,
    RotateDown,
}

#[derive(Debug)]
pub enum MouseWheelHorizontal {
    Nothing,
    RotateLeft,
    RotateRight,
}

pub trait MyEvents {
    fn init(&mut self) {}
    fn update(&mut self) {}
    fn draw(&mut self) {}

    fn resize_event(&mut self, _new_size: Vec2i) {}
    fn mouse_wheel_event(&mut self, _pos: Vec2i, _dir_vertical: MouseWheelVertical, _dir_horizontal: MouseWheelHorizontal) {}
    fn mouse_motion_event(&mut self, _pos: Vec2i, _offset: Vec2i) {}
    fn mouse_button_event(&mut self, _button: MouseButton, _state: ButtonState, _pos: Vec2i) {}
    fn char_event(&mut self, _character: char, _keymods: KeyMods, _repeat: bool) {}
    fn key_event(&mut self, _keycode: KeyCode, _keymods: KeyMods, _state: ButtonState) {}


    fn touch_one_start(&mut self, _pos: &Vec2i) {}
    fn touch_one_move(&mut self, _pos: &Vec2i, _offset: &Vec2i) {}
    fn touch_one_end(&mut self) {}

    fn touch_scale_start(&mut self, _pos: &Vec2i) {}
    fn touch_scale_change(&mut self, scale: f32, _pos: &Vec2i, _offset: &Vec2i) {}
    fn touch_scale_end(&mut self) {}

    fn touch_three_start(&mut self, _pos: &Vec2i) {}
    fn touch_three_move(&mut self, _pos: &Vec2i, _offset: &Vec2i) {}
    fn touch_three_end(&mut self) {}

    fn touch_start_event(&mut self, touches: &Vec<Touch>) {}
    fn touch_end_event(&mut self, touches: &Vec<Touch>) {}
    fn touch_cancel_event(&mut self, touches: &Vec<Touch>) {}
    fn touch_move_event(&mut self, touches: &Vec<Touch>) {}
}

pub trait ImageTrait {
    fn get_rgba8_buffer(&self) -> &[u8];
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;
}

use std::collections::HashMap;

struct MyWindow<T: MyEvents + ImageTrait> {
    external: T,

    vertex_buffer: Buffer,
    index_buffer: Buffer,
    pipeline: Pipeline,

    last_mouse_pos: Vec2i,

    current_touches: HashMap<u32, Vec2i>,
        one_touch_regime: bool,
        one_touch_pos: Vec2i,

        two_touch_regime: bool,
        two_touch_pos: Vec2i,
        scale_start: f32,

        three_touch_regime: bool,
        three_touch_pos: Vec2i,
}

fn make_bindings<T: MyEvents + ImageTrait>(ctx: &mut Context, my_window: &mut MyWindow<T>) -> Bindings {
    let texture = Texture::from_rgba8(
        ctx,
        my_window.external.get_width() as u16, 
        my_window.external.get_height() as u16, 
        &my_window.external.get_rgba8_buffer()
    );

    let bindings = Bindings {
        vertex_buffers: vec![my_window.vertex_buffer],
        index_buffer: my_window.index_buffer,
        images: vec![texture],
    };

    bindings
}

impl<T: MyEvents + ImageTrait> MyWindow<T> {
    pub fn new(ctx: &mut Context, external: T) -> MyWindow<T> {
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

        #[rustfmt::skip]
        let vertices: [Vertex; 4] = [
            Vertex { pos : Vec2 { x: -1.0, y:  1.0 }, uv: Vec2 { x: 0., y: 0. } },
            Vertex { pos : Vec2 { x:  1.0, y:  1.0 }, uv: Vec2 { x: 1., y: 0. } },
            Vertex { pos : Vec2 { x:  1.0, y: -1.0 }, uv: Vec2 { x: 1., y: 1. } },
            Vertex { pos : Vec2 { x: -1.0, y: -1.0 }, uv: Vec2 { x: 0., y: 1. } },
        ];
        let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);

        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
        let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);

        MyWindow { 
            external,
            vertex_buffer,
            index_buffer,
            last_mouse_pos: Vec2i::default(),
            current_touches: HashMap::new(),
            one_touch_regime: false,
            two_touch_regime: false,
            three_touch_regime: false,
            one_touch_pos: Vec2i::default(),
            two_touch_pos: Vec2i::default(),
            three_touch_pos: Vec2i::default(),
            scale_start: 0.0,
            pipeline: {
                let shader = Shader::new(ctx, shader::VERTEX, shader::FRAGMENT, shader::META);

                Pipeline::new(
                    ctx,
                    &[BufferLayout::default()],
                    &[
                        VertexAttribute::new("pos", VertexFormat::Float2),
                        VertexAttribute::new("uv", VertexFormat::Float2),
                    ],
                    shader,
                )
            }
        }
    }
}

impl<T: MyEvents + ImageTrait> MyWindow<T> {
    fn insert_touches(&mut self, touches: &Vec<Touch>) {
        for touch in touches {
            self.current_touches.insert(touch.id, (touch.x, touch.y).into());
        }
    }

    fn remove_touches(&mut self, touches: &Vec<Touch>) {
        for touch in touches {
            self.current_touches.remove(&touch.id);
        }
    }

    fn get_first_touch(&self) -> Option<&Vec2i> {
        if let Some((_, pos)) = self.current_touches.iter().next() {
            Some(pos)
        } else {
            None
        }
    }

    fn get_first_two_touches(&self) -> Option<(&Vec2i, &Vec2i)> {
        let mut iter = self.current_touches.iter();
        if let Some((_, pos1)) = iter.next() {
            if let Some((_, pos2)) = iter.next() {
                Some((pos1, pos2))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_first_three_touches(&self) -> Option<(&Vec2i, &Vec2i, &Vec2i)> {
        let mut iter = self.current_touches.iter();
        if let Some((_, pos1)) = iter.next() {
            if let Some((_, pos2)) = iter.next() {
                if let Some((_, pos3)) = iter.next() {
                    Some((pos1, pos2, pos3))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    fn process_one_touch(&mut self) {
        if self.current_touches.len() == 1 {
            let new_pos = self.get_first_touch().unwrap().clone();
            if self.one_touch_regime {
                self.external.touch_one_move(&self.one_touch_pos, &(new_pos.clone() - &self.one_touch_pos));
                self.one_touch_pos = new_pos;
            } else {
                self.one_touch_pos = new_pos;
                self.one_touch_regime = true;
                self.external.touch_one_start(&self.one_touch_pos);
            }
        } else {
            if self.one_touch_regime {
                self.one_touch_regime = false;
                self.external.touch_one_end();
            }
        }
    }

    fn process_two_touches(&mut self) {
        if self.current_touches.len() == 2 {
            let (pos1, pos2) = self.get_first_two_touches().unwrap();
            let center = (pos1.clone() + pos2) / 2;
            let current_scale = (pos1.clone() - pos2).len();
            if self.two_touch_regime {
                self.external.touch_scale_change(current_scale / self.scale_start, &center, &(center.clone() - &self.two_touch_pos));
                self.two_touch_pos = center;
            } else {
                self.two_touch_regime = true;
                self.scale_start = current_scale;
                self.two_touch_pos = center;
                self.external.touch_scale_start(&self.two_touch_pos);
            }
        } else {
            if self.two_touch_regime {
                self.two_touch_regime = false;
                self.external.touch_scale_end();
            }
        }
    }

    fn process_three_touches(&mut self) {
        if self.current_touches.len() == 3 {
            let (pos1, pos2, pos3) = self.get_first_three_touches().unwrap();
            let center = (pos1.clone() + pos2 + pos3) / 3;
            if self.three_touch_regime {
                self.external.touch_three_move(&center, &(center.clone() - &self.three_touch_pos));
                self.three_touch_pos = center;
            } else {
                self.three_touch_regime = true;
                self.three_touch_pos = center;
                self.external.touch_three_start(&self.three_touch_pos);
            }
        } else {
            if self.three_touch_regime {
                self.three_touch_regime = false;
                self.external.touch_three_end();
            }
        }
    }
}

impl<T: MyEvents + ImageTrait> EventHandler for MyWindow<T> {
    fn update(&mut self, _ctx: &mut Context) {
        self.external.update();
    }

    fn draw(&mut self, ctx: &mut Context) {
        ctx.begin_default_pass(Default::default());
        ctx.apply_pipeline(&self.pipeline);

        self.external.draw();

        let bindings = make_bindings(ctx, self);
        ctx.apply_bindings(&bindings);

        ctx.draw(0, 6, 1);
        ctx.end_render_pass();

        ctx.commit_frame();

        bindings.images[0].delete();
    }

    fn resize_event(&mut self, _ctx: &mut Context, width: f32, height: f32) {
        self.external.resize_event((width, height).into());
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, dx: f32, dy: f32) {
        self.last_mouse_pos = (x, y).into();
        self.external.mouse_motion_event((x, y).into(), (dx, dy).into());
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, x: f32, y: f32) {
        let mouse_horizontal = if x < 0.0 {
            MouseWheelHorizontal::RotateRight
        } else if x > 0.0 {
            MouseWheelHorizontal::RotateLeft
        } else {
            MouseWheelHorizontal::Nothing
        };
        let mouse_vertical = if y < 0.0 {
            MouseWheelVertical::RotateDown
        } else if y > 0.0 {
            MouseWheelVertical::RotateUp
        } else {
            MouseWheelVertical::Nothing
        };
        self.external.mouse_wheel_event(self.last_mouse_pos.clone(), mouse_vertical, mouse_horizontal);
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.last_mouse_pos = (x, y).into();
        self.external.mouse_button_event(button, ButtonState::Down, (x, y).into());
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.last_mouse_pos = (x, y).into();
        self.external.mouse_button_event(button, ButtonState::Up, (x, y).into());
    }

    fn char_event(&mut self, _ctx: &mut Context, character: char, keymods: KeyMods, repeat: bool) {
        self.external.char_event(character, keymods, repeat);
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, keymods: KeyMods, repeat: bool) {
        if repeat {
            self.external.key_event(keycode, keymods, ButtonState::DownRepeat);
        } else {
            self.external.key_event(keycode, keymods, ButtonState::Down);
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, keymods: KeyMods) {
        self.external.key_event(keycode, keymods, ButtonState::Up);
    }

    fn touch_start_event(&mut self, ctx: &mut Context, touches: Vec<Touch>) {
        self.external.touch_start_event(&touches);
        self.insert_touches(&touches);
        self.process_one_touch();
        self.process_two_touches();
        self.process_three_touches();
    }

    fn touch_end_event(&mut self, ctx: &mut Context, touches: Vec<Touch>) {
        self.external.touch_end_event(&touches);
        self.remove_touches(&touches);
        self.process_one_touch();
        self.process_two_touches();
        self.process_three_touches();
    }

    fn touch_cancel_event(&mut self, ctx: &mut Context, touches: Vec<Touch>) {
        self.external.touch_cancel_event(&touches);
        self.remove_touches(&touches);
        self.process_one_touch();
        self.process_two_touches();
        self.process_three_touches();
    }

    fn touch_move_event(&mut self, ctx: &mut Context, touches: Vec<Touch>) {
        self.external.touch_move_event(&touches);
        self.insert_touches(&touches);
        self.process_one_touch();
        self.process_two_touches();
        self.process_three_touches();
    }
}

pub fn start<T: 'static +  MyEvents + ImageTrait>(t: T) {
    #[cfg(target_arch = "wasm32")]
    sapp_console_log::init().unwrap();

    #[cfg(not(target_arch = "wasm32"))]
    {
        std::env::set_var("RUST_LOG", "info");
        env_logger::init();
    }

    miniquad::start(conf::Conf::default(), |ctx| {
        let mut result = MyWindow::new(ctx, t);
        let current_size = ctx.screen_size();
        result.resize_event(ctx, current_size.0, current_size.1);
        Box::new(result)
    });
}

mod shader {
    use miniquad::*;

    pub const VERTEX: &str = r#"#version 100
    attribute vec2 pos;
    attribute vec2 uv;

    uniform vec2 offset;

    varying lowp vec2 texcoord;

    void main() {
        gl_Position = vec4(pos + offset, 0, 1);
        texcoord = uv;
    }"#;

    pub const FRAGMENT: &str = r#"#version 100
    varying lowp vec2 texcoord;

    uniform sampler2D tex;

    void main() {
        gl_FragColor = texture2D(tex, texcoord);
    }"#;

    pub const META: ShaderMeta = ShaderMeta {
        images: &["tex"],
        uniforms: UniformBlockLayout {
            uniforms: &[],
        },
    };
}