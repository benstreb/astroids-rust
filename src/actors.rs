use graphics::{DrawState, Transformed, rectangle};
use graphics::polygon::Polygon;
use graphics::line::Line;
use graphics::math::{transform_pos, rotate_radians};
use opengl_graphics::GlGraphics;
use piston::input::Key;
use rand::Rng;
use rand::distributions::{Range, IndependentSample, Normal};
use rand::distributions::range::SampleRange;
use std::f64::consts::PI;
use intersect::{lines_intersect, point_in};
use point::Point;

use config::Config;

pub fn to_cartesian(theta: f64, r: f64) -> (f64, f64) {
    return (theta.sin() * r, -theta.cos() * r);
}

fn random<T: SampleRange + PartialOrd>(low: T, high: T, mut rng: &mut Rng) -> T {
    return Range::new(low, high).ind_sample(&mut rng);
}

pub fn wrapped_add(a: f64, b: f64, bound: f64) -> f64 {
    (a + b + bound) % bound
}

#[derive(Clone)]
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

const SPACESHIP_POINTS: [[f64; 2]; 3] = [[5.0, 7.0], [-5.0, 7.0], [0.0, -13.0]];

impl Spaceship {
    pub fn new(config: &Config) -> Spaceship {
        return Spaceship {
            x: config.width() / 2.0,
            y: config.height() / 2.0,
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

    pub fn draw(&self, color: [f32; 4], ds: &DrawState, t: [[f64; 3]; 2], gl: &mut GlGraphics) {
        Polygon::new(color).draw(&SPACESHIP_POINTS,
                                 ds,
                                 t.trans(self.x, self.y)
                                  .rot_rad(self.theta),
                                 gl);
    }

    pub fn go(&mut self, dt: f64, x_max: f64, y_max: f64) {
        let (dx, dy) = to_cartesian(self.v_theta, self.v * dt);
        self.x = wrapped_add(self.x, dx, x_max);
        self.y = wrapped_add(self.y, dy, y_max);
    }

    pub fn accelerate(&mut self, dt: f64) {
        let net_accel = (self.accel - self.reverse) * dt * 100.0;
        let (dx, dy) = to_cartesian(self.v_theta, self.v);
        let (ddx, ddy) = to_cartesian(self.theta, net_accel);
        let new_dx = dx + ddx;
        let new_dy = dy + ddy;
        self.v = (new_dx * new_dx + new_dy * new_dy).sqrt().min(200.0).max(-200.0);
        self.v_theta = new_dx.atan2(-new_dy);
    }

    pub fn turn(&mut self, dt: f64) {
        self.theta += (self.right - self.left) * dt * 100.0;
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

    pub fn edges(&self) -> Vec<[f64; 4]> {
        let rotation_matrix = rotate_radians(self.theta);
        let points: Vec<[f64; 2]> = SPACESHIP_POINTS.iter()
                                                    .map(|p| transform_pos(rotation_matrix, *p))
                                                    .collect();
        return points.iter()
                     .zip(points.iter().cycle().skip(1))
                     .map(|(p1, p2)| {
                         [p1[0] + self.x, p1[1] + self.y, p2[0] + self.x, p2[1] + self.y]
                     })
                     .collect();
    }

    pub fn collides<I: Iterator<Item = [f64; 4]>>(&self, edges: I) -> bool {
        let edges_vec: Vec<[f64; 4]> = edges.collect();
        return self.edges().iter().any(|edge| {
            edges_vec.iter().any(|other_edge| lines_intersect(*edge, *other_edge))
        });
    }
}

#[derive(Clone, Debug)]
pub struct Bullet {
    x: f64,
    y: f64,
    theta: f64,
    distance: f64,
}

impl Bullet {
    fn new(x: f64, y: f64, theta: f64) -> Bullet {
        return Bullet {
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
        self.x = wrapped_add(self.x, self.theta.sin() * v * dt, x_max);
        self.y = wrapped_add(self.y, -self.theta.cos() * v * dt, y_max);
        self.distance += v * dt;
    }

    pub fn is_alive(&self) -> bool {
        return self.distance < 100.0;
    }

    pub fn coords(&self) -> Point {
        return Point::new(self.x, self.y);
    }

    pub fn collides(&self, astroid: &Astroid) -> bool {
        return point_in(self.coords(), astroid.edges().iter().cloned());
    }
}

#[derive(Clone)]
pub struct Astroid {
    x: f64,
    y: f64,
    v: f64,
    theta: f64,
    size: i64,
    border: Vec<[f64; 4]>,
}

const ASTROID_LARGE: i64 = 3;

impl Astroid {
    pub fn large_new(config: &Config, mut rng: &mut Rng) -> Astroid {
        return Self::new(ASTROID_LARGE, config, rng);
    }

    fn random_start(max: f64, gap: f64, mut rng: &mut Rng) -> f64 {
        if random(0, 2, rng) == 0 {
            return random(0.0, max / 2.0 - gap, rng);
        } else {
            return random(max / 2.0 + gap, max, rng);
        }
    }

    pub fn new(size: i64, config: &Config, mut rng: &mut Rng) -> Astroid {
        let radius = (size * 5) as f64;
        return Astroid {
            x: Astroid::random_start(config.width(), config.astroid_gap_distance(), &mut rng),
            y: Astroid::random_start(config.height(), config.astroid_gap_distance(), &mut rng),
            v: random(40.0, 60.0, &mut rng),
            theta: random(0.0, 2.0 * PI, &mut rng),
            size: size,
            border: Astroid::create_border(&mut rng, radius),
        };
    }

    fn exploded(&self, mut rng: &mut Rng) -> Astroid {
        let new_size = self.size - 1;
        let radius = (new_size * 5) as f64;
        let theta_range = Normal::new(0.0, PI / 2.0);
        let d_theta = theta_range.ind_sample(&mut rng);
        let theta = self.theta + d_theta;
        return Astroid {
            x: self.x + random(-5.0, 5.0, &mut rng),
            y: self.y + random(-5.0, 5.0, &mut rng),
            v: random(40.0, 60.0, &mut rng),
            theta: theta,
            size: new_size,
            border: Astroid::create_border(&mut rng, radius),
        };
    }

    pub fn explode(&self, mut rng: &mut Rng) -> Vec<Astroid> {
        if self.size <= 1 {
            vec![]
        } else {
            vec![self.exploded(rng), self.exploded(rng)]
        }
    }

    pub fn draw(&self, color: [f32; 4], ds: &DrawState, t: [[f64; 3]; 2], gl: &mut GlGraphics) {
        let line_info = Line::new(color, 0.5);
        for line_points in self.border.iter() {
            line_info.draw(*line_points, ds, t.trans(self.x, self.y), gl);
        }
    }

    pub fn go(&mut self, dt: f64, x_max: f64, y_max: f64) {
        self.x = wrapped_add(self.x, self.theta.sin() * self.v * dt, x_max);
        self.y = wrapped_add(self.y, -self.theta.cos() * self.v * dt, y_max);
    }

    pub fn create_border(mut rng: &mut Rng, radius: f64) -> Vec<[f64; 4]> {
        let spread = radius / 5.0;
        let point_count = random(8, 12, &mut rng);
        let mut points = Vec::with_capacity(point_count);
        let theta_0 = random(0.0, 2.0 * PI, &mut rng);
        let corner_i_to_theta = |i| theta_0 + 2.0 * PI * i as f64 / point_count as f64;
        for theta in (1..point_count + 1).map(corner_i_to_theta) {
            let distance = radius + random(-spread, spread, &mut rng);
            points.push(to_cartesian(theta, distance));
        }
        let point = points[0];
        points.push(point);
        let mut lines = Vec::with_capacity(point_count);
        for point in points.iter().zip(points.iter().skip(1)) {
            let (&(x1, x2), &(y1, y2)) = point;
            lines.push([x1, x2, y1, y2]);
        }
        return lines;
    }

    pub fn edges(&self) -> Vec<[f64; 4]> {
        return self.border
                   .iter()
                   .map(|edge| {
                       [edge[0] + self.x, edge[1] + self.y, edge[2] + self.x, edge[3] + self.y]
                   })
                   .collect();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::f64::consts::PI;
    use expectest::prelude::*;

    #[test]
    fn test_wrapped_add() {
        assert_eq!(wrapped_add(1.0, 1.0, 10.0), 2.0);
        assert_eq!(wrapped_add(1.0, 10.0, 10.0), 1.0);
        assert_eq!(wrapped_add(1.0, -5.0, 10.0), 6.0);
    }

    fn expect_both_close_to((r, theta): (f64, f64), (x_expected, y_expected): (f64, f64)) {
        match to_cartesian(r, theta) {
            (x, y) => {
                expect!(x_expected).to(be_close_to(x));
                expect!(y_expected).to(be_close_to(y));
            }
        }
    }

    #[test]
    fn test_to_cartesian() {
        expect_both_close_to((0.0, 1.0), (0.0, -1.0));
        expect_both_close_to((PI / 2.0, 1.0), (1.0, 0.0));
    }
}
