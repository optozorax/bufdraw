use bufdraw::text::TextCache;
use bufdraw::text::{draw_text, text_size};
use bufdraw::*;
use bufdraw::vec::*;
use bufdraw::image::Image;

use raqote::*;

const MARGIN: i32 = 50;
const SIZE: i32 = 220;
const COUNT: i32 = 7;
const PADDING: i32 = 70;
const TEXT_PADDING: i32 = 100;
const MARGIN_DOWN: i32 = 20;
const COEFF: f64 = 0.9;

const WIDTH: i32 = MARGIN * 2 + SIZE * 3 + PADDING * 2;
const HEIGHT: i32 = MARGIN + MARGIN_DOWN + SIZE + TEXT_PADDING;

fn grid(a: f64, b: f64, count: i32, pos: i32, m: impl Fn(f64) -> f64) -> f64 {
	a + m(pos as f64 / count as f64) * (b-a)
}

fn non_linear_grid(t: f64) -> impl Fn(f64) -> f64 {
	move |x: f64| -> f64 {
		if t == 0.0 {
			linear_grid(x)
		} else {
			(1. - (1. - t.abs()).powf(x * t.signum())) / (1. - (1. - t.abs()).powf(t.signum()))	
		}
	}
}

fn linear_grid(x: f64) -> f64 {
	x
}

fn main() {
	let font_data = include_bytes!("WenQuanYiMicroHei.ttf");
	let mut cache = TextCache::new(Font::from_bytes(font_data as &[u8]).expect("Error constructing Font"));
	let mut dt = DrawTarget::new(WIDTH, HEIGHT);

	dt.clear(SolidSource::from_unpremultiplied_argb(0xff, 0xff, 0xff, 0xff));

	fn square(image: &mut DrawTarget, left_up: Vec2i, right_down: Vec2i) {
		let mut pb = PathBuilder::new();
		pb.move_to(left_up.x as f32, left_up.y as f32);
		pb.line_to(right_down.x as f32, left_up.y as f32);
		pb.line_to(right_down.x as f32, right_down.y as f32);
		pb.line_to(left_up.x as f32, right_down.y as f32);
		pb.line_to(left_up.x as f32, left_up.y as f32);
		let path = pb.finish();

		image.stroke(
			&path,
			&Source::Solid(SolidSource {
				r: 153,
				g: 0,
				b: 0,
				a: 255,
			}),
			&StrokeStyle {
				cap: LineCap::Round,
				join: LineJoin::Round,
				width: 1.0,
				miter_limit: 1.,
				dash_array: vec![],
				dash_offset: 0.,
			},
			&DrawOptions::new()
		);
	}

	let square_size = Vec2i::new(SIZE, SIZE);
	let texts = [
		"Неравномерная сетка,\nсгущающаяся к\nлевому краю", 
		"Равномерная сетка", 
		"Неравномерная сетка,\nсгущающаяся к\nправому краю"
	];

	for ((start, non_linearity), text) in (0..3)
		.map(|x| (
			Vec2i::new(
				MARGIN + x * (PADDING + SIZE), MARGIN), 
				(x as f64 - 1.0) * COEFF
			)
		)
		.zip(texts.iter()) 
	{
		square(&mut dt, start.clone(), start.clone() + &square_size);
		let pos = |i, j| Vec2i::new(
			grid(start.x as f64, (start.x + SIZE) as f64, COUNT, i, non_linear_grid(non_linearity)) as i32, 
			grid(start.y as f64, (start.y + SIZE) as f64, COUNT, j, linear_grid) as i32
		);
		for i in 0..COUNT {
			for j in 0..COUNT {
				let a = pos(i, j);
				let b = pos(i+1, j+1);
				square(&mut dt, a, b);
			}
		}

		let text_height = 24.0;
		let text_size = text_size(&cache, text, text_height);

		// TODO сделать что-то, чтобы не гонять их туда-сюда
		let mut image = Image::new(&Vec2i::new(dt.width(), dt.height()));
		image.buffer = dt.get_data_u8().to_vec();

		draw_text(
			&mut image,
			&mut cache,
			text,
			text_height, 
			&(start + &Vec2i::new(SIZE / 2, SIZE + TEXT_PADDING / 2) - &(text_size / 2)), 
			&bufdraw::image::Color::rgba(0, 0, 0, 255)
		);

		// TODO сделать что-то, чтобы не гонять их туда-сюда
		dt = DrawTarget::from_vec(image.width as i32, image.height as i32, image.get_u32_buffer().to_vec());
	}

	// TODO сделать что-то, чтобы не гонять их туда-сюда
	let mut image = Image::new(&Vec2i::new(dt.width(), dt.height()));
	image.buffer = dt.get_data_u8().to_vec();
	image.save_png(std::path::Path::new("grid.png")).unwrap();
}