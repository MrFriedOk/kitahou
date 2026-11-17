pub use crate::*;

pub enum BulletKind {
    Straight,
    Accelerating { accel: f32 },
    Homing { rate: f32 },
}

pub struct Bullet {
    pos: Vec2,
    r: f32,
    vel: Vec2,
    kind: BulletKind,
}

impl Bullet {
    pub fn new(pos: Vec2, r: f32, vel: Vec2, kind: BulletKind) -> Self {
        Self { pos, r, vel, kind }
    }

    pub fn move_bullet(&mut self, target: Vec2) {}

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, 5., RED)
    }

    pub fn update(&mut self, dt: f32) {
        self.draw();
    }
}
