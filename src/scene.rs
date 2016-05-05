use piston_window::PistonWindow as Window;
use graphics::{DrawState, Transformed};
use graphics::text::Text;
use opengl_graphics::GlGraphics;
use opengl_graphics::glyph_cache::GlyphCache;
use piston::input::{Button, Event, Input, Key, RenderArgs, UpdateArgs};
use rand::Rng;
use std::cell::RefCell;
use std::borrow::BorrowMut;
use std::path::Path;
use std::iter::repeat;
use std::rc::Rc;

use actors::{Astroid, Bullet, Spaceship};
use config::Config;

pub trait Scene {
    fn events(&mut self,
              &mut Rng,
              Rc<RefCell<Window>>,
              &mut GlGraphics,
              &Config)
              -> Option<Box<Scene>>;
}

#[derive(Clone)]
pub struct MainScene {
    difficulty: usize,
    spaceship: Spaceship,
    bullets: Vec<Bullet>,
    astroids: Vec<Astroid>,
}

impl MainScene {
    pub fn new(difficulty: usize, config: &Config, rng: &mut Rng) -> MainScene {
        return MainScene {
            difficulty: difficulty,
            spaceship: Spaceship::new(config),
            bullets: Vec::new(),
            astroids: repeat(0)
                          .take(difficulty)
                          .map(|_| Astroid::large_new(config, rng))
                          .collect(),
        };
    }

    fn draw(&self, r: RenderArgs, ds: DrawState, gl: &mut GlGraphics) {
        use graphics::clear;
        gl.draw(r.viewport(), |c, gl| {
            clear(BLACK, gl);
            for astroid in self.astroids.iter() {
                astroid.draw(WHITE, &ds, c.transform, gl);
            }
            self.spaceship.draw(WHITE, &ds, c.transform, gl);
            for bullet in self.bullets.iter() {
                bullet.draw(WHITE, c.transform, gl);
            }
        })
    }

    fn update(&mut self, u: UpdateArgs, rng: &mut Rng, config: &Config) -> Option<Box<Scene>> {
        self.spaceship.accelerate(u.dt);
        self.spaceship.turn(u.dt);
        self.spaceship.go(u.dt, config.width(), config.height());
        self.spaceship.cooldown(u.dt);
        for ref mut astroid in self.astroids.iter_mut() {
            astroid.go(u.dt, config.width(), config.height());
        }
        {
            let astroid_edges = self.astroids
                                    .iter()
                                    .flat_map(|astroid| astroid.edges());
            if self.spaceship.collides(astroid_edges) {
                return Some(Box::new(GameOverScene::new(self)));
            }
        }
        if self.spaceship.is_firing() && self.spaceship.ready_to_fire() {
            self.spaceship.fire(&mut self.bullets);
        }
        let mut new_bullets = Vec::with_capacity(self.bullets.len());
        for mut bullet in self.bullets.iter_mut() {
            bullet.go(u.dt, config.width(), config.height());
            let mut collided = false;
            self.astroids = self.astroids
                                .iter()
                                .flat_map(|a| {
                                    if bullet.collides(a) {
                                        collided = true;
                                        a.explode(rng)
                                    } else {
                                        vec![a.clone()]
                                    }
                                })
                                .collect();
            if !collided {
                new_bullets.push(bullet.clone());
            }
        }
        new_bullets.retain(Bullet::is_alive);
        self.bullets = new_bullets;
        if self.astroids.len() == 0 {
            return Some(Box::new(MainScene::new(self.difficulty + 1, config, rng)));
        }
        return None;
    }
}

const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

impl Scene for MainScene {
    fn events(&mut self,
              mut rng: &mut Rng,
              window: Rc<RefCell<Window>>,
              gl: &mut GlGraphics,
              config: &Config)
              -> Option<Box<Scene>> {

        let ds = DrawState::default();
        while let Some(e) = (*window).borrow_mut().next() {
            match e {
                Event::Update(u) => {
                    let scene_change = self.update(u, rng, config);
                    if scene_change.is_some() {
                        return scene_change;
                    }
                }
                Event::Render(r) => self.draw(r, ds, gl),
                Event::Input(Input::Press(Button::Keyboard(k))) => {
                    self.spaceship.handle_press(k);
                    match k {
                        Key::R => return Some(Box::new(MainScene::new(1, config, &mut rng))),
                        Key::Q => return None,
                        _ => (),
                    }
                }
                Event::Input(Input::Release(Button::Keyboard(k))) => {
                    self.spaceship.handle_release(k);
                }
                _ => (),
            }
        }
        return None;
    }
}


struct GameOverScene {
    end_game: MainScene,
}

impl GameOverScene {
    fn new(end_game: &MainScene) -> GameOverScene {
        GameOverScene { end_game: end_game.clone() }
    }
}

impl Scene for GameOverScene {
    fn events(&mut self,
              rng: &mut Rng,
              window: Rc<RefCell<Window>>,
              gl: &mut GlGraphics,
              config: &Config)
              -> Option<Box<Scene>> {
        let ds = DrawState::default();
        let game_over_text = Text::new_color(WHITE, 20);
        let font_path = Path::new("/usr/share/fonts/TTF/DejaVuSans.ttf");
        let mut character_cache: Box<GlyphCache> = Box::new(GlyphCache::new(font_path).unwrap());
        while let Some(e) = (*window).borrow_mut().next() {
            match e {
                Event::Render(r) => {
                    self.end_game.draw(r, ds, gl);
                    gl.draw(r.viewport(), |c, gl| {
                        game_over_text.draw("Game Over",
                                            character_cache.borrow_mut() as &mut GlyphCache,
                                            &ds,
                                            c.transform
                                             .trans(config.width() / 2.0, config.height() / 2.0),
                                            gl);
                    });
                }
                Event::Input(Input::Press(k)) => {
                    match k {
                        Button::Keyboard(Key::Space) | Button::Keyboard(Key::R) => {
                            return Some(Box::new(MainScene::new(1, config, rng)))
                        }
                        Button::Keyboard(Key::Q) => return None,
                        _ => (),
                    }
                }
                _ => (),
            }
        }
        return None;
    }
}
