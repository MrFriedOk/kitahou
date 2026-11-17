use crate::bullet::BulletKind;
pub use crate::*;

pub struct Pattern {
    pub count: i32,
    pub spread: f32,
    pub distance: f32,
    pub spiral: f32,
    pub cooldown: f32,
    pub shots: i32,
}

impl Pattern {
    pub fn new(
        count: i32,
        spread: f32,
        distance: f32,
        spiral: f32,
        cooldown: f32,
        shots: i32,
    ) -> Self {
        Self {
            count,
            spread,
            distance,
            spiral,
            cooldown,
            shots,
        }
    }

    pub fn flower(&mut self, pos: Vec2, bullets: &mut Vec<Bullet>, dt: f32) {
        self.cooldown -= dt;

        let interval: f32 = 0.005;
        if self.cooldown <= 0. {
            //let f: f32 = b as f32;
            self.cooldown += interval;
            let angle: f32 = self.shots as f32 * self.spiral;

            let speed: f32 = self.distance * (1. + (1. * angle).cos()) / 2.;

            let dir: Vec2 = Vec2::new(angle.cos(), angle.sin() * speed);
            bullets.push(Bullet::new(pos, 5., dir, BulletKind::Straight, Vec2::ZERO));

            self.shots += 1;
        }
    }
}
