use graphics::{DrawState, Transformed, rectangle};
use graphics::polygon::Polygon;
use graphics::line::Line;
use opengl_graphics::GlGraphics;
use piston::input::Key;
use rand::Rng;
use std::f64::consts::PI;

fn to_cartesian(theta: f64, r: f64) -> (f64, f64) {
    return (
        theta.sin()*r,
        -theta.cos()*r,
    )
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

const LARGE_ASTROID_BORDER: [[f64; 4]; 4] = [
    [-15.0, 0.0, 0.0, 15.0],
    [0.0, 15.0, 15.0, 0.0],
    [15.0, 0.0, 0.0, -15.0],
    [0.0, -15.0, -15.0, 0.0],
];

const ASTROID_BORDER: [[f64; 4]; 4] = [
    [-10.0, 0.0, 0.0, 10.0],
    [0.0, 10.0, 10.0, 0.0],
    [10.0, 0.0, 0.0, -10.0],
    [0.0, -10.0, -10.0, 0.0],
];

const SMALL_ASTROID_BORDER: [[f64; 4]; 4] = [
    [-5.0, 0.0, 0.0, 5.0],
    [0.0, 5.0, 5.0, 0.0],
    [5.0, 0.0, 0.0, -5.0],
    [0.0, -5.0, -5.0, 0.0],
];

const ASTROID_BORDERS: [&'static[[f64; 4]; 4]; 3] = [&LARGE_ASTROID_BORDER, &ASTROID_BORDER, &SMALL_ASTROID_BORDER];

pub struct Astroid {
    x: f64,
    y: f64,
    v: f64,
    theta: f64,
    border: &'static[[f64; 4]; 4],
}

impl Astroid {
    pub fn new<R: Rng>(rng: &mut R) -> Astroid {
        use rand::sample;
        return Astroid {
            x: rng.gen_range(0.0, 100.0),
            y: rng.gen_range(0.0, 100.0),
            v: rng.gen_range(40.0, 60.0),
            theta: rng.gen_range(0.0, 2.0*PI),
            border: sample(rng, ASTROID_BORDERS.iter(), 1)[0],
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
}