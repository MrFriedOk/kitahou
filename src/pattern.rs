use crate::bullet::BulletKind;
pub use crate::*;

pub struct Pattern {
    count: i32,
}

impl Pattern {
    pub fn new(count: i32) -> Self {
        Self { count }
    }

    pub fn circle(&self, enemy_pos: Vec2, bullets: &mut Vec<Bullet>, rotation: bool) {
        //note
        //for self, rotaton is how points "rotate" on a circle. this can be used to create a
        //'checkerboard'?
        for c in 0..self.count {
            let i: f32 = c as f32;
            let rotate: f32 = if rotation {
                self.count as f32 / std::f32::consts::PI
            } else {
                self.count as f32 / std::f32::consts::TAU
            };
            let angle: f32 = rotate + i * std::f32::consts::TAU / self.count as f32;

            bullets.push(Bullet::new(
                enemy_pos,
                10.,
                Vec2::new(angle.cos(), angle.sin()),
                BulletKind::Straight,
                Vec2::ZERO,
                RED,
            ))
        }
    }

    pub fn flower(&self, pos: Vec2, bullets: &mut Vec<Bullet>) {
        todo!()
    }

    pub fn fan(&self, enemy_pos: Vec2, bullets: &mut Vec<Bullet>, player_pos: Vec2) {
        //i already did all the math in desmos. rust has an arctan function. what is the 2nd point
        //tho? it's the player, because a fan should aim at the player. maybe home, idk yet.
        //player's pos muse be passed in with this function.
        for c in 0..self.count {
            let i: f32 = c as f32;

            let rotation: f32 = f32::atan(
                (enemy_pos.x.powi(2) - player_pos.x.powi(2))
                    / (enemy_pos.y.powi(2) - player_pos.y.powi(2)),
            );

            let angle: f32 = rotation + i * (std::f32::consts::PI / self.count as f32);
            bullets.push(Bullet::new(
                enemy_pos,
                15.,
                Vec2::new(angle.cos(), angle.sin()),
                BulletKind::Homing,
                player_pos,
                RED,
            ));
        }
    }
}
