use std::collections::HashMap;
use crate::image::*;
use crate::vec::*;
use crate::minmax::*;
use rusttype::{point, Font, Scale, GlyphId, PositionedGlyph, Rect};

struct AlphaImage {
	width: i32,
	height: i32,
	image: Vec<u8>,
}

pub struct TextCache {
	font: Font<'static>,
	size: f32,
	cache: HashMap<GlyphId, AlphaImage>,
}

impl TextCache {
	pub fn new(font: Font<'static>) -> Self {
		TextCache {
			font,
			size: 0.0,
			cache: HashMap::new(),
		}
	}

	fn clear(&mut self, size: f32) {
		self.size = size;
		self.cache.clear();
	}

	fn cache_glyph<'a>(&mut self, glyph: &PositionedGlyph<'a>, bounding_box: Rect<i32>) {
		let width = bounding_box.width();
		let height = bounding_box.height();
		let mut glyph_image = AlphaImage { 
			width, 
			height, 
			image: vec![0; (width * height) as usize],
		};
		glyph.draw(|x, y, v| {
			glyph_image.image[(x + y * width as u32) as usize] = (v * 255.0) as u8;
		});
		self.cache.insert(glyph.id(), glyph_image);
	}

	fn draw_and_cache<'a>(&mut self, image: &mut Image, glyph: PositionedGlyph<'a>, pos: &Vec2i, color: &Color) {
		if let Some(bounding_box) = glyph.pixel_bounding_box() {
			let min = Vec2i::new(bounding_box.min.x, bounding_box.min.y);
			let max = Vec2i::new(bounding_box.max.x, bounding_box.max.y);
			if image.get_rect().is_intersect(&Rect2i { min: min + pos, max: max + pos }) {
				if !self.cache.contains_key(&glyph.id()) {
					self.cache_glyph(&glyph, bounding_box);
				}
				let glyph_image = self.cache.get(&glyph.id()).unwrap();

				let pos = Vec2i::new(pos.x + bounding_box.min.x, pos.y + bounding_box.min.y);

				let start_y = 0.max((image.height as i32).min(pos.y));
				let end_y = 0.max((image.height as i32).min(pos.y + glyph_image.height as i32));
				let start_x = 0.max((image.width as i32).min(pos.x));
				let end_x = 0.max((image.width as i32).min(pos.x + glyph_image.width as i32));
				for y in start_y..end_y {
					for x in start_x..end_x {
						let current = Vec2i::new(x, y);
						let glyph_pos = ((current.x - pos.x) + (current.y - pos.y) * glyph_image.width) as usize;
						draw_pixel(image, &current, &Color::rgba(
							color.r, 
							color.g, 
							color.b, 
							(color.a as f32 * glyph_image.image[glyph_pos] as f32 / 255.0) as u8
						));
					}
				}
			}
		}
	}
}

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
	cache: &mut TextCache,
	text: &str, 
	size: f32, 
	pos: &Vec2i, 
	color: &Color
) {
	if size != cache.size {
		cache.clear(size);
	}

	let mut y_offset = 0.0;
	for line in text.split('\n') {
		let scale = Scale::uniform(size);
		let v_metrics = cache.font.v_metrics(scale);
		let glyphs: Vec<_> = cache.font
			.layout(line, scale, point(0.0, 0.0 + v_metrics.ascent))
			.collect();

		for glyph in glyphs {
			cache.draw_and_cache(image, glyph, &(pos.clone() + &Vec2i::new(0, y_offset as i32)), &color);
		}
		y_offset += v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;
	}
}

pub fn text_size(
	cache: &TextCache, 
	text: &str, 
	size: f32, 
) -> Vec2i {
	let font = &cache.font;
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