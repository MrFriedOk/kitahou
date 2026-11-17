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
        for b in 0..=self.count {
            let f: f32 = b as f32;
            let angle: f32 = f / self.count as f32 * std::f32::consts::TAU + f * self.spiral;

            let dir: Vec2 = Vec2::from_angle(angle) * self.distance;
            bullets.push(Bullet::new(pos, 5., dir, BulletKind::Straight, Vec2::ZERO))
        }
    }
}
