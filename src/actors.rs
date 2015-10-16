use graphics::{DrawState, Transformed, rectangle};
use graphics::polygon::Polygon;
use opengl_graphics::GlGraphics;
use piston::input::Key;

pub struct Spaceship {
    x: f64,
    y: f64,
    dx: f64,
    dy: f64,
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
            dx: 0.0,
            dy: 0.0,
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
        self.x = (self.x + self.dx*dt + x_max) % x_max;
        self.y = (self.y + self.dy*dt + y_max) % y_max;
    }

    pub fn accelerate(&mut self) {
        let net_accel = self.accel - self.reverse;
        self.dx += self.theta.sin()*net_accel;
        self.dy -= self.theta.cos()*net_accel;
    }

    pub fn turn(&mut self) {
        self.theta += self.right - self.left;
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
