extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;
extern crate rand;
extern crate rand_distr;

#[cfg(test)]
extern crate expectest;

use opengl_graphics::{GlGraphics, OpenGL};
use piston_window::WindowSettings;
use std::cell::RefCell;
use std::rc::Rc;

mod actors;
mod config;
mod intersect;
mod point;
mod scene;
use config::Config;
use scene::{MainScene, Scene};

fn main() {
    let opengl = OpenGL::V3_2;
    let config = Config::new();

    let dims = [config.width() as u32, config.height() as u32];
    let window_settings = WindowSettings::new("vs-game", dims).exit_on_esc(true);

    let window = Rc::new(RefCell::new(window_settings.build().unwrap()));

    let mut gl = GlGraphics::new(opengl);
    let mut rng = rand::thread_rng();

    let mut scene: Box<dyn Scene> = Box::new(MainScene::new(1, &config, &mut rng));
    while let Some(new_scene) = scene.events(&mut rng, window.clone(), &mut gl, &config) {
        scene = new_scene;
    }
}
