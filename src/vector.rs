use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

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
            y: self.y + rhs.y,
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
            y: self.y - rhs.y,
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
            y: self.y * rhs,
        }
    }
}
impl Div<f32> for Vec2d {
    type Output = Vec2d;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Neg for Vec2d {
    type Output = Vec2d;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Vec2d {
    // radians
    pub fn heading(&self) -> f32 {
        // Invert y axis because the 0,0 point is in the top-left, making the y-axis inverted
        f32::atan2(self.x, -self.y)
    }
    /// Naive way to get the closest items in a list:
    /// No allocs, but no use of advanced datastructures for improved indexing either, relies on sorting.
    ///
    /// O(n log_n) when used to search for all items, since it involves sorting the slice for every indiviual boid to find the closest neighbors.
    #[allow(unused)]
    pub fn naive_n_closest2<'a>(&self, list: &'a mut [Self], n: usize) -> &'a [Self] {
        list.sort_by_key(|rhs| ordered_float::OrderedFloat(self.distance(rhs)));
        let n = std::cmp::min(list.len(), n - 1);
        &list[1..n]
    }
    /// Distance between two points
    pub fn distance(&self, rhs: &Self) -> f32 {
        (*self - *rhs).length()
    }
    /// Normalizes the vector so that the total length is 1.
    pub fn normalize(self) -> Self {
        self / self.length()
    }
    /// Preserves the direction of the vector while setting a maximum value for the length
    pub fn clamp(self, max: f32) -> Self {
        let length = self.length();
        if length > max {
            self.set_length_inner(length, max)
        } else {
            self
        }
    }
    /// Length of the vector
    #[inline]
    pub fn length(self) -> f32 {
        f32::sqrt(self.x.powi(2) + self.y.powi(2))
    }
    #[allow(unused)]
    pub fn set_length(self, length: f32) -> Self {
        let old_length = self.length();
        self.set_length_inner(old_length, length)
    }
    fn set_length_inner(self, old_length: f32, new_length: f32) -> Self {
        let factor = new_length / old_length;
        Self {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
}
