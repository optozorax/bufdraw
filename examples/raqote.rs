use bufdraw::*;
use bufdraw::vec::*;
use bufdraw::image::*;

use raqote::*;

use log::info;

struct Window {
    image: DrawTarget,
    counter: u32,
    cursor: Vec2i,
    click: Vec2i,
    wheel: Vec2i,
}

impl ImageTrait for Window {
    fn get_rgba8_buffer(&self) -> &[u8] {
        &self.image.get_data_u8()
    }

    fn get_width(&self) -> usize {
        self.image.width() as usize
    }

    fn get_height(&self) -> usize {
        self.image.height() as usize
    }
}

impl Window {
    fn new() -> Self {
        Window {
            image: DrawTarget::new(1920, 1080),
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
        let dt = &mut self.image;

        dt.clear(SolidSource::from_unpremultiplied_argb(0xff, 0xff, 0xff, 0xff));

        let mut pb = PathBuilder::new();
        pb.move_to(100., 10.);
        pb.cubic_to(150., 40., 175., self.counter as f32, 200., 10.);
        pb.quad_to(120., 100., 80., 200.);
        pb.quad_to(150., 180., 300., 300.);
        pb.close();
        let path = pb.finish();

        let gradient = Source::new_radial_gradient(
            Gradient {
                stops: vec![
                    GradientStop {
                        position: 0.2,
                        color: raqote::Color::new(0xff, 0, 0xff, 0),
                    },
                    GradientStop {
                        position: 0.8,
                        color: raqote::Color::new(0xff, 0xff, 0xff, 0xff),
                    },
                    GradientStop {
                        position: 1.,
                        color: raqote::Color::new(0xff, 0xff, 0, 0xff),
                    },
                ],
            },
            Point::new(150., 150.),
            128.,
            Spread::Pad,
        );
        dt.fill(&path, &gradient, &DrawOptions::new());

        let mut pb = PathBuilder::new();
        pb.move_to(100., 100.);
        pb.line_to(300., 300.);
        pb.line_to(200., 300.);
        let path = pb.finish();

        dt.stroke(
            &path,
            &Source::Solid(SolidSource {
                r: 0x0,
                g: 0x0,
                b: 0x80,
                a: 0x80,
            }),
            &StrokeStyle {
                cap: LineCap::Round,
                join: LineJoin::Round,
                width: 10.,
                miter_limit: 2.,
                dash_array: vec![10., 18.],
                dash_offset: 16.,
            },
            &DrawOptions::new()
        );

        draw_text(
            dt, 
            format!("Русский текст ± ≈ ≠ × ÷ ∃ ∀ {} ∞", self.counter).as_str(), 
            32.0, 
            &Vec2i::new(50, 100), 
            &bufdraw::image::Color::rgba(0, 0, 150, 128)
        );
    }

    fn resize_event(&mut self, new_size: Vec2i) {
        //self.image.resize_lazy(&new_size);
        self.image = DrawTarget::new(new_size.x, new_size.y);
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

use rusttype::{point, Font, Scale};

fn blend(up: &bufdraw::image::Color, low: &bufdraw::image::Color) -> bufdraw::image::Color {
    let upr:i32 = up.r as i32;
    let upg:i32 = up.g as i32;
    let upb:i32 = up.b as i32;
    let upa:i32 = up.a as i32;

    let lowr:i32 = low.r as i32;
    let lowg:i32 = low.g as i32;
    let lowb:i32 = low.b as i32;
    let lowa:i32 = low.a as i32;

    bufdraw::image::Color::rgba(
        (((upr - lowr) * upa + (lowr << 8)) >> 8) as u8,
        (((upg - lowg) * upa + (lowg << 8)) >> 8) as u8,
        (((upb - lowb) * upa + (lowb << 8)) >> 8) as u8,
        ((upa + lowa) - ((lowa * upa + 255) >> 8)) as u8
    )
}

// Modified https://github.com/redox-os/rusttype/blob/master/dev/examples/image.rs
fn draw_text(image: &mut DrawTarget, text: &str, size: f32, pos: &Vec2i, color: &bufdraw::image::Color) {
    // Load the font
    let font_data = include_bytes!("WenQuanYiMicroHei.ttf");
    // This only succeeds if collection consists of one font
    let font = Font::from_bytes(font_data as &[u8]).expect("Error constructing Font");

    // The font size to use
    let scale = Scale::uniform(size);

    let v_metrics = font.v_metrics(scale);

    // layout the glyphs in a line with 20 pixels padding
    let glyphs: Vec<_> = font
        .layout(text, scale, point(20.0, 20.0 + v_metrics.ascent))
        .collect();

    // work out the layout size
    let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
    let glyphs_width = {
        let min_x = glyphs
            .first()
            .map(|g| g.pixel_bounding_box().unwrap().min.x)
            .unwrap();
        let max_x = glyphs
            .last()
            .map(|g| g.pixel_bounding_box().unwrap().max.x)
            .unwrap();
        (max_x - min_x) as u32
    };

    // Loop through the glyphs in the text, positing each one on a line
    for glyph in glyphs {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            // Draw the glyph into the image per-pixel by using the draw closure
            glyph.draw(|x, y, v| {
                let x = (x as i32 + pos.x) as u32 + bounding_box.min.x as u32;
                let y = (y as i32 + pos.y) as u32 + bounding_box.min.y as u32;
                let a = (v * (color.a as f32)) as u8;
                let c = bufdraw::image::Color::rgba(color.b, color.g, color.r, a);
                let pos = x as usize + y as usize * image.width() as usize;
                if x >= 0 && (x as i32) < image.width() && y >= 0 && (y as i32) < image.height() {
                    if let Some(elem) = image.get_data_mut().get_mut(pos) {
                        //(A << 24) | (R << 16) | (G << 8) | B
                        let bytes: [u8; 4] = (*elem).to_be_bytes();
                        let current_color = bufdraw::image::Color::rgba(bytes[1], bytes[2], bytes[3], bytes[0]);
                        let new_color = blend(&c, &current_color);
                        *elem = u32::from_be_bytes([new_color.a, new_color.r, new_color.g, new_color.b]);
                    }    
                }
                
                    
            });
        }
    }
}

fn main() {
    start(Window::new());
}