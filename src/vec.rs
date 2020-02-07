use std::ops;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Vec2i {
    pub x: i32, 
    pub y: i32
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
