use miniquad::*;
use crate::vec::*;

pub mod image;
pub mod vec;
pub mod text;
pub mod measure;
pub mod interpolate;
pub mod minmax;
pub mod rangetools;

pub use miniquad::MouseButton;
pub use miniquad::TouchPhase;
pub use miniquad::KeyCode;
pub use miniquad::KeyMods;
pub use miniquad::date::now;
pub use rusttype::Font;

pub use miniquad_parameters::PROGRAM_PARAMETERS;

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

	fn touch_event(&mut self, _phase: TouchPhase, _id: u64, _pos: &Vec2i) {}
}

pub trait ImageTrait {
	fn get_rgba8_buffer(&self) -> &[u8];
	fn get_width(&self) -> usize;
	fn get_height(&self) -> usize;
}

struct MyWindow<T: MyEvents + ImageTrait> {
	external: T,

	vertex_buffer: Buffer,
	index_buffer: Buffer,
	pipeline: Pipeline,
	texture: Option<Texture>,
	bindings: Option<Bindings>,

	last_mouse_pos: Vec2i,
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
			texture: None,
			bindings: None,
			last_mouse_pos: Vec2i::default(),
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

impl<T: MyEvents + ImageTrait> EventHandler for MyWindow<T> {
	fn update(&mut self, _ctx: &mut Context) {
		self.external.update();
	}

	fn draw(&mut self, ctx: &mut Context) {
		ctx.begin_default_pass(Default::default());
		ctx.apply_pipeline(&self.pipeline);

		self.external.draw();

		if let Some(texture) = self.texture {
			texture.update(ctx, self.external.get_rgba8_buffer());
		} else {
			self.texture = Some(Texture::from_rgba8(
				ctx,
				self.external.get_width() as u16, 
				self.external.get_height() as u16, 
				&self.external.get_rgba8_buffer()
			));
			self.bindings = Some(Bindings {
				vertex_buffers: vec![self.vertex_buffer],
				index_buffer: self.index_buffer,
				images: vec![self.texture.unwrap()],
			});
		}

		ctx.apply_bindings(self.bindings.as_ref().unwrap());

		ctx.draw(0, 6, 1);
		ctx.end_render_pass();

		ctx.commit_frame();
	}

	fn resize_event(&mut self, _ctx: &mut Context, width: f32, height: f32) {
		self.external.resize_event((width, height).into());
		if let Some(bindings) = &self.bindings {
			bindings.images[0].delete();
		}
		self.texture = None;
	}

	fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32) {
		let current: Vec2i = (x, y).into();
		self.last_mouse_pos = current.clone();
		self.external.mouse_motion_event(current.clone(), current - &self.last_mouse_pos);
	}

	fn mouse_wheel_event(&mut self, _ctx: &mut Context, x: f32, y: f32) {
		use std::cmp::Ordering::*;
		let mouse_horizontal = match x.partial_cmp(&0.0).unwrap_or(Equal) {
			Less => MouseWheelHorizontal::RotateRight,
			Equal => MouseWheelHorizontal::Nothing,
			Greater => MouseWheelHorizontal::RotateLeft,
		};
		let mouse_vertical = match y.partial_cmp(&0.0).unwrap_or(Equal) {
			Less => MouseWheelVertical::RotateDown,
			Equal => MouseWheelVertical::Nothing,
			Greater => MouseWheelVertical::RotateUp,
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

	fn touch_event(&mut self, _ctx: &mut Context, phase: TouchPhase, id: u64, x: f32, y: f32) {
		let pos: Vec2i = (x, y).into();
		self.external.touch_event(phase, id, &pos);
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

	miniquad::start(conf::Conf { high_dpi: true, ..Default::default() }, |mut ctx| {
		let mut result = MyWindow::new(&mut ctx, t);
		result.external.init();
		let current_size = ctx.screen_size();
		result.resize_event(&mut ctx, current_size.0, current_size.1);
		miniquad::UserData::Owning((Box::new(result), ctx))
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
