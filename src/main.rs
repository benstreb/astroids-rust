extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate rand;

use piston::event_loop::Events;
use piston::input::{Event, Input, Button, Key};
use piston::window::WindowSettings;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use graphics::DrawState;
use rand::thread_rng;
use std::iter::repeat;

mod actors;
use actors::{Spaceship, Astroid};

const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

fn main() {
    let opengl = OpenGL::V3_2;
    const WIDTH: f64 = 200.0;
    const HEIGHT: f64 = 200.0;

    let window = Window::new(
        WindowSettings::new(
            "vs-game",
            [WIDTH as u32, HEIGHT as u32],
        )
        .exit_on_esc(true)
    ).unwrap();

    let mut gl = GlGraphics::new(opengl);
    let ds = DrawState::new();
    let mut rng = rand::thread_rng();

    let mut spaceship = Spaceship::new();
    let mut bullets = Vec::new();
    let mut astroids: Vec<Astroid> = repeat(0)
        .take(5)
        .map(|_| Astroid::new(&mut rng))
        .collect();

    for e in window.events() {
        use graphics::clear;

        match e {
            Event::Update(u) => {
                for ref mut astroid in astroids.iter_mut() {
                    astroid.go(u.dt, WIDTH, HEIGHT);
                }
                spaceship.accelerate(u.dt);
                spaceship.turn(u.dt);
                spaceship.go(u.dt, WIDTH, HEIGHT);
                spaceship.cooldown(u.dt);
                if spaceship.is_firing() && spaceship.ready_to_fire() {
                    spaceship.fire(&mut bullets);
                }
                for ref mut bullet in bullets.iter_mut() {
                    bullet.go(u.dt, WIDTH, HEIGHT);
                }
                bullets.retain(|b| b.is_alive());
            },
            Event::Render(r) => gl.draw(r.viewport(), |c, gl| {
                clear(BLACK, gl);
                for astroid in astroids.iter() {
                    astroid.draw(WHITE, c.transform, gl);
                }
                spaceship.draw(WHITE, &ds, c.transform, gl);
                for bullet in bullets.iter() {
                    bullet.draw(WHITE, c.transform, gl);
                }
            }),
            Event::Input(Input::Press(Button::Keyboard(k))) => {
                spaceship.handle_press(k);
                match k {
                    Key::R => spaceship = Spaceship::new(),
                    Key::Q => return,
                    _ => (),
                }
            },
            Event::Input(Input::Release(Button::Keyboard(k))) => {
                spaceship.handle_release(k);
            },
            _ => (),
        }
    }
}
