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
}

const SPACESHIP_POINTS: [[f64; 2]; 3] = [
    [50.0, 0.0],
    [0.0, 50.0],
    [50.0, 50.0],
];

impl Spaceship {
    fn new() -> Spaceship {
        return Spaceship{x: 0.0, y: 0.0};
    }
}

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

    let poly = Polygon::new([0.0, 0.0, 0.0, 1.0]);
    let mut spaceship = Spaceship::new();

    for e in window.events() {
        use graphics::clear;
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        match e {
            Event::Render(r) => gl.draw(r.viewport(), |c, gl| {
                clear(WHITE, gl);
                poly.draw(&SPACESHIP_POINTS, &ds, c.transform.trans(spaceship.x, spaceship.y), gl)
            }),
            Event::Input(Input::Press(Button::Keyboard(k))) => 
                match k {
                    Key::Up => spaceship.y -= 1.0,
                    Key::Down => spaceship.y += 1.0,
                    Key::Left => spaceship.x -= 1.0,
                    Key::Right => spaceship.x += 1.0,
                    Key::Q => return,
                    _ => (),
                },
            _ => (),
        }
    }
}
