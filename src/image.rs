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

#[derive(Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Image {
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
        function_for_all_pixels(self, |_, _| color.clone());
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
    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color::rgba(r, g, b, 255)
    }

    #[inline]
    pub fn gray(rgb: u8) -> Color {
        Color::rgb(rgb, rgb, rgb)
    }
}

#[inline]
pub fn set_pixel(image: &mut Image, pos: &Vec2i, color: &Color) {
    let mut offset = (pos.x + pos.y * image.width as i32) as usize;
    offset *= 4;
    image.buffer[offset + 0] = color.r;
    image.buffer[offset + 1] = color.g;
    image.buffer[offset + 2] = color.b;
    image.buffer[offset + 3] = color.a;
}

#[inline]
pub fn function_for_all_pixels<F: FnMut(usize, usize) -> Color>(image: &mut Image, mut f: F) {
    let mut iter = image.buffer.iter_mut();
    for y in 0..image.height {
        for x in 0..image.width {
            let color = f(x, y);
            if let Some(r) = iter.next() { *r = color.r; }
            if let Some(g) = iter.next() { *g = color.g; }
            if let Some(b) = iter.next() { *b = color.b; }
            if let Some(a) = iter.next() { *a = color.a; }
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