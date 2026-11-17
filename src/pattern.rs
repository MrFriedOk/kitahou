pub use crate::*;

pub struct Pattern {
    pub count: f32,
    r: f32,
    pub spread: f32,
    pub distance: f32,
    pub spiral: f32,
}

impl Pattern {
    pub fn new(count: f32,r: f32,spread: f32,distance:f32,spiral:f32) -> Self {
        Self {
        count,
        r,
        spread,
        distance,
        spiral,
    }
    }

    pub fn flower(&mut self) -> Vec2 {
        

        dir: Vec2::new()
    }
}
