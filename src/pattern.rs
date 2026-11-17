use crate::bullet::BulletKind;
pub use crate::*;

pub struct Pattern {
    pub count: i32,
    pub spread: f32,
    pub distance: f32,
    pub spiral: f32,
}

impl Pattern {
    pub fn new(count: i32, spread: f32, distance: f32, spiral: f32) -> Self {
        Self {
            count,
            spread,
            distance,
            spiral,
        }
    }

    pub fn flower(&mut self, pos: Vec2) -> Bullet {
        for b in 0..=self.count {
            let f: f32 = b as f32;
            let dir: Vec2 = Vec2::new(
                (f * self.distance) * self.spread.cos() * f * self.spiral,
                (f * self.distance) * self.spread.sin() * f * self.spiral,
            );

            Bullet::new(pos, 5., dir, BulletKind::Straight, Vec2::ZERO)
        }
    }
}
