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
}