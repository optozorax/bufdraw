use crate::Vec2i;

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
}

impl Color {
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }
}

pub fn set_pixel(image: &mut Image, pos: &Vec2i, color: &Color) {
    let mut offset = (pos.x + pos.y * image.width as i32) as usize;
    offset *= 4;
    image.buffer[offset + 0] = color.r;
    image.buffer[offset + 1] = color.g;
    image.buffer[offset + 2] = color.b;
    image.buffer[offset + 3] = color.a;
}

pub fn function_for_all_pixels<F: Fn(usize, usize) -> i32>(image: &mut Image, f: F) {
    use PixelPos::*;
    let width = image.width;
    let height = image.height;
    let image_iter = image.buffer
        .iter_mut()
        .enumerate()
        .filter(|(index, _)| index < &(width * height * 4))
        .map(|(index, object)| 
            (
                index as usize / 4 % width, 
                index as usize / 4 / width, 
                match index % 4 {
                    0 => R, 
                    1 => G, 
                    2 => B, 
                    3 => A, 
                    _ => unreachable!()
                },
                object
            )
        );

    for (x, y, pos, pix) in image_iter {
        *pix = (match pos {
            R | G | B => f(x, y),
            A => 255,
        } % 256) as u8;
    }
}

pub fn rect(mut image: &mut Image, pos: &Vec2i, size: &Vec2i, color: &Color) {
    for y in 0..size.y {
        for x in 0..size.x {
            set_pixel(&mut image, &Vec2i::new(pos.x + x, pos.y + y), color);   
        }
    }
}