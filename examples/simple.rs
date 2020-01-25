use bufdraw::*;
use bufdraw::vec::*;
use bufdraw::image::*;
use bufdraw::text::*;

use log::info;

struct Window {
    image: Image,
    counter: u32,
    cursor: Vec2i,
    click: Vec2i,
    wheel: Vec2i,
    font: Font<'static>,
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
        let font_data = include_bytes!("WenQuanYiMicroHei.ttf");
        Window {
            image: Image::new(&Vec2i::new(1920, 1080)),
            counter: 0,
            cursor: Vec2i::default(),
            click: Vec2i::default(),
            wheel: Vec2i::default(),
            font: Font::from_bytes(font_data as &[u8]).expect("Error constructing Font"),
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
        draw_text(&mut self.image, &self.font, "Раз два три четыре", 50.0, &Vec2i::new(10, 10), &Color::rgba(0, 190, 190, 128));
    }

    fn resize_event(&mut self, new_size: Vec2i) {
        self.image.resize_lazy(&new_size);
    }

    fn mouse_motion_event(&mut self, pos: Vec2i, _offset: Vec2i) {
        self.cursor = pos;
    }

    fn mouse_button_event(&mut self, button: MouseButton, _state: ButtonState, pos: Vec2i) {
        self.click = pos;
        info!("Mouse button clicked: {:?}", button);
    }

    fn mouse_wheel_event(&mut self, pos: Vec2i, dir_vertical: MouseWheelVertical, dir_horizontal: MouseWheelHorizontal) {
        self.wheel = pos.clone();
        info!("Mouse wheel rotated: {:?}, {:?}, {:?}", pos, dir_vertical, dir_horizontal);
    }
}

fn main() {
    start(Window::new());
}