pub use crate::*;

pub struct Enemy {
    pos: Vec2,
    r: f32,
    vel: Vec2,
}

impl Enemy {
    pub fn new(pos: Vec2, r: f32, vel: Vec2) -> Self {
        Self { pos, r, vel }
    }
}
