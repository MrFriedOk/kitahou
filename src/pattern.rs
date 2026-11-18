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

    pub fn circle(&mut self, enemy_pos: Vec2, bullets: &mut Vec<Bullet>, dt: f32, rotation: f32) {
        //note
        //for self, rotaton is how points "rotate" on a circle. this can be used to create a
        //'checkerboard'?
        for c in 0..self.count {
            let i: f32 = c as f32;
            let angle: f32 = rotation + i * std::f32::consts::TAU / self.count as f32;

            bullets.push(Bullet::new(
                Vec2::new(enemy_pos.x + angle.cos(), enemy_pos.y + angle.sin()),
                5.,
                Vec2::new(angle.cos(), angle.sin()),
                BulletKind::Straight,
                Vec2::ZERO,
            ))
        }
    }

    pub fn flower(&mut self, pos: Vec2, bullets: &mut Vec<Bullet>, dt: f32) {
        todo!()
    }
}
