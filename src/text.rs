use crate::image::*;
use crate::vec::*;
use crate::minmax::*;
use rusttype::{point, Font, Scale};

pub fn preprocess_text(
	text: &str,
	tab_size: usize,
	cut_width: Option<usize>,
	consider_tab_when_cut: bool,
) -> String {
	let mut text = String::from(text);
	text = text.replace("\t", &" ".repeat(tab_size));
	let mut result = String::new();
	for mut line in text.lines() {
		if let Some(mut cut_width) = cut_width {
			debug_assert!(cut_width != 0);
			cut_width = cut_width.max(1);
			if consider_tab_when_cut {
				let indent = {
					let mut result = 0;
					for i in line.chars() {
						if i == ' ' {
							result += 1;
						} else {
							break;
						}
					}
					result
				};

				if line.chars().count() > cut_width {
					let (a, b) = line.split_at(line.char_indices().nth(cut_width).unwrap().0);
					result += a;
					result += "\n";
					line = b;

					while line.chars().count() > (cut_width - indent) {
						let (a, b) = line.split_at(line.char_indices().nth(cut_width - indent).unwrap().0);
						result += &" ".repeat(indent);
						result += a;
						result += "\n";
						line = b;
					}
					result += &" ".repeat(indent);
				}
			} else {
				while line.chars().count() > cut_width {
					let (a, b) = line.split_at(line.char_indices().nth(cut_width).unwrap().0);
					result += a;
					result += "\n";
					line = b;
				}
			}
			result += line;
			result += "\n";
		}
	}
	result
}

pub fn draw_text(
	image: &mut Image, 
	font: &Font, 
	text: &str, 
	size: f32, 
	pos: &Vec2i, 
	color: &Color
) {
	let mut y_offset = 0.0;
	for line in text.split('\n') {
		let scale = Scale::uniform(size);
		let v_metrics = font.v_metrics(scale);
		let glyphs: Vec<_> = font
			.layout(line, scale, point(0.0, 0.0 + v_metrics.ascent))
			.collect();

		for glyph in glyphs {
			if let Some(bounding_box) = glyph.pixel_bounding_box() {
				let min = Vec2i::new(bounding_box.min.x, bounding_box.min.y + y_offset as i32);
				let max = Vec2i::new(bounding_box.max.x, bounding_box.max.y + y_offset as i32);
				if image.get_rect().is_intersect(&Rect2i { min: min + pos, max: max + pos }) {
					glyph.draw(|x, y, v| {
						let x = (x as i32 + pos.x) + bounding_box.min.x;
						let y = (y as i32 + pos.y) + bounding_box.min.y + y_offset as i32;
						let a = (v * (color.a as f32)) as u8;
						let c = Color::rgba(color.r, color.g, color.b, a);
						let pos = Vec2i::new(x as i32, y);
						if image.get_rect().is_inside(&pos) {
							draw_pixel(image, &pos, &c);
						}
					});
				}
			}
		}
		y_offset += v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;
	}
}

pub fn text_size(
	font: &Font, 
	text: &str, 
	size: f32, 
) -> Vec2i {
	let mut bounding_box_finder = BoundingBoxFinder::default();

	let mut y_offset = 0.0;
	for line in text.split('\n') {
		let scale = Scale::uniform(size);
		let v_metrics = font.v_metrics(scale);
		let glyphs: Vec<_> = font
			.layout(line, scale, point(0.0, 0.0 + v_metrics.ascent))
			.collect();

		for glyph in glyphs {
			if let Some(bounding_box) = glyph.pixel_bounding_box() {
				let min = Vec2i::new(bounding_box.min.x, bounding_box.min.y + y_offset as i32);
				let max = Vec2i::new(bounding_box.max.x, bounding_box.max.y + y_offset as i32);
				bounding_box_finder.process(&min);
				bounding_box_finder.process(&max);
			}
		}
		y_offset += v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;
	}

	return bounding_box_finder.size().unwrap_or(Vec2i::default());
}