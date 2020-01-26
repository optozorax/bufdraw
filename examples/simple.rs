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
    text: String,
    rect_size: i32,

    one_touch: Option<Vec2i>,
    rect_start: i32,
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
        let font_data = include_bytes!("Anonymous.ttf");
        Window {
            image: Image::new(&Vec2i::new(1920, 1080)),
            counter: 0,
            cursor: Vec2i::default(),
            click: Vec2i::default(),
            wheel: Vec2i::default(),
            font: Font::from_bytes(font_data as &[u8]).expect("Error constructing Font"),
            text: String::from("nothing\n"),
            rect_size: 10,

            one_touch: None,
            rect_start: 0,
        }
    }
}

impl MyEvents for Window {
    fn update(&mut self) {

    }

    fn draw(&mut self) {
        self.counter += 1;
        let counter = self.counter as usize;
        //function_for_all_pixels(&mut self.image, |x, y| Color::gray(((x & y) ^ counter) as u8));
        self.image.clear(&Color::gray(0));
        let size = Vec2i::new(self.rect_size, self.rect_size);
        rect(&mut self.image, &self.cursor, &size, &Color::rgba(255, 0, 0, 255));
        rect(&mut self.image, &self.click, &size, &Color::rgba(0, 255, 0, 255));
        rect(&mut self.image, &self.wheel, &size, &Color::rgba(0, 0, 255, 255));

        if let Some(pos) = &self.one_touch {
            rect(&mut self.image, pos, &size, &Color::rgba(0, 255, 255, 255));
        }

        let max_lines: i32 = 50;
        let text_arr: Vec<&str> = self.text.split('\n').collect();
        let len: i32 = text_arr.len() as i32;
        let text_arr: Vec<String> = text_arr.iter().skip(0.max(len - max_lines) as usize).map(|&x| x.to_string()).collect();
        self.text = text_arr.join("\n");
        draw_text(&mut self.image, &self.font, self.text.as_str(), 20.0, &Vec2i::new(10, 10), &Color::rgba(0, 190, 190, 128));
    }

    fn resize_event(&mut self, new_size: Vec2i) {
        self.image.resize_lazy(&new_size);
    }

    fn mouse_motion_event(&mut self, pos: Vec2i, _offset: Vec2i) {
        self.cursor = pos;
    }

    fn mouse_button_event(&mut self, button: MouseButton, _state: ButtonState, pos: Vec2i) {
        self.click = pos;
        self.text += format!("Mouse button clicked: {:?}\n", button).as_str();
    }

    fn mouse_wheel_event(&mut self, pos: Vec2i, dir_vertical: MouseWheelVertical, dir_horizontal: MouseWheelHorizontal) {
        self.wheel = pos.clone();
        self.text += format!("Mouse wheel rotated: {:?}, {:?}, {:?}\n", pos, dir_vertical, dir_horizontal).as_str();
        use MouseWheelHorizontal::*;
        match dir_horizontal {
            RotateUp => if self.rect_size < 255 { self.rect_size += 1; },
            RotateDown => if self.rect_size > 0 { self.rect_size -= 1; },
        }
    }

    fn touch_one_start(&mut self, pos: &Vec2i) {
        self.one_touch = Some(pos.clone());
    }
    fn touch_one_move(&mut self, pos: &Vec2i, _offset: &Vec2i) {
        self.one_touch = Some(pos.clone());
    }
    fn touch_one_end(&mut self) {
        self.one_touch = None;
    }

    fn touch_scale_start(&mut self) {
        self.rect_start = self.rect_size;
    }
    fn touch_scale_change(&mut self, scale: f32) {
        self.rect_size = (self.rect_start as f32 * scale) as i32;
    }
    fn touch_scale_end(&mut self) {
        self.rect_start = self.rect_size;
    }

    fn touch_start_event(&mut self, touches: &Vec<Touch>) {
        for touch in touches {
            self.text += format!("start id: {}, x: {:.1}, y: {:.1}\n", touch.id, touch.x, touch.y).as_str();
        }
    }

    fn touch_end_event(&mut self, touches: &Vec<Touch>) {
        for touch in touches {
            self.text += format!("end id: {}, x: {:.1}, y: {:.1}\n", touch.id, touch.x, touch.y).as_str();
        }
    }

    fn touch_cancel_event(&mut self, touches: &Vec<Touch>) {
        for touch in touches {
            self.text += format!("cancel id: {}, x: {:.1}, y: {:.1}\n", touch.id, touch.x, touch.y).as_str();
        }
    }

    fn touch_move_event(&mut self, touches: &Vec<Touch>) {
        for touch in touches {
            self.text += format!("move id: {}, x: {:.1}, y: {:.1}\n", touch.id, touch.x, touch.y).as_str();
        }
    }
}

fn main() {
    start(Window::new());
}