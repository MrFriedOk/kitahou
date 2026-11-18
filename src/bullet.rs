pub use crate::*;

#[derive(Clone)]
pub enum BulletKind {
    Straight,
    Accelerating { accel: f32 },
    Homing { rate: f32 },
}

#[derive(Clone)]
pub struct Bullet {
    pub pos: Vec2,
    pub r: f32,
    pub vel: Vec2,
    kind: BulletKind,
    target: Vec2,
}

impl Bullet {
    pub fn new(pos: Vec2, r: f32, vel: Vec2, kind: BulletKind, target: Vec2) -> Self {
        Self {
            pos,
            r,
            vel,
            kind,
            target,
        }
    }

    pub fn move_bullet(&mut self, dt: f32, target: Vec2) {
        self.pos += self.vel * dt;
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.r, RED)
    }

    pub fn update(&mut self, dt: f32) {
        self.move_bullet(dt, self.target);

        self.draw();
    }
}
