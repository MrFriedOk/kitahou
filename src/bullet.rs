pub use crate::*;

pub enum BulletKind {
    Straight,
    Accelerating { accel: f32 },
    Homing { rate: f32 },
}

pub struct Bullet {
    pub pos: Vec2,
    pub r: f32,
    pub vel: Vec2,
    kind: BulletKind,
    target: Vec2,
    pub avel: Vec2, //angular velocity, required to make things spinny
}

impl Bullet {
    pub fn new(pos: Vec2, r: f32, vel: Vec2, kind: BulletKind, target: Vec2) -> Self {
        Self {
            pos,
            r,
            vel,
            kind,
            target,
            avel: Vec2::ZERO,
        }
    }

    pub fn move_bullet(&mut self, target: Vec2) {
        self.pos += self.vel;
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.r, RED)
    }

    pub fn update(&mut self) {
        self.move_bullet(self.target);

        self.draw();
    }

    pub fn out_of_bounds(&self) -> bool {
        let p: f32 = 10.;
        self.pos.x > SW + p || self.pos.x < 0. - p || self.pos.y > SH + p || self.pos.y < 0. - p
    }
}
