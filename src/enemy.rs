pub use crate::*;

pub struct Enemy {
    pos: Vec2,
    r: f32,
    vel: Vec2,
}

impl Enemy {
    pub fn new(pos: Vec2, r: f32, vel: Vec2) -> Self {
        Self { pos, r, vel }
    }

    pub fn update(&mut self, dt: f32) {
        self.draw();
    }

    pub fn danmaku_ring(&self, bullets: &mut Vec<Bullet>, count: f32, speed: f32) {
        for ring in 0..count {
            let angle = ring as f32 / count as f32 * std::f32::consts::TAU;
        }
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.r, RED);
    }
}
