use crate::bullet::BulletKind;
pub use crate::*;

pub struct Pattern {
    pub count: i32,
    pub spread: f32,
    pub distance: f32,
    pub spiral: f32,
    pub cooldown: f32,
}

impl Pattern {
    pub fn new(count: i32, spread: f32, distance: f32, spiral: f32, cooldown: f32) -> Self {
        Self {
            count,
            spread,
            distance,
            spiral,
            cooldown,
        }
    }

    pub fn circle(&mut self, enemy_pos: Vec2, bullets: &mut Vec<Bullet>, dt: f32) {
        for c in 0..self.count {
            let i: f32 = c as f32;
        }
    }

    pub fn flower(&mut self, pos: Vec2, bullets: &mut Vec<Bullet>, dt: f32) {
        todo!()
    }
}
