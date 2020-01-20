use bufdraw::*;

enum PixelPos {
    R,
    G,
    B,
    A,
}

struct Image {
    buffer: Vec<u8>,
    width: usize,
    height: usize,
}

struct Window {
    image: Image,
    counter: u32,
    cursor: Vec2i,
    click: Vec2i,
    wheel: Vec2i,
}

struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl ImageTrait for Window {
    fn get_rgba8_buffer(&self) -> &[u8] {
        &self.image.buffer
    }

    fn get_width(&self) -> usize {
        self.image.width
    }

    fn get_height(&self) -> usize {
        self.image.height
    }
}

impl Window {
    fn new() -> Self {
        let width = 1920;
        let height = 1080;
        Window {
            image: Image {
                buffer: vec![0; width * height * 4],
                width,
                height,
            },
            counter: 0,
            cursor: Vec2i { x: 0, y: 0 },
            click: Vec2i { x: 0, y: 0 },
            wheel: Vec2i { x: 0, y: 0 },
        }
    }
}

impl MyEvents for Window {
    fn update(&mut self) {

    }
    fn draw(&mut self) {
        self.counter += 1;
        let counter = self.counter as usize;
        function_for_all_pixels(&mut self.image, |x, y| ((x & y) ^ counter) as i32);
        let size = Vec2i { x: 10, y: 10 };
        rect(&mut self.image, &self.cursor, &size, &Color::rgba(255, 0, 0, 255));
        rect(&mut self.image, &self.click, &size, &Color::rgba(0, 255, 0, 255));
        rect(&mut self.image, &self.wheel, &size, &Color::rgba(0, 0, 255, 255));
    }

    fn resize_event(&mut self, new_size: Vec2i) {
        self.image.width = new_size.x as usize;
        self.image.height = new_size.y as usize;
        let needed_size = self.image.width * self.image.height * 4;
        if self.image.buffer.len() < needed_size {
            self.image.buffer.resize((needed_size as f32 * 1.2) as usize, 0);
        }
    }

    fn mouse_motion_event(&mut self, pos: Vec2i, _offset: Vec2i) {
        self.cursor = pos;
    }

    fn mouse_button_event(&mut self, _button: MouseButton, _state: ButtonState, pos: Vec2i) {
        self.click = pos;
    }

    fn mouse_wheel_event(&mut self, pos: Vec2i, _dir: MouseWheel, _press: bool) {
        self.wheel = pos;
    }
}

fn rect(mut image: &mut Image, pos: &Vec2i, size: &Vec2i, color: &Color) {
    for y in 0..size.x {
        for x in 0..size.y {
            set_pixel(&mut image, &Vec2i { x: pos.x + x, y: pos.y + y }, color);   
        }
    }
}

impl Color {
    fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }
}

fn set_pixel(image: &mut Image, pos: &Vec2i, color: &Color) {
    let mut offset = (pos.x + pos.y * image.width as i32) as usize;
    offset *= 4;
    image.buffer[offset + 0] = color.r;
    image.buffer[offset + 1] = color.g;
    image.buffer[offset + 2] = color.b;
    image.buffer[offset + 3] = color.a;
}

fn function_for_all_pixels<F: Fn(usize, usize) -> i32>(image: &mut Image, f: F) {
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

fn main() {
    start(Window::new());
}