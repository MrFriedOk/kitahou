pub use crate::*;

pub struct Enemy {
    pub pos: Vec2,
    r: f32,
    vel: Vec2,
    player: Vec2,
    cooldown: f32,
}

impl Enemy {
    pub fn new(pos: Vec2, r: f32, vel: Vec2, player: Vec2, cooldown: f32) -> Self {
        Self {
            pos,
            r,
            vel,
            player,
            cooldown,
        }
    }

    pub fn update(&mut self, dt: f32, player: Vec2, bullets: &mut Vec<Bullet>) {
        //self.pos += Pattern::new(12., 5., 1., 1., 5.).flower();

        if self.cooldown <= 0. || self.cooldown == 1. {
            Pattern::new(12, 10., 100., 5.).circle(
                self.pos,
                bullets,
                dt,
                if self.cooldown >= 1. { 0. } else { 0.5 },
            );

            self.cooldown += 2.
        } else {
            self.cooldown -= 0.01;
        }
        println!("{}", self.cooldown);
        self.draw();
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.r, RED);
    }
}
