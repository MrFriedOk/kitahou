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

    pub fn update(&mut self, tick: f32, p_bullets: &mut Vec<Bullet>) {
        let dir: Vec2 = self.read_dir(p_bullets, tick);

        self.pos += dir * tick;

        self.pos = self.pos.clamp(Vec2::ZERO, vec2(SW, SH));

        if self.cooldown != 0. {
            self.cooldown -= 1. * tick;
        }
        println!("{}", self.cooldown);

        self.draw();
    }

    pub fn damage(&mut self) {
        self.hp -= 1;
        self.i_frame = true;
    }

    pub fn read_dir(&mut self, p_bullets: &mut Vec<Bullet>, tick: f32) -> Vec2 {
        let mut dir: Vec2 = Vec2::new(0., 0.);
        let mut speed: f32 = 300.;

        if is_key_down(KeyCode::LeftShift) {
            speed = 100.0;
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
                self.pos,
                self.r,
                Vec2::new(0., -10.),
                bullet::BulletKind::Straight,
                Vec2::ZERO,
            ));
            self.cooldown += 1. //* tick
        }

        dir
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.r, self.col);
    }
}
