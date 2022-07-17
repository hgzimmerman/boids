use std::ops::{Add, Sub, Mul, Div, Neg, SubAssign, AddAssign};

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
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

impl AddAssign for Vec2d {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
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
impl SubAssign for Vec2d {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
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
impl Div<f32> for Vec2d {
    type Output = Vec2d;

    fn div(self, rhs: f32) -> Self::Output {
		Self {
			x: self.x / rhs,
			y: self.y / rhs
		}
    }
}

impl Neg for Vec2d {
    type Output = Vec2d;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y
        }
    }
}

impl Vec2d {
	// radians
	pub fn heading(&self) -> f32 {
        // Invert y axis because the 0,0 point is in the top-left, making the y-axis inverted
		f32::atan2(self.x, -self.y)
	}
    /// Slightly better, no allocs, but no indexing either
    #[allow(unused)]
    pub fn naive_n_closest2<'a>(&self, list: &'a mut [Self], n: usize) -> &'a [Self] {
        list.sort_by_key(|rhs| ordered_float::OrderedFloat(self.distance(rhs)));
        let n = std::cmp::min(list.len(), n-1);
        &list[1..n]
    }
    pub fn distance(&self, rhs: &Self) -> f32 {
        let lhs= self;
        return f32::sqrt(
            (lhs.x - rhs.x).powi(2) + (lhs.y - rhs.y).powi(2)
        )
    }
    /// normalizes the vector so that the total length is 1 
    pub fn normalize_vector(self) -> Self{
        self / self.length()
    }
    pub fn clamp(self, max: f32) -> Self{
        if self.length() > max {
            self.set_length(max)
        } else {
            self
        }
    }
    #[inline]
    pub fn length(self) -> f32 {
        f32::sqrt(self.x.powi(2) + self.y.powi(2))
    }
    pub fn set_length(self, length: f32) -> Self {
        let old_length = self.length();
        let factor = length / old_length;
        Self { 
            x: self.x * factor,
            y: self.y * factor 
        }
    }
}