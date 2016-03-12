extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate rand;

#[cfg(test)]
#[macro_use(expect)]
extern crate expectest;

use piston::window::WindowSettings;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use rand::thread_rng;
use std::cell::RefCell;
use std::rc::Rc;

mod actors;
mod intersect;
mod point;
mod scene;
mod config;
use scene::{MainScene, Scene};
use config::Config;

fn main() {
    let opengl = OpenGL::V3_2;
    let config = Config::new();

    let dims = [config.width() as u32, config.height() as u32];
    let window_settings = WindowSettings::new("vs-game", dims).exit_on_esc(true);

    let window = Rc::new(RefCell::new(Window::new(window_settings).unwrap()));

    let mut gl = GlGraphics::new(opengl);
    let mut rng = rand::thread_rng();

    let mut scene: Box<Scene> = Box::new(MainScene::new(1, &config, &mut rng));
    while let Some(new_scene) = scene.events(&mut rng, window.clone(), &mut gl, &config) {
        scene = new_scene;
    }
}
