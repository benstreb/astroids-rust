extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate rand;

use piston::window::WindowSettings;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use rand::thread_rng;
use std::cell::RefCell;
use std::rc::Rc;

mod actors;
mod intersect;
mod scene;
use scene::{MainScene, Scene};

fn main() {
    let opengl = OpenGL::V3_2;
    const WIDTH: f64 = 200.0;
    const HEIGHT: f64 = 200.0;

    let window = Rc::new(RefCell::new(Window::new(
        WindowSettings::new(
            "vs-game",
            [WIDTH as u32, HEIGHT as u32],
        )
        .exit_on_esc(true)
    ).unwrap()));

    let mut gl = GlGraphics::new(opengl);
    let mut rng = rand::thread_rng();

    let mut scene: Box<Scene> = Box::new(MainScene::new(&mut rng));
    while let Some(new_scene) = scene.events(&mut rng, window.clone(), &mut gl, (WIDTH, HEIGHT)) {
        scene = new_scene;
    }
}
