use std::ops::{Sub, Div};

#[derive(Copy, Clone, PartialEq, Debug)]
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

#[cfg(test)]
mod test {
    use super::*;
    use expectest::prelude::*;

    const P1: Point = Point { x: 2.0, y: 3.0 };
    const P2: Point = Point { x: 5.0, y: 7.0 };

    #[test]
    fn test_new() {
        expect!(Point::new(2.0, 3.0)).to(be_equal_to(P1));
    }

    #[test]
    fn test_cross() {
        expect!(P1.cross(P2)).to(be_equal_to(-1.0));
    }

    #[test]
    fn test_dot() {
        expect!(P1.dot(P2)).to(be_equal_to(31.0));
    }
}
