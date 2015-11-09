extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate rand;

use piston::window::WindowSettings;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use rand::thread_rng;

mod actors;
mod intersect;
mod scene;
use scene::{MainScene, Scene};

fn main() {
    let opengl = OpenGL::V3_2;
    const WIDTH: f64 = 200.0;
    const HEIGHT: f64 = 200.0;

    let window = Box::new(Window::new(
        WindowSettings::new(
            "vs-game",
            [WIDTH as u32, HEIGHT as u32],
        )
        .exit_on_esc(true)
    ).unwrap());

    let mut gl = GlGraphics::new(opengl);
    let mut rng = rand::thread_rng();

    let mut main_game = MainScene::new(&mut rng);
    main_game.events(window, &mut gl, (WIDTH, HEIGHT));
}
