use core::ops::Range;
use crate::ImageTrait;
use crate::vec::*;
use crate::rangetools::*;

pub enum PixelPos {
	R,
	G,
	B,
	A,
}

pub struct Image {
	pub buffer: Vec<u8>,
	pub width: usize,
	pub height: usize,
}

impl ImageTrait for Image {
	fn get_rgba8_buffer(&self) -> &[u8] { &self.buffer[0..(self.height * self.width *4)] }
	fn get_width(&self) -> usize { self.width }
	fn get_height(&self) -> usize { self.height }
}

#[derive(Clone, Debug)]
pub struct Color {
	pub r: u8,
	pub g: u8,
	pub b: u8,
	pub a: u8,
}

impl Image {
	pub fn get_u32_buffer(&self) -> &[u32] {
		let len = self.height * self.width;
		let buffer = &self.buffer[0..(len * 4)];
		unsafe {
			let (prefix, shorts, suffix) = buffer.align_to();
			assert!(prefix.is_empty());
			assert!(suffix.is_empty());
			shorts
		}
	}

	pub fn get_u32_mut_buffer(&mut self) -> &mut [u32] {
		let len = self.height * self.width;
		let buffer = &mut self.buffer[0..(len * 4)];
		unsafe {
			let (prefix, shorts, suffix) = buffer.align_to_mut();
			assert!(prefix.is_empty());
			assert!(suffix.is_empty());
			shorts
		}
	}

	pub fn new(size: &Vec2i) -> Image {
		let width = size.x as usize;
		let height = size.y as usize;
		Image {
			buffer: vec![0; width * height * 4],
			width,
			height,
		}
	}

	pub fn resize_lazy(&mut self, size: &Vec2i) {
		let width = size.x as usize;
		let height = size.y as usize;
		let needed_size = width * height * 4 * 12 / 10; // With capacity
		if self.buffer.len() < needed_size {
			self.buffer.resize(needed_size, 0);
		}
		self.width = width;
		self.height = height;
	}

	#[inline]
	pub fn clear(&mut self, color: &Color) {
		let color = color.to_u32();
		for pix in self.get_u32_mut_buffer() {
			*pix = color;
		}
	}

	#[inline]
	pub fn get_rect(&self) -> Rect2i {
		Rect2i {
			min: Vec2i::default(),
			max: Vec2i::new(self.width as i32, self.height as i32),
		}
	}

	#[inline]
	pub fn range_x(&self) -> Range<i32> {
		0..(self.width as i32)
	}

	#[inline]
	pub fn range_y(&self) -> Range<i32> {
		0..(self.height as i32)
	}
}

impl Color {
	#[inline]
	pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
		Color { r, g, b, a }
	}

	#[inline]
	pub fn rgba_f64(r: f64, g: f64, b: f64, a: f64) -> Color {
		Color { 
			r: (r * 255.0) as u8, 
			g: (g * 255.0) as u8, 
			b: (b * 255.0) as u8, 
			a: (a * 255.0) as u8,
		}
	}

	#[inline]
	pub fn to_rgba_f64(&self) -> (f64, f64, f64, f64) {
		(
			self.r as f64 / 255.0,
			self.g as f64 / 255.0,
			self.b as f64 / 255.0,
			self.a as f64 / 255.0,
		)
	}

	#[inline]
	pub fn rgb(r: u8, g: u8, b: u8) -> Color {
		Color::rgba(r, g, b, 255)
	}

	#[inline]
	pub fn gray(rgb: u8) -> Color {
		Color::rgb(rgb, rgb, rgb)
	}

	#[inline]
	pub fn from_u32(v: u32) -> Self {
		let res = u32::to_le_bytes(v);
		Color::rgba(res[0], res[1], res[2], res[3])
	} 	

	#[inline]
	pub fn to_u32(&self) -> u32 {
		u32::from_le_bytes([self.r, self.g, self.b, self.a])
	}
}

#[inline]
pub fn get_pixel(image: &Image, pos: &Vec2i) -> Color {
	Color::from_u32(image.get_u32_buffer()[pos.x as usize + pos.y as usize * image.width])
}

#[inline]
pub fn set_pixel(image: &mut Image, pos: &Vec2i, color: &Color) {
	let width = image.width;
	image.get_u32_mut_buffer()[pos.x as usize + pos.y as usize * width] = color.to_u32();
}

#[inline]
pub fn draw_pixel(image: &mut Image, pos: &Vec2i, color: &Color) {
    set_pixel(image, &pos, &blend(&color, &get_pixel(image, &pos)));
}

fn for_two_images<F: Fn(&mut u32, &u32)>(dst: &mut Image, src: &Image, pos: &Vec2i, f: F) {
	let dst_y_range = intersect_range(
		&dst.range_y(), 
		&offset_range(&src.range_y(), pos.y)
	);

	let dst_x_range = intersect_range(
		&dst.range_x(), 
		&offset_range(&src.range_x(), pos.x)
	);

	if dst_x_range.end == dst_x_range.start {
		return;
	}

	let src_y_range = offset_range(&dst_y_range, -pos.y);
	let src_x_range = offset_range(&dst_x_range, -pos.x);

	let dst_width = dst.width as i32;
	let src_width = src.width as i32;

	let mut dst_x_range = offset_range(&dst_x_range, dst_y_range.start * dst_width);
	let mut src_x_range = offset_range(&src_x_range, src_y_range.start * src_width);

	let dst_buf = dst.get_u32_mut_buffer();
	let src_buf = src.get_u32_buffer();

	for _ in dst_y_range {
		let dst_slice = &mut dst_buf[range_to_usize(&dst_x_range)];
		let src_slice = &src_buf[range_to_usize(&src_x_range)];

		for (pix_dst, pix_src) in dst_slice.iter_mut().zip(src_slice.iter()) {
			f(pix_dst, pix_src);
		}

		dst_x_range = offset_range(&dst_x_range, dst_width);
		src_x_range = offset_range(&src_x_range, src_width);
	}
}

pub fn place_image(dst: &mut Image, src: &Image, pos: &Vec2i) {
	for_two_images(dst, src, pos, |pix_dst, pix_src| *pix_dst = *pix_src);
}

pub fn place_image_scaled(dst: &mut Image, src: &Image, pos: &Vec2i, scale: i32) {
	let dst_y_range = intersect_range(
		&dst.range_y(), 
		&offset_range(&(0..(src.height as i32 * scale)), pos.y)
	);

	let dst_x_range = intersect_range(
		&dst.range_x(), 
		&offset_range(&(0..(src.width as i32 * scale)), pos.x)
	);

	if dst_x_range.end == dst_x_range.start {
		return;
	}

	let src_y_range = offset_range(&dst_y_range, -pos.y);
	let src_x_range = offset_range(&dst_x_range, -pos.x);
	
	let src_y_range_slice = div_range(&src_y_range, scale);
	let src_x_range_slice = div_range(&src_x_range, scale);

	let dst_width = dst.width as i32;
	let src_width = src.width as i32;

	let mut dst_pos_range = offset_range(&dst_x_range, dst_y_range.start * dst_width);
	let mut src_pos_range = offset_range(&src_x_range_slice, src_y_range_slice.start * src_width);

	let dst_buf = dst.get_u32_mut_buffer();
	let src_buf = src.get_u32_buffer();

	let mut current_y = src_y_range.start / scale;

	for src_y in src_y_range {
		let dst_slice = &mut dst_buf[range_to_usize(&dst_pos_range)];
		let src_slice = &src_buf[range_to_usize(&src_pos_range)];
		
		let mut current_x = src_x_range_slice.start;

		let mut src_iter = src_slice.iter();
		let mut pix_src = src_iter.next().unwrap();
		for (pix_dst, src_x) in dst_slice.iter_mut().zip(src_x_range.clone()) {
			if src_x / scale != current_x {
				pix_src = src_iter.next().unwrap();
				current_x = src_x / scale;
			}
			*pix_dst = *pix_src;
		}

		dst_pos_range = offset_range(&dst_pos_range, dst_width);

		if src_y / scale != current_y {
			src_pos_range = offset_range(&src_pos_range, src_width);
			current_y = src_y / scale;
		}
		
	}
}

pub fn draw_image(dst: &mut Image, src: &Image, pos: &Vec2i) {
	for_two_images(dst, src, pos, |pix_dst, pix_src| {
		*pix_dst = blend(&Color::from_u32(*pix_src), &Color::from_u32(*pix_dst)).to_u32();
	});
}

#[inline]
pub fn function_for_all_pixels<F: FnMut(usize, usize) -> Color>(image: &mut Image, mut f: F) {
	let height = image.height;
	let width = image.width;
	let mut iter = image.get_u32_mut_buffer().iter_mut();
	for y in 0..height {
		for x in 0..width {
			let color = f(x, y);
			if let Some(c) = iter.next() {
				*c = color.to_u32();
			}
		}
	}
}

fn for_image_and_rect<F: Fn(&mut u32)>(dst: &mut Image, rect_size: &Vec2i, pos: &Vec2i, f: F) {
	let dst_y_range = intersect_range(
		&dst.range_y(), 
		&offset_range(&(0..rect_size.y), pos.y)
	);

	let dst_x_range = intersect_range(
		&dst.range_x(), 
		&offset_range(&(0..rect_size.x), pos.x)
	);

	if dst_x_range.end == dst_x_range.start {
		return;
	}

	let dst_width = dst.width as i32;

	let mut dst_x_range = offset_range(&dst_x_range, dst_y_range.start * dst_width);

	let dst_buf = dst.get_u32_mut_buffer();

	for _ in dst_y_range {
		let dst_slice = &mut dst_buf[range_to_usize(&dst_x_range)];

		for pix_dst in dst_slice.iter_mut() {
			f(pix_dst);
		}

		dst_x_range = offset_range(&dst_x_range, dst_width);
	}
}

#[inline]
pub fn draw_rect(image: &mut Image, pos: &Vec2i, size: &Vec2i, color: &Color) {
	for_image_and_rect(image, size, pos, |pix| {
		*pix = blend(&color, &Color::from_u32(*pix)).to_u32();
	});
}

#[inline]
pub fn rect(image: &mut Image, pos: &Vec2i, size: &Vec2i, color: &Color) {
	let color = color.to_u32();
	for_image_and_rect(image, size, pos, |pix| *pix = color);
}

#[inline]
/// Fast blend on integer numbers without gamma correction and premultiplied alpha. Source: https://en.wikipedia.org/wiki/Alpha_compositing#Alpha_blending
pub fn blend(src: &Color, dst: &Color) -> Color {
	let srca = src.a as i32;
	let dsta = dst.a as i32;

	let outa = (srca + dsta) * 255 - srca * dsta; 

	macro_rules! blend {
		($src:expr, $dst:expr) => {
			((255 * ($src as i32) * srca + ($dst as i32) * dsta * (255 - srca)) / outa) as u8
		};
	}

	if outa == 0 {
		Color::rgba(0, 0, 0, 0)
	} else {
		Color::rgba(
			blend!(src.r, dst.r),
			blend!(src.g, dst.g),
			blend!(src.b, dst.b),
			(outa / 255) as u8
		)
	}
}

#[inline]
/// Works on f32 with gamma correction of 2.2 power. Source: https://en.wikipedia.org/wiki/Alpha_compositing#Alpha_blending + https://en.wikipedia.org/wiki/Alpha_compositing#Composing_alpha_blending_with_gamma_correction
pub fn ideal_blend(src: &Color, dst: &Color) -> Color {
	let srca = src.a as f32 / 255.0;
	let dsta = dst.a as f32 / 255.0;

	let outa = 1. - (1. - srca) * (1. - dsta);

	macro_rules! blend {
		($src:expr, $dst:expr) => {
			(((($src as f32 / 255.0).powf(2.2) * srca + ($dst as f32 / 255.0).powf(2.2) * dsta * (1.0 - srca)) / outa).powf(1. / 2.2) * 255.0) as u8
		};
	}

	if outa == 0.0 {
		Color::rgba(0, 0, 0, 0)
	} else {
		Color::rgba(
			blend!(src.r, dst.r),
			blend!(src.g, dst.g),
			blend!(src.b, dst.b),
			(outa * 255.0) as u8
		)
	}
}

pub fn place_repeated_scaled_image(image: &mut Image, repeated_image: &Image, pos: &Vec2i, scale: i32,  repeat_x: bool, repeat_y: bool) {
	let size = Vec2i::new(repeated_image.get_width() as i32, repeated_image.get_height() as i32) * scale;
	let range_x = calc_range_for_repeated_line(repeat_x, pos.x, size.x, image.get_width() as i32);
	let range_y = calc_range_for_repeated_line(repeat_y, pos.y, size.y, image.get_height() as i32);

	for y in range_y {
		for x in range_x.clone() {
			place_image_scaled(image, repeated_image, &(Vec2i::new(
				x * size.x,
				y * size.y
			) + pos), scale);
		}
	}

	fn calc_range_for_repeated_line(repeat: bool, pos: i32, len: i32, size: i32) -> std::ops::Range<i32> {
		if repeat {
			let minus = {
				let mut pos_offset = 0;
				while pos + pos_offset * len >= -len {
					pos_offset -= 1 ;
				}
				pos_offset
			};

			let plus = {
				let mut pos_offset = 0;
				while pos + pos_offset * len < size {
					pos_offset += 1 ;
				}
				pos_offset
			};

			minus..plus
		} else {
			0i32..1i32
		}
	}
}
