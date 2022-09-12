use std::fmt;
use std::ops::{AddAssign, Mul, Sub, MulAssign};

#[derive(Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32
}

impl Point {
    pub fn origin() -> Point {
        p(0, 0)
    }

    /// Return the distance between the two points, squared
    pub fn distance_sq(&self, rhs: &Point) -> f32 {
        let a = self.x - rhs.x;
        let b = self.y - rhs.y;
        return (a * a) + (b * b);
    }

    /// Compute the inner product
    pub fn inner(self, rhs: Point) -> f32 {
        (self.x * rhs.x) + (self.y * rhs.y)
    }

    pub fn magnitude(self) -> f32 {
        ((self.x * self.x) + (self.y * self.y)).sqrt()
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<f32> for Point {
    type Output = Point;

    fn mul(self, rhs: f32) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign<f32> for Point {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Self{
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

const fn p(x: isize, y: isize) -> Point {
    Point{
        x: x as f32,
        y: y as f32
    }
}

