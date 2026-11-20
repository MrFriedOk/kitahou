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
    pub avel: Vec2,
    col: Color,
    //angular velocity, required to make things spinny
}

impl Bullet {
    pub fn new(pos: Vec2, r: f32, vel: Vec2, kind: BulletKind, target: Vec2, col: Color) -> Self {
        Self {
            pos,
            r,
            vel,
            kind,
            target,
            avel: Vec2::ZERO,
            col,
        }
    }

    pub fn move_bullet(&mut self, target: Vec2) {
        self.pos += self.vel * 60.;
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.r, self.col)
    }

    pub fn update(&mut self) {
        self.move_bullet(self.target);

        self.draw();
    }

    pub fn out_of_bounds(&self) -> bool {
        let p: f32 = 10.;
        self.pos.x > SW + p || self.pos.x < 0. - p || self.pos.y > SH + p || self.pos.y < 0. - p
    }

    pub fn bombed(&self, bomb: &Option<Bomb>) -> bool {
        match &bomb {
            Some(bomb) => {
                let bomb_hitbox: Circle = Circle::new(bomb.pos.x, bomb.pos.y, bomb.r);
                let bullet_hitbox: Circle = Circle::new(self.pos.x, self.pos.y, self.r);

                bullet_hitbox.overlaps(&bomb_hitbox)
            }
            None => false,
        }
    }
}
