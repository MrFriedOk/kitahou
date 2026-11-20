//pub use macroquad::prelude::*;

//pub use crate::config::*;

pub use crate::*;

pub struct Player {
    //r = radius
    pub hp: i8,
    pub pos: Vec2,
    pub r: f32,
    //slow: bool,
    pub i_frame: bool,
    pub col: Color,
    cooldown: f32,
    //pub bomb: Bomb,
}

impl Player {
    pub fn new(hp: i8, pos: Vec2, r: f32, i_frame: bool, col: Color) -> Self {
        Self {
            hp,
            pos,
            r,
            i_frame,
            col,
            cooldown: 0.,
            //bomb: Bomb::new(pos),
        }
    }

    pub fn update(&mut self, p_bullets: &mut Vec<Bullet>) {
        let dir: Vec2 = self.read_dir(p_bullets);

        self.pos += dir;

        self.pos = self.pos.clamp(Vec2::ZERO, vec2(SW, SH));

        self.cooldown = self.cooldown.clamp(0., 1.);
        self.cooldown -= 1.;

        self.draw();
    }

    pub fn damage(&mut self) {
        self.hp -= 1;
        self.i_frame = true;
    }

    pub fn read_dir(&mut self, p_bullets: &mut Vec<Bullet>) -> Vec2 {
        let mut dir: Vec2 = Vec2::new(0., 0.);
        let mut speed: f32 = 11.;

        if is_key_down(KeyCode::LeftShift) {
            speed = 6.0;
            self.r = 8.;
        } else {
            self.r = 10.;
        }

        if is_key_down(KeyCode::Up) {
            dir.y -= speed
        }
        if is_key_down(KeyCode::Down) {
            dir.y += speed
        }
        if is_key_down(KeyCode::Left) {
            dir.x -= speed
        }
        if is_key_down(KeyCode::Right) {
            dir.x += speed
        }

        if is_key_down(KeyCode::Z) && self.cooldown <= 0. {
            p_bullets.push(Bullet::new(
                Vec2::new(self.pos.x + 10., self.pos.y),
                7.,
                Vec2::new(0., -7. * TICK),
                bullet::BulletKind::Straight,
                Vec2::ZERO,
                GREEN,
            ));
            p_bullets.push(Bullet::new(
                Vec2::new(self.pos.x - 10., self.pos.y),
                7.,
                Vec2::new(0., -7. * TICK),
                bullet::BulletKind::Straight,
                Vec2::ZERO,
                GREEN,
            ));
            self.cooldown += 1. //* tick
        }

        dir
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.r, self.col);
    }
}
