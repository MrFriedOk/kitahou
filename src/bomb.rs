use crate::*;

pub struct Bomb {
    pos: Vec2,
    pub r: f32,
}

impl Bomb {
    pub fn new(pos: Vec2) -> Self {
        Self { pos, r: 0. }
    }

    pub fn update(&mut self, tick: f32) {
        self.r += 1. * tick;
    }
}
