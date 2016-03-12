use std::ops::{Sub, Div};

#[derive(Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x: x, y: y }
    }

    pub fn cross(self, other: Point) -> f64 {
        return self.x * other.y - other.x * self.y;
    }

    pub fn dot(self, other: Point) -> f64 {
        return self.x * other.x + self.y * other.y;
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Self::Output {
        return Point {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

impl Div<f64> for Point {
    type Output = Point;

    fn div(self, other: f64) -> Self::Output {
        return Point {
            x: self.x / other,
            y: self.y / other,
        };
    }
}
