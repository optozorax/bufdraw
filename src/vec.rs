use std::ops;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Vec2i {
    pub x: i32, 
    pub y: i32
}

impl ops::Add<&Vec2i> for Vec2i {
	type Output = Vec2i;

	fn add(self, _rhs: &Vec2i) -> Vec2i {
		Vec2i { 
			x: self.x + _rhs.x, 
			y: self.y + _rhs.y
		}
	}
}

impl ops::Sub<&Vec2i> for Vec2i {
	type Output = Vec2i;

	fn sub(self, _rhs: &Vec2i) -> Vec2i {
		Vec2i { 
			x: self.x - _rhs.x, 
			y: self.y - _rhs.y
		}
	}
}

impl ops::AddAssign for Vec2i {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl ops::SubAssign for Vec2i {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

impl Vec2i {
    pub fn new(x: i32, y: i32) -> Vec2i {
        Vec2i { x, y }
    }
}

impl Default for Vec2i {
    fn default() -> Self {
        Vec2i::new(0, 0)
    }
}

impl From<(i32, i32)> for Vec2i {
    fn from(val: (i32, i32)) -> Self {
        Vec2i::new(val.0, val.1)
    }
}

impl From<(usize, usize)> for Vec2i {
    fn from(val: (usize, usize)) -> Self {
        Vec2i::new(val.0 as i32, val.1 as i32)
    }
}

impl From<(f32, f32)> for Vec2i {
    fn from(val: (f32, f32)) -> Self {
        Vec2i::new(val.0 as i32, val.1 as i32)
    }
}