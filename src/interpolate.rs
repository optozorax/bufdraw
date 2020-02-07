use crate::image::Color;

pub trait Interpolate {
	fn interpolate(&self, b: &Self, t: f64) -> Self;
}

impl Interpolate for f64 {
	fn interpolate(&self, b: &Self, t: f64) -> Self {
		self + (b - self) * t
	}
}

impl Interpolate for Color {
	fn interpolate(&self, b: &Self, t: f64) -> Self {
		let a = self.to_rgba_f64();
		let b = b.to_rgba_f64();
		Color::rgba_f64(
			a.0.interpolate(&b.0, t),
			a.1.interpolate(&b.1, t),
			a.2.interpolate(&b.2, t),
			a.3.interpolate(&b.3, t),
		)
	}
}