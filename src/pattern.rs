pub use crate::*;

pub struct Pattern {
    pub count: f32,
    r: f32,
    pub spread: f32,
    pub distance: f32,
    pub spiral: f32,
}

impl Pattern {
    //pub fn new() -> Self {}

    pub fn flower(&mut self) -> Vec2 {
        self.count = 12.;
        self.spread = 1.;
        self.distance = 1.;
        self.spiral = 5.;

        dir: Vec2::new()
    }
}
