use crate::ImageTrait;
use crate::vec::*;

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
		unsafe { std::slice::from_raw_parts(buffer.as_ptr() as *const u32, len) }
	}

	pub fn get_u32_mut_buffer(&mut self) -> &mut [u32] {
		let len = self.height * self.width;
		let buffer = &mut self.buffer[0..(len * 4)];
		unsafe { std::slice::from_raw_parts_mut(buffer.as_mut_ptr() as *mut u32, len) }
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
	let mut offset = (pos.x + pos.y * image.width as i32) as usize;
	offset *= 4;
	assert!(offset + 3 < image.buffer.len());
	Color::rgba(
		image.buffer[offset + 0],
		image.buffer[offset + 1],
		image.buffer[offset + 2],
		image.buffer[offset + 3],
	)
}

#[inline]
pub fn set_pixel(image: &mut Image, pos: &Vec2i, color: &Color) {
	let mut offset = (pos.x + pos.y * image.width as i32) as usize;
	offset *= 4;
	assert!(offset + 3 < image.buffer.len());
	image.buffer[offset + 0] = color.r;
	image.buffer[offset + 1] = color.g;
	image.buffer[offset + 2] = color.b;
	image.buffer[offset + 3] = color.a;
}

#[inline]
pub fn draw_pixel(image: &mut Image, pos: &Vec2i, color: &Color) {
    set_pixel(image, &pos, &blend(&color, &get_pixel(image, &pos)));
}

#[inline]
pub fn pos_in_interval<'a, T>(min: T, pos: T, max: T) -> T where
	T: 'a +  PartialOrd + Copy + std::ops::Sub<T, Output = T> + std::cmp::Ord
{
	min.max(max.min(pos))
}

fn for_two_images<F: Fn(&mut u32, &u32)>(dst: &mut Image, src: &Image, pos: &Vec2i, f: F) {
	let start_y_dst = pos_in_interval(0, pos.y, dst.height as i32) as usize;
	let end_y_dst = pos_in_interval(0, pos.y + src.height as i32, dst.height as i32) as usize;
	let start_x_dst = pos_in_interval(0, pos.x, dst.width as i32) as usize;
	let end_x_dst = pos_in_interval(0, pos.x + src.width as i32, dst.width as i32) as usize;

	let start_y_src = (start_y_dst as i32 - pos.y) as usize;
	let end_y_src = (end_y_dst as i32 - pos.y) as usize;
	let start_x_src = (start_x_dst as i32 - pos.x) as usize;
	let end_x_src = (end_x_dst as i32 - pos.x) as usize;

	let dst_width = dst.width;
	let src_width = src.width;

	let dst_buf = dst.get_u32_mut_buffer();
	let src_buf = src.get_u32_buffer();

	for (y_dst, y_src) in (start_y_dst..end_y_dst).zip(start_y_src..end_y_src) {
		let dst_offset = y_dst * dst_width;
		let src_offset = y_src * src_width;

		let dst_slice = &mut dst_buf[(dst_offset + start_x_dst)..(dst_offset + end_x_dst)];
		let src_slice = &src_buf[(src_offset + start_x_src)..(src_offset + end_x_src)];

		for (pix_dst, pix_src) in dst_slice.iter_mut().zip(src_slice.iter()) {
			f(pix_dst, pix_src);
		}
	}
}

pub fn place_image(dst: &mut Image, src: &Image, pos: &Vec2i) {
	for_two_images(dst, src, pos, |pix_dst, pix_src| *pix_dst = *pix_src);
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

#[inline]
pub fn draw_rect(mut image: &mut Image, pos: &Vec2i, size: &Vec2i, color: &Color) {
	for y in pos.y.max(0)..(image.height as i32).min(size.y + pos.y) {
		for x in pos.x.max(0)..(image.width as i32).min(size.x + pos.x) {
			draw_pixel(&mut image, &Vec2i::new(x, y), color);   
		}
	}
}

#[inline]
pub fn rect(mut image: &mut Image, pos: &Vec2i, size: &Vec2i, color: &Color) {
	for y in pos.y.max(0)..(image.height as i32).min(size.y + pos.y) {
		for x in pos.x.max(0)..(image.width as i32).min(size.x + pos.x) {
			set_pixel(&mut image, &Vec2i::new(x, y), color);   
		}
	}
}

#[inline]
pub fn blend(up: &Color, low: &Color) -> Color {
	let upr:i32 = up.r as i32;
	let upg:i32 = up.g as i32;
	let upb:i32 = up.b as i32;
	let upa:i32 = up.a as i32;

	let lowr:i32 = low.r as i32;
	let lowg:i32 = low.g as i32;
	let lowb:i32 = low.b as i32;
	let lowa:i32 = low.a as i32;

	Color::rgba(
		(((upr - lowr) * upa + (lowr << 8)) >> 8) as u8,
		(((upg - lowg) * upa + (lowg << 8)) >> 8) as u8,
		(((upb - lowb) * upa + (lowb << 8)) >> 8) as u8,
		((upa + lowa) - ((lowa * upa + 255) >> 8)) as u8
	)
}

pub fn draw_repeated_rect(image: &mut Image, pos: &Vec2i, size: &Vec2i, color: &Color, repeat_x: Option<u32>, repeat_y: Option<u32>) {
	let range_x = calc_range_for_repeated_line(repeat_x, pos.x, size.x, image.get_width() as i32);
	let range_y = calc_range_for_repeated_line(repeat_y, pos.y, size.y, image.get_height() as i32);

	for y in range_y {
		for x in range_x.clone() {
			rect(image, &(Vec2i::new(
				x * size.x * repeat_x.unwrap_or(1) as i32, 
				y * size.y * repeat_y.unwrap_or(1) as i32
			) + pos), size, color);
		}
	}

	fn calc_range_for_repeated_line(repeat: Option<u32>, pos: i32, len: i32, size: i32) -> std::ops::Range<i32> {
		if let Some(repeat) = repeat {
			let minus = {
				let mut pos_offset = 0;
				while pos + pos_offset * len * (repeat as i32) >= -len {
					pos_offset -= 1 ;
				}
				pos_offset
			};

			let plus = {
				let mut pos_offset = 0;
				while pos + pos_offset * len * (repeat as i32) < size {
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
