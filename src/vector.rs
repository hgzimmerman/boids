use std::ops::{Add, Sub, Mul};

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Vec2d {
	pub x: f32,
	pub y: f32,
}

impl Add for Vec2d {
    type Output = Vec2d;

    fn add(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y
		}
    }
}
impl Sub for Vec2d {
    type Output = Vec2d;

    fn sub(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x - rhs.x,
			y: self.y - rhs.y
		}
    }
}

impl Mul<f32> for Vec2d {
    type Output = Vec2d;

    fn mul(self, rhs: f32) -> Self::Output {
		Self {
			x: self.x * rhs,
			y: self.y * rhs
		}
    }
}

impl Vec2d {
	// radians
	pub fn heading(&self) -> f32 {
		f32::atan2(self.x, -self.y)
	}
}