extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;

use piston::event_loop::Events;
use piston::input::{Event, Input, Button, Key};
use piston::window::WindowSettings;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use graphics::polygon::Polygon;
use graphics::{DrawState, Transformed};

struct Spaceship {
    x: f64,
    y: f64,
    dx: f64,
    dy: f64,
    theta: f64,
}

const SPACESHIP_POINTS: [[f64; 2]; 3] = [
    [5.0, 7.0],
    [-5.0, 7.0],
    [0.0, -13.0],
];

impl Spaceship {
    fn new() -> Spaceship {
        return Spaceship{
            x: 100.0,
            y: 100.0,
            dx: 0.0,
            dy: 0.0,
            theta: 0.0,
        };
    }

    fn accelerate(&mut self, ddv: f64) {
        self.dx += self.theta.sin()*ddv;
        self.dy -= self.theta.cos()*ddv;
    }
}

const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

fn main() {
    let opengl = OpenGL::V3_2;

    let window = Window::new(
        WindowSettings::new(
            "vs-game",
            [200, 200],
        )
        .exit_on_esc(true)
    ).unwrap();

    let mut gl = GlGraphics::new(opengl);
    let ds = DrawState::new();

    let poly = Polygon::new(WHITE);
    let mut spaceship = Spaceship::new();

    for e in window.events() {
        use graphics::clear;

        match e {
            Event::Update(u) => {
                spaceship.x = (spaceship.x + spaceship.dx*u.dt + 200.0) % 200.0;
                spaceship.y = (spaceship.y + spaceship.dy*u.dt + 200.0) % 200.0;
            },
            Event::Render(r) => gl.draw(r.viewport(), |c, gl| {
                clear(BLACK, gl);
                poly.draw(
                    &SPACESHIP_POINTS,
                    &ds,
                    c.transform
                        .trans(spaceship.x, spaceship.y)
                        .rot_rad(spaceship.theta),
                    gl,
                )
            }),
            Event::Input(Input::Press(Button::Keyboard(k))) =>
                match k {
                    Key::Up => spaceship.accelerate(2.0),
                    Key::Down => spaceship.accelerate(-2.0),
                    Key::Left => spaceship.theta -= 0.1,
                    Key::Right => spaceship.theta += 0.1,
                    Key::R => spaceship = Spaceship::new(),
                    Key::Q => return,
                    _ => (),
                },
            _ => (),
        }
    }
}
