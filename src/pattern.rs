use crate::bullet::BulletKind;
pub use crate::*;

pub struct Pattern {
    pub count: f32,
    pub spread: f32,
    pub distance: f32,
    pub spiral: f32,
}

impl Pattern {
    pub fn new(count: f32, spread: f32, distance: f32, spiral: f32) -> Self {
        Self {
            count,
            spread,
            distance,
            spiral,
        }
    }

    pub fn flower(&mut self, pos: Vec2) -> Bullet {
        let vel: Vec2 = Vec2::new(10., 10.);

        Bullet::new(pos, 5., vel, BulletKind::Straight, Vec2::ZERO)
    }
}
