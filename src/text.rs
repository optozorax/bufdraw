use crate::image::Color;
use crate::image::Image;
use crate::vec::Vec2i;
use crate::image::set_pixel;
use crate::image::blend;
use crate::image::get_pixel;
use rusttype::{point, Font, Scale};

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
	            glyph.draw(|x, y, v| {
	                let x = (x as i32 + pos.x) + bounding_box.min.x;
	                let y = (y as i32 + pos.y) + bounding_box.min.y + y_offset as i32;
	                let a = (v * (color.a as f32)) as u8;
	                let c = Color::rgba(color.r, color.g, color.b, a);
	                let pos = Vec2i::new(x as i32, y);
	                set_pixel(image, &pos, &blend(&c, &get_pixel(image, &pos)));
	            });
	        }
	    }
	    y_offset += v_metrics.ascent;
	}
}