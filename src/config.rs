use std::path::Path;

pub struct Config;

impl Config {
    pub fn new() -> Config {
        return Config;
    }

    pub fn width(&self) -> f64 {
        return 200.0;
    }

    pub fn height(&self) -> f64 {
        return 200.0;
    }

    pub fn astroid_gap_distance(&self) -> f64 {
        return 25.0;
    }

    pub fn font_path(&self) -> &Path {
        return Path::new("res/Carlito-Regular.ttf");
    }

    pub fn font_offset(&self) -> (f64, f64) {
        return (-45.0, 0.0);
    }
}
