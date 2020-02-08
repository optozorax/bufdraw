use std::ops;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Vec2i {
	pub x: i32, 
	pub y: i32
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Rect2i {
	pub min: Vec2i,
	pub max: Vec2i,
}

impl Rect2i {
	pub fn is_inside(&self, pos: &Vec2i) -> bool {
		pos.x >= self.min.x && pos.y >= self.min.y &&
		pos.x <  self.max.x && pos.y <  self.max.y
	}

	pub fn is_intersect(&self, rect: &Rect2i) -> bool {
		self.is_inside(&rect.min) || 
		self.is_inside(&rect.max) ||
		self.is_inside(&Vec2i::new(rect.min.x, rect.max.y)) ||
		self.is_inside(&Vec2i::new(rect.max.x, rect.min.y))

		||

		rect.is_inside(&rect.min) || 
		rect.is_inside(&rect.max) ||
		rect.is_inside(&Vec2i::new(self.min.x, self.max.y)) ||
		rect.is_inside(&Vec2i::new(self.max.x, self.min.y))
	}
}

impl ops::Add<&Vec2i> for Vec2i {
	type Output = Vec2i;

	#[inline]
	fn add(self, _rhs: &Vec2i) -> Vec2i {
		Vec2i { 
			x: self.x + _rhs.x, 
			y: self.y + _rhs.y
		}
	}
}

impl ops::Sub<&Vec2i> for Vec2i {
	type Output = Vec2i;

	#[inline]
	fn sub(self, _rhs: &Vec2i) -> Vec2i {
		Vec2i { 
			x: self.x - _rhs.x, 
			y: self.y - _rhs.y
		}
	}
}

impl ops::Mul<i32> for Vec2i {
	type Output = Vec2i;

	#[inline]
	fn mul(self, _rhs: i32) -> Vec2i {
		Vec2i { 
			x: self.x * _rhs, 
			y: self.y * _rhs
		}
	}
}

impl ops::Mul<f32> for Vec2i {
	type Output = Vec2i;

	#[inline]
	fn mul(self, _rhs: f32) -> Vec2i {
		Vec2i { 
			x: (self.x as f32 * _rhs) as i32, 
			y: (self.y as f32 * _rhs) as i32
		}
	}
}

impl ops::Div<i32> for Vec2i {
	type Output = Vec2i;

	#[inline]
	fn div(self, _rhs: i32) -> Vec2i {
		Vec2i { 
			x: self.x / _rhs, 
			y: self.y / _rhs
		}
	}
}


impl ops::AddAssign for Vec2i {
	#[inline]
	fn add_assign(&mut self, other: Self) {
		*self = Self {
			x: self.x + other.x,
			y: self.y + other.y,
		};
	}
}

impl ops::SubAssign for Vec2i {
	#[inline]
	fn sub_assign(&mut self, other: Self) {
		*self = Self {
			x: self.x - other.x,
			y: self.y - other.y,
		};
	}
}

impl Vec2i {
	#[inline]
	pub fn new(x: i32, y: i32) -> Vec2i {
		Vec2i { x, y }
	}

	pub fn len(&self) -> f32 {
		((self.x.pow(2) + self.y.pow(2)) as f32).sqrt()
	}
}

impl Default for Vec2i {
	#[inline]
	fn default() -> Self {
		Vec2i::new(0, 0)
	}
}

impl From<(i32, i32)> for Vec2i {
	#[inline]
	fn from(val: (i32, i32)) -> Self {
		Vec2i::new(val.0, val.1)
	}
}

impl From<(usize, usize)> for Vec2i {
	#[inline]
	fn from(val: (usize, usize)) -> Self {
		Vec2i::new(val.0 as i32, val.1 as i32)
	}
}

impl From<(f32, f32)> for Vec2i {
	#[inline]
	fn from(val: (f32, f32)) -> Self {
		Vec2i::new(val.0 as i32, val.1 as i32)
	}
}

pub fn next_in_rect(pos: &Vec2i, size: &Vec2i) -> Option<Vec2i> {
	if pos.x < size.x - 1 {
		return Some(Vec2i::new(pos.x+1, pos.y))
	}
	
	if pos.y < size.y - 1 {
		Some(Vec2i::new(0, pos.y + 1))
	} else {
		None
	}
}
