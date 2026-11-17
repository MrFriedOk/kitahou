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

    pub fn flower(&self, pos: Vec2, bullets: &mut Vec<Bullet>) {
        for b in 0..self.count {
            let f: f32 = b as f32;
            let angle: f32 = f * self.spiral;

            let speed: f32 = self.distance * (1. + (12. * angle).cos()) / 2.;

            let dir: Vec2 = Vec2::new(angle.cos(), angle.sin() * speed);
            bullets.push(Bullet::new(pos, 5., dir, BulletKind::Straight, Vec2::ZERO))
        }
    }
}
