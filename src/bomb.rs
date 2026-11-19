use crate::*;

pub struct Bomb {
    pos: f32,
    pub r: f32,
}

impl Bomb {
    pub fn new(pos: Vec2) -> Self {
        Self { pos }
    }

    pub fn update(&mut self, tick: f32) {}
}
