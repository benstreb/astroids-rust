use glutin_window::GlutinWindow;
use graphics::DrawState;
use opengl_graphics::GlGraphics;
use piston::event_loop::Events;
use piston::input::{Button, Event, Input, Key};
use rand::Rng;
use std::iter::repeat;

use actors::{Astroid, Bullet, Spaceship};

pub trait Scene {
    fn events(&mut self, Box<GlutinWindow>, &mut GlGraphics, (f64, f64)) -> Option<Box<Scene>>;
}

pub struct MainScene {
    spaceship: Spaceship,
    bullets: Vec<Bullet>,
    astroids: Vec<Astroid>,
}

impl MainScene {
    pub fn new<R: Rng>(rng: &mut R) -> MainScene {
        return MainScene{
            spaceship: Spaceship::new(),
            bullets: Vec::new(),
            astroids: repeat(0).take(5).map(|_| Astroid::new(rng))
                .collect(),
        };
    }
}

const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

impl Scene for MainScene {
    fn events(&mut self, window: Box<GlutinWindow>, gl: &mut GlGraphics, (width, height): (f64, f64)) -> Option<Box<Scene>> {
        use graphics::clear;

        let ds = DrawState::new();
        for e in window.events() {
            match e {
                Event::Update(u) => {
                    self.spaceship.accelerate(u.dt);
                    self.spaceship.turn(u.dt);
                    self.spaceship.go(u.dt, width, height);
                    self.spaceship.cooldown(u.dt);
                    for ref mut astroid in self.astroids.iter_mut() {
                        astroid.go(u.dt, width, height);
                    }
                    let astroid_edges = self.astroids.iter()
                        .flat_map(|astroid| astroid.edges());
                    if self.spaceship.collides(astroid_edges) {
                        print!("Collided with astroid\n");
                    }
                    if self.spaceship.is_firing() && self.spaceship.ready_to_fire() {
                        self.spaceship.fire(&mut self.bullets);
                    }
                    for ref mut bullet in self.bullets.iter_mut() {
                        bullet.go(u.dt, width, height);
                    }
                    self.bullets.retain(|b| b.is_alive());
                },
                Event::Render(r) => gl.draw(r.viewport(), |c, gl| {
                    clear(BLACK, gl);
                    for astroid in self.astroids.iter() {
                        astroid.draw(WHITE, &ds, c.transform, gl);
                    }
                    self.spaceship.draw(WHITE, &ds, c.transform, gl);
                    for bullet in self.bullets.iter() {
                        bullet.draw(WHITE, c.transform, gl);
                    }
                }),
                Event::Input(Input::Press(Button::Keyboard(k))) => {
                    self.spaceship.handle_press(k);
                    match k {
                        Key::R => self.spaceship = Spaceship::new(),
                        Key::Q => return None,
                        _ => (),
                    }
                },
                Event::Input(Input::Release(Button::Keyboard(k))) => {
                    self.spaceship.handle_release(k);
                },
                _ => (),
            }
        }
        return None;
    }
}
