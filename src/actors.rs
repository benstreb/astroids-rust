use crate::config::Config;
use crate::intersect::{lines_intersect, point_in};
use crate::point::Point;
use graphics::line::Line;
use graphics::math::{rotate_radians, transform_pos};
use graphics::polygon::Polygon;
use graphics::{rectangle, DrawState, Transformed};
use opengl_graphics::GlGraphics;
use piston::input::Key;
use rand::{Rng, RngCore};
use rand_distr::uniform::SampleUniform;
use rand_distr::Normal;
use std::f64::consts::PI;

pub fn to_cartesian(theta: f64, r: f64) -> (f64, f64) {
    return (theta.sin() * r, -theta.cos() * r);
}

pub fn to_polar(x: f64, y: f64) -> (f64, f64) {
    return (x.atan2(-y), (x * x + y * y).sqrt());
}

fn random<T: std::cmp::PartialOrd + SampleUniform>(low: T, high: T, rng: &mut dyn RngCore) -> T {
    rng.gen_range(low..high)
}

pub fn wrapped_add(a: f64, b: f64, bound: f64) -> f64 {
    (a + b + bound) % bound
}

#[derive(PartialEq, Debug, Clone)]
pub struct GameObject {
    x: f64,
    y: f64,
    v: f64,
    theta: f64,
}

impl GameObject {
    fn new(x: f64, y: f64, v: f64, theta: f64) -> GameObject {
        GameObject {
            x: x,
            y: y,
            v: v,
            theta: theta,
        }
    }

    pub fn with_go(&self, dt: f64, x_max: f64, y_max: f64) -> GameObject {
        let (dx, dy) = to_cartesian(self.theta, self.v * dt);
        GameObject::new(
            wrapped_add(self.x, dx, x_max),
            wrapped_add(self.y, dy, y_max),
            self.v,
            self.theta,
        )
    }
}

#[derive(Clone)]
pub struct Spaceship {
    obj: GameObject,
    sprite_theta: f64,
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
            obj: GameObject::new(config.width() / 2.0, config.height() / 2.0, 0.0, 0.0),
            sprite_theta: 0.0,
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
        Polygon::new(color).draw(
            &SPACESHIP_POINTS,
            ds,
            t.trans(self.obj.x, self.obj.y).rot_rad(self.sprite_theta),
            gl,
        );
    }

    pub fn go(&mut self, dt: f64, x_max: f64, y_max: f64) {
        self.obj = self.obj.with_go(dt, x_max, y_max);
    }

    pub fn accelerate(&mut self, dt: f64) {
        let net_accel = (self.accel - self.reverse) * dt * 100.0;
        let (dx, dy) = to_cartesian(self.obj.theta, self.obj.v);
        let (ddx, ddy) = to_cartesian(self.sprite_theta, net_accel);
        let new_dx = dx + ddx;
        let new_dy = dy + ddy;
        let (new_theta, new_v) = to_polar(new_dx, new_dy);
        self.obj.v = new_v.min(200.0).max(-200.0);
        self.obj.theta = new_theta;
    }

    pub fn turn(&mut self, dt: f64) {
        self.sprite_theta += (self.right - self.left) * dt * 100.0;
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
        bullets.push(Bullet::new(self.obj.x, self.obj.y, self.sprite_theta));
    }

    pub fn edges(&self) -> Vec<[f64; 4]> {
        let rotation_matrix = rotate_radians(self.sprite_theta);
        let points: Vec<[f64; 2]> = SPACESHIP_POINTS
            .iter()
            .map(|p| transform_pos(rotation_matrix, *p))
            .collect();
        return points
            .iter()
            .zip(points.iter().cycle().skip(1))
            .map(|(p1, p2)| {
                [
                    p1[0] + self.obj.x,
                    p1[1] + self.obj.y,
                    p2[0] + self.obj.x,
                    p2[1] + self.obj.y,
                ]
            })
            .collect();
    }

    pub fn collides<I: Iterator<Item = [f64; 4]>>(&self, edges: I) -> bool {
        let edges_vec: Vec<[f64; 4]> = edges.collect();
        return self.edges().iter().any(|edge| {
            edges_vec
                .iter()
                .any(|other_edge| lines_intersect(*edge, *other_edge))
        });
    }
}

#[derive(Clone, Debug)]
pub struct Bullet {
    obj: GameObject,
    distance: f64,
}

impl Bullet {
    fn new(x: f64, y: f64, theta: f64) -> Bullet {
        return Bullet {
            obj: GameObject::new(x, y, 100.0, theta),
            distance: 0.0,
        };
    }

    pub fn draw(&self, color: [f32; 4], t: [[f64; 3]; 2], gl: &mut GlGraphics) {
        rectangle(color, rectangle::square(self.obj.x, self.obj.y, 2.0), t, gl);
    }

    pub fn go(&mut self, dt: f64, x_max: f64, y_max: f64) {
        self.obj = self.obj.with_go(dt, x_max, y_max);
        self.distance += self.obj.v * dt;
    }

    pub fn is_alive(&self) -> bool {
        return self.distance < 100.0;
    }

    pub fn coords(&self) -> Point {
        return Point::new(self.obj.x, self.obj.y);
    }

    pub fn collides(&self, astroid: &Astroid) -> bool {
        return point_in(self.coords(), astroid.edges().iter().cloned());
    }
}

#[derive(Clone)]
pub struct Astroid {
    obj: GameObject,
    size: i64,
    border: Vec<[f64; 4]>,
}

const ASTROID_LARGE: i64 = 3;

impl Astroid {
    pub fn large_new(config: &Config, rng: &mut dyn RngCore) -> Astroid {
        return Self::new(ASTROID_LARGE, config, rng);
    }

    fn random_start(max: f64, gap: f64, rng: &mut dyn RngCore) -> f64 {
        if random(0, 2, rng) == 0 {
            return random(0.0, max / 2.0 - gap, rng);
        } else {
            return random(max / 2.0 + gap, max, rng);
        }
    }

    pub fn new(size: i64, config: &Config, mut rng: &mut dyn RngCore) -> Astroid {
        let radius = (size * 5) as f64;
        return Astroid {
            obj: GameObject::new(
                Astroid::random_start(config.width(), config.astroid_gap_distance(), &mut rng),
                Astroid::random_start(config.height(), config.astroid_gap_distance(), &mut rng),
                random(40.0, 60.0, &mut rng),
                random(0.0, 2.0 * PI, &mut rng),
            ),
            size: size,
            border: Astroid::create_border(&mut rng, radius),
        };
    }

    fn exploded(&self, mut rng: &mut dyn RngCore) -> Astroid {
        let new_size = self.size - 1;
        let radius = (new_size * 5) as f64;
        let theta_range = Normal::new(0.0, PI / 2.0).unwrap();
        let d_theta = rng.sample(theta_range);
        let theta = self.obj.theta + d_theta;
        return Astroid {
            obj: GameObject::new(
                self.obj.x + random(-5.0, 5.0, &mut rng),
                self.obj.y + random(-5.0, 5.0, &mut rng),
                random(40.0, 60.0, &mut rng),
                theta,
            ),
            size: new_size,
            border: Astroid::create_border(&mut rng, radius),
        };
    }

    pub fn explode(&self, rng: &mut dyn RngCore) -> Vec<Astroid> {
        if self.size <= 1 {
            vec![]
        } else {
            vec![self.exploded(rng), self.exploded(rng)]
        }
    }

    pub fn draw(&self, color: [f32; 4], ds: &DrawState, t: [[f64; 3]; 2], gl: &mut GlGraphics) {
        let line_info = Line::new(color, 0.5);
        for line_points in self.border.iter() {
            line_info.draw(*line_points, ds, t.trans(self.obj.x, self.obj.y), gl);
        }
    }

    pub fn go(&mut self, dt: f64, x_max: f64, y_max: f64) {
        self.obj = self.obj.with_go(dt, x_max, y_max);
    }

    pub fn create_border(mut rng: &mut dyn RngCore, radius: f64) -> Vec<[f64; 4]> {
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
        return self
            .border
            .iter()
            .map(|edge| {
                [
                    edge[0] + self.obj.x,
                    edge[1] + self.obj.y,
                    edge[2] + self.obj.x,
                    edge[3] + self.obj.y,
                ]
            })
            .collect();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use expectest::prelude::*;
    use std::f64::consts::PI;

    #[test]
    fn test_wrapped_add() {
        assert_eq!(wrapped_add(1.0, 1.0, 10.0), 2.0);
        assert_eq!(wrapped_add(1.0, 10.0, 10.0), 1.0);
        assert_eq!(wrapped_add(1.0, -5.0, 10.0), 6.0);
    }

    fn expect_both_close_to((x, y): (f64, f64), (x_expected, y_expected): (f64, f64)) {
        expect!(x).to(be_close_to(x_expected));
        expect!(y).to(be_close_to(y_expected));
    }

    #[test]
    fn test_to_cartesian() {
        expect_both_close_to(to_cartesian(0.0, 1.0), (0.0, -1.0));
        expect_both_close_to(to_cartesian(PI / 2.0, 1.0), (1.0, 0.0));
    }

    #[test]
    fn test_to_polar() {
        expect_both_close_to(to_polar(0.0, -1.0), (0.0, 1.0));
        expect_both_close_to(to_polar(1.0, 0.0), (PI / 2.0, 1.0));
    }

    #[test]
    fn test_game_object_factories() {
        expect!(GameObject::new(2.0, 3.0, 5.0, 7.0)).to(be_equal_to(GameObject {
            x: 2.0,
            y: 3.0,
            v: 5.0,
            theta: 7.0,
        }));
    }

    #[test]
    fn test_game_object_go() {
        let obj = GameObject::new(2.0, 3.0, 1.0, 0.0);
        expect!(obj.with_go(1.0, 200.0, 200.0))
            .to(be_equal_to(GameObject::new(2.0, 2.0, 1.0, 0.0)));
        expect!(obj.with_go(200.0, 200.0, 200.0))
            .to(be_equal_to(GameObject::new(2.0, 3.0, 1.0, 0.0)));
    }
}
