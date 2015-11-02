use graphics::{DrawState, Transformed, rectangle};
use graphics::polygon::Polygon;
use graphics::line::Line;
use opengl_graphics::GlGraphics;
use piston::input::Key;
use rand::Rng;
use std::f64::consts::PI;
use std::ops::{Sub, Mul, Div};

fn to_cartesian(theta: f64, r: f64) -> (f64, f64) {
    return (
        theta.sin()*r,
        -theta.cos()*r,
    )
}

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

fn lines_intersect(l1: [f64; 4], l2: [f64; 4]) -> bool {
    let p = Point::new(l1[0], l1[1]);
    let q = Point::new(l2[0], l2[1]);
    let r = Point::new(l1[2] - l1[0], l1[3] - l1[1]);
    let s = Point::new(l2[2] - l2[0], l2[3] - l2[1]);

    //If r × s = 0 and (q − p) × r = 0, then the two lines are collinear.
    if (r.cross(s) == 0.0 && (p - q).cross(r) == 0.0) {
        // If the interval between t0 and t1 intersects the interval [0, 1] then the line segments are collinear and overlapping; otherwise they are collinear and disjoint.
        // Note that if s and r point in opposite directions, then s · r < 0 and so the interval to be checked is [t1, t0] rather than [t0, t1].
        //t0 = (q − p) · r / (r · r)
        //t1 = (q + s − p) · r / (r · r) = t0 + s · r / (r · r)
        let t0 = (q - p).dot(r / r.dot(r));
        let t1 = t0 + s.dot(r / r.dot(r));
        return !(t0 < 0.0 && t1 < 0.0 || t0 > 1.0 && t1 > 1.0)
    } 

    // If r × s = 0 and (q − p) × r ≠ 0, then the two lines are parallel and non-intersecting.
    if (r.cross(s) == 0.0) {
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

pub struct Spaceship {
    x: f64,
    y: f64,
    v: f64,
    v_theta: f64,
    theta: f64,
    accel: f64,
    reverse: f64,
    left: f64,
    right: f64,
    firing: bool,
    cooldown: f64,
}

const SPACESHIP_POINTS: [[f64; 2]; 3] = [
    [5.0, 7.0],
    [-5.0, 7.0],
    [0.0, -13.0],
];

impl Spaceship {
    pub fn new() -> Spaceship {
        return Spaceship{
            x: 100.0,
            y: 100.0,
            v: 0.0,
            v_theta: 0.0,
            theta: 0.0,
            accel: 0.0,
            reverse: 0.0,
            left: 0.0,
            right: 0.0,
            firing: false,
            cooldown: 0.0,
        };
    }
    
    pub fn handle_press(&mut self, key: Key) {
        match key {
            Key::Up => self.accel = 1.0,
            Key::Down => self.reverse = 1.0,
            Key::Left => self.left = 0.05,
            Key::Right => self.right = 0.05,
            Key::Space => self.firing = true,
            _ => (),
        }
    }
    
    pub fn handle_release(&mut self, key: Key) {
        match key {
            Key::Up => self.accel = 0.0,
            Key::Down => self.reverse = 0.0,
            Key::Left => self.left = 0.0,
            Key::Right => self.right = 0.0,
            Key::Space => self.firing = false,
            _ => (),
        }
    }
    
    pub fn draw(&mut self, color: [f32; 4], ds: &DrawState, t: [[f64; 3]; 2], gl: &mut GlGraphics) {
        Polygon::new(color).draw(
            &SPACESHIP_POINTS,
            ds,
            t
                .trans(self.x, self.y)
                .rot_rad(self.theta),
            gl,
        );
    }
    
    pub fn go(&mut self, dt: f64, x_max: f64, y_max: f64) {
        let (dx, dy) = to_cartesian(self.v_theta, self.v*dt);
        self.x = (self.x + dx + x_max) % x_max;
        self.y = (self.y + dy + y_max) % y_max;
    }

    pub fn accelerate(&mut self, dt: f64) {
        let net_accel = (self.accel - self.reverse)*dt*100.0;
        let (dx, dy) = to_cartesian(self.v_theta, self.v);
        let (ddx, ddy) = to_cartesian(self.theta, net_accel);
        let new_dx = dx + ddx;
        let new_dy = dy + ddy;
        self.v = (new_dx*new_dx+new_dy*new_dy).sqrt().min(200.0).max(-200.0);
        self.v_theta = new_dx.atan2(-new_dy);
    }

    pub fn turn(&mut self, dt: f64) {
        self.theta += (self.right - self.left)*dt*100.0;
    }
    
    pub fn cooldown(&mut self, dt: f64) {
        self.cooldown = (self.cooldown - dt).max(0.0);
    }
    
    pub fn ready_to_fire(&self) -> bool {
        return self.cooldown == 0.0;
    }
    
    pub fn is_firing(&self) -> bool {
        return self.firing;
    }
    
    pub fn fire(&mut self, bullets: &mut Vec<Bullet>) {
        self.cooldown = 0.5;
        bullets.push(Bullet::new(self.x, self.y, self.theta));
    }
}


pub struct Bullet {
    x: f64,
    y: f64,
    theta: f64,
    distance: f64,
}

impl Bullet {
    fn new(x: f64, y: f64, theta: f64) -> Bullet {
        return Bullet{
            x: x,
            y: y,
            theta: theta,
            distance: 0.0,
        };
    }
    
    pub fn draw(&self, color: [f32; 4], t: [[f64; 3]; 2], gl: &mut GlGraphics) {
        rectangle(color, rectangle::square(self.x, self.y, 2.0), t, gl);
    }

    pub fn go(&mut self, dt: f64, x_max: f64, y_max: f64) {
        let v = 100.0;
        self.x = (self.x + self.theta.sin()*v*dt + x_max) % x_max;
        self.y = (self.y - self.theta.cos()*v*dt + y_max) % y_max;
        self.distance += v*dt;
    }
    
    pub fn is_alive(&self) -> bool {
        return self.distance < 100.0;
    }
}

pub struct Astroid {
    x: f64,
    y: f64,
    v: f64,
    theta: f64,
    border: Vec<[f64; 4]>,
}

impl Astroid {
    pub fn new<R: Rng>(rng: &mut R) -> Astroid {
        let radius = (rng.gen_range(1, 3)*5) as f64;
        return Astroid {
            x: rng.gen_range(0.0, 100.0),
            y: rng.gen_range(0.0, 100.0),
            v: rng.gen_range(40.0, 60.0),
            theta: rng.gen_range(0.0, 2.0*PI),
            border: Astroid::create_border(rng, radius),
        }
    }

    pub fn draw(&self, color: [f32; 4], ds: &DrawState, t: [[f64; 3]; 2], gl: &mut GlGraphics) {
        let line_info = Line::new(color, 0.5);
        for line_points in self.border.iter() {
            line_info.draw(*line_points, ds, t.trans(self.x, self.y), gl);
        }
    }

    pub fn go(&mut self, dt: f64, x_max: f64, y_max: f64) {
        self.x = (self.x + self.theta.sin()*self.v*dt + x_max) % x_max;
        self.y = (self.y - self.theta.cos()*self.v*dt + y_max) % y_max;
    }

    pub fn create_border<R: Rng>(rng: &mut R, radius: f64) -> Vec<[f64; 4]> {
        let spread = radius/5.0;
        let point_count = rng.gen_range(8, 12);
        let mut points = Vec::with_capacity(point_count);
        let theta_0 = rng.gen_range(0.0, 2.0*PI);
        for theta in (1..point_count+1).map(|i| theta_0 + 2.0*PI*i as f64/point_count as f64) {
            let distance = radius + rng.gen_range(-spread, spread);
            points.push(to_cartesian(theta, distance));
        };
        let point = points[0];
        points.push(point);
        let mut lines = Vec::with_capacity(point_count);
        for point in points.iter().zip(points.iter().skip(1)) {
            let (&(x1, x2), &(y1, y2)) = point;
            lines.push([x1, x2, y1, y2]);
        }
        return lines;
    }
    
    pub fn is_collision(&self, bullet_path: [f64; 4]) -> bool {
        return false;
    }
}