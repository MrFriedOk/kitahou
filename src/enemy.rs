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

    pub fn update(&mut self, dt: f32, player: Vec2, bullets: &mut Vec<Bullet>) {
        //self.pos += Pattern::new(12., 5., 1., 1., 5.).flower();

        Pattern::new(12, 10., 100., 5., 0.).circle(self.pos, bullets, dt);

        self.draw();
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.r, RED);
    }
}
