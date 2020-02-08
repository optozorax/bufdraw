use crate::vec::Vec2i;

pub struct MinFinder<T>(Option<T>);
pub struct MaxFinder<T>(Option<T>);

pub trait MinMaxFind<T> {
	fn process(&mut self, value: &T);
}

pub struct MinMaxFinder<T> {
	min: MinFinder<T>,
	max: MaxFinder<T>,
}

pub struct BoundingBoxFinder {
	x: MinMaxFinder<i32>,
	y: MinMaxFinder<i32>,
}

impl<T> Default for MinFinder<T> {
	fn default() -> Self {
		MinFinder(None)
	}
}

impl<T> Default for MaxFinder<T> {
	fn default() -> Self {
		MaxFinder(None)
	}
}

impl<T> Default for MinMaxFinder<T> {
	fn default() -> Self {
		MinMaxFinder { min: MinFinder::default(), max: MaxFinder::default() }
	}
}

impl Default for BoundingBoxFinder {
	fn default() -> Self {
		BoundingBoxFinder { x: MinMaxFinder::default(), y: MinMaxFinder::default() }
	}
}

impl<T: std::cmp::Ord + Clone> MinMaxFind<T> for MinFinder<T> {
	fn process(&mut self, value: &T) {
		self.0 = match self.0.clone() {
			Some(v) => Some(value.clone().min(v)),
			None => Some(value.clone()),
		};
	}
}

impl<T: std::cmp::Ord + Clone> MinMaxFind<T> for MaxFinder<T> {
	fn process(&mut self, value: &T) {
		self.0 = match self.0.clone() {
			Some(v) => Some(value.clone().max(v)),
			None => Some(value.clone()),
		};
	}
}

impl<T: std::cmp::Ord + Clone> MinMaxFind<T> for MinMaxFinder<T> {
	fn process(&mut self, value: &T) {
		self.min.process(value);
		self.max.process(value);
	}
}

impl<'a, T: 'static + std::ops::Sub<&'a T, Output = T> + Clone> MinMaxFinder<T> {
	pub fn length(&'a self) -> Option<T> {
		Some(self.max.0.clone()? - self.min.0.as_ref()?)
	}
}

impl MinMaxFind<Vec2i> for BoundingBoxFinder {
	fn process(&mut self, point: &Vec2i) {
		self.x.process(&point.x);
		self.y.process(&point.y);
	}
}

impl BoundingBoxFinder {
	pub fn min(&self) -> Option<Vec2i> {
		Some(Vec2i::new(self.x.min.0?, self.y.min.0?))
	}

	pub fn max(&self) -> Option<Vec2i> {
		Some(Vec2i::new(self.x.max.0?, self.y.max.0?))
	}

	pub fn size(&self) -> Option<Vec2i> {
		Some(Vec2i::new(self.x.length()?, self.y.length()?))
	}
}