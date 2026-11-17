pub use crate::*;

pub struct Enemy {
    pub pos: Vec2,
    r: f32,
    vel: Vec2,
    player: Vec2,
}

impl Enemy {
    pub fn new(pos: Vec2, r: f32, vel: Vec2, player: Vec2) -> Self {
        Self {
            pos,
            r,
            vel,
            player,
        }
    }

    pub fn update(&mut self, dt: f32, player: Vec2) {
        self.pos += Pattern::new().flower();

        self.draw();
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.r, RED);
    }
}
