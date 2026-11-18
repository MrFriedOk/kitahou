pub use crate::*;

pub struct Enemy {
    pub pos: Vec2,
    r: f32,
    vel: Vec2,
    player: Vec2,
    cooldown: f32,
    rotate: bool,
}

impl Enemy {
    pub fn new(pos: Vec2, r: f32, vel: Vec2, player: Vec2, cooldown: f32, rotate: bool) -> Self {
        Self {
            pos,
            r,
            vel,
            player,
            cooldown,
            rotate,
        }
    }

    pub fn update(&mut self, dt: f32, player: Vec2, bullets: &mut Vec<Bullet>) {
        //self.pos += Pattern::new(12., 5., 1., 1., 5.).flower();

        //let rotate: bool = if self.cooldown <= 0.5 { true } else { false };

        if self.cooldown <= 0. {
            Pattern::new(10, 0., 0., 0.).circle(self.pos, bullets, dt, self.rotate);

            self.cooldown += 1.;
            self.rotate = !self.rotate;
        } else {
            self.cooldown -= 0.02;
        }
        println!("{}", self.cooldown);
        self.draw();
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.r, RED);
    }
}
