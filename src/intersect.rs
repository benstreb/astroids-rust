use std::ops::{Sub, Div};

#[derive(Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Point {
            x: x,
            y: y,
        }
    }

    fn cross(self, other: Point) -> f64 {
        return self.x*other.y - other.x*self.y;
    }

    fn dot(self, other: Point) -> f64 {
        return self.x*other.x + self.y*other.y;
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Self::Output {
        return Point{
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Div<f64> for Point {
    type Output = Point;

    fn div(self, other: f64) -> Self::Output {
        return Point{
            x: self.x / other,
            y: self.y / other,
        }
    }
}

// Intersection of p, p+r and q, q+s
// For details, See: http://stackoverflow.com/questions/563198/how-do-you-detect-where-two-line-segments-intersect
// p + t r = q + u s
// t = ((q − p) × s) / (r × s)
// u = ((q − p) × r) / (r × s)

pub fn lines_intersect(l1: [f64; 4], l2: [f64; 4]) -> bool {
    let p = Point::new(l1[0], l1[1]);
    let q = Point::new(l2[0], l2[1]);
    let r = Point::new(l1[2] - l1[0], l1[3] - l1[1]);
    let s = Point::new(l2[2] - l2[0], l2[3] - l2[1]);

    //If r × s = 0 and (q − p) × r = 0, then the two lines are collinear.
    if r.cross(s) == 0.0 && (p - q).cross(r) == 0.0 {
        // If the interval between t0 and t1 intersects the interval [0, 1] then the line segments are collinear and overlapping; otherwise they are collinear and disjoint.
        // Note that if s and r point in opposite directions, then s · r < 0 and so the interval to be checked is [t1, t0] rather than [t0, t1].
        //t0 = (q − p) · r / (r · r)
        //t1 = (q + s − p) · r / (r · r) = t0 + s · r / (r · r)
        let t0 = (q - p).dot(r / r.dot(r));
        let t1 = t0 + s.dot(r / r.dot(r));
        return !(t0 < 0.0 && t1 < 0.0 || t0 > 1.0 && t1 > 1.0)
    }

    // If r × s = 0 and (q − p) × r ≠ 0, then the two lines are parallel and non-intersecting.
    if r.cross(s) == 0.0 {
        return false;
    }

    // t = (q − p) × s / (r × s)
    // u = (p − q) × r / (s × r)
    let t = (q - p).cross(s / (r.cross(s)));
    let u = (p - q).cross(r / (s.cross(r)));

    // If r × s ≠ 0 and 0 ≤ t ≤ 1 and 0 ≤ u ≤ 1, the two line segments meet at the point p + t r = q + u s.
    return 0.0 <= t && t <= 1.0 && 0.0 <= u && u <= 1.0;
}

#[cfg(test)]
#[test]
fn test_lines_intersect() {
    let line = [-1.0, -1.0, 1.0, 1.0];
    // Two lines intersect normally
    assert!(lines_intersect(line, [1.0, -1.0, -1.0, 1.0]));
    // Two lines don't intersect at all
    assert!(!lines_intersect(line, [-2.0, -2.0, -3.0, -3.0]));
    // Two lines are parallel and don't intersect
    assert!(!lines_intersect(line, [-1.0, 0.0, 1.0, 2.0]));
    // Two lines are co-linear and overlapping
    assert!(lines_intersect(line, [-1.5, -1.5, 0.5, 0.5]));
    assert!(lines_intersect(line, [0.5, 0.5, 1.5, 1.5]));
    // Two lines are co-linear and one contains the other
    assert!(lines_intersect(line, [-2.0, -2.0, 2.0, 2.0]));
    assert!(lines_intersect(line, [-0.5, -0.5, 0.5, 0.5]));
    // Two lines are co-linear and non-overlapping
    assert!(!lines_intersect(line, [-3.0, -3.0, -2.0, -2.0]));
    assert!(!lines_intersect(line, [2.0, 2.0, 3.0, 3.0]));
    // Two lines are the same
    assert!(lines_intersect(line, line));
    // Two lines intersect at one point
    assert!(lines_intersect(line, [1.0, 1.0, 2.0, 2.0]));
    assert!(lines_intersect(line, [1.0, 1.0, 1.0, 2.0]));
    // One line's point touches the other line
    assert!(lines_intersect(line, [0.0, 2.0, 2.0, 0.0]));
}
