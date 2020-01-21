use bufdraw::*;
use bufdraw::vec::*;
use bufdraw::image::*;

use log::info;

struct Window {
    image: Image,
    counter: u32,
    cursor: Vec2i,
    click: Vec2i,
    wheel: Vec2i,
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
        Window {
            image: Image::new(&Vec2i::new(1920, 1080)),
            counter: 0,
            cursor: Vec2i::default(),
            click: Vec2i::default(),
            wheel: Vec2i::default(),
        }
    }
}

impl MyEvents for Window {
    fn update(&mut self) {

    }

    fn draw(&mut self) {
        self.counter += 1;
        let counter = self.counter as usize;
        function_for_all_pixels(&mut self.image, |x, y| Color::gray(((x & y) ^ counter) as u8));
        let size = Vec2i::new(10, 10);
        rect(&mut self.image, &self.cursor, &size, &Color::rgba(255, 0, 0, 255));
        rect(&mut self.image, &self.click, &size, &Color::rgba(0, 255, 0, 255));
        rect(&mut self.image, &self.wheel, &size, &Color::rgba(0, 0, 255, 255));
    }

    fn resize_event(&mut self, new_size: Vec2i) {
        self.image.resize_lazy(&new_size);
    }

    fn mouse_motion_event(&mut self, pos: Vec2i, _offset: Vec2i) {
        self.cursor = pos;
    }

    fn mouse_button_event(&mut self, _button: MouseButton, _state: ButtonState, pos: Vec2i) {
        self.click = pos;
        info!("Mouse button clicked");
    }

    fn mouse_wheel_event(&mut self, pos: Vec2i, _dir: MouseWheel, _press: bool) {
        self.wheel = pos;
    }
}

fn main() {
    start(Window::new());
}