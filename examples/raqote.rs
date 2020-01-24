use bufdraw::*;
use bufdraw::vec::*;

use raqote::*;
use font_kit::source::SystemSource;
use font_kit::family_name::FamilyName;
use font_kit::properties::Properties;

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
        pb.cubic_to(150., 40., 175., 0., 200., 10.);
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

        let font = SystemSource::new()
            .select_best_match(&[FamilyName::SansSerif], &Properties::new())
            .unwrap()
            .load()
            .unwrap();

        dt.draw_text(
            &font,
            24.,
            "Hello",
            Point::new(0., 100.),
            &Source::Solid(SolidSource {
                r: 0,
                g: 0,
                b: 0xff,
                a: 0xff,
            }),
            &DrawOptions::new(),
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

fn main() {
    start(Window::new());
}