//pub use macroquad::prelude::*;

//pub use crate::config::*;

pub use crate::*;

pub struct Player {
    //r = radius
    pub hp: i8,
    pub pos: Vec2,
    pub r: f32,
    //slow: bool,
    pub i_frame: f32,
    pub col: Color,
    cooldown: f32,
    pub level: i8,
    pub points: i16,
    //pub bomb: Bomb,
}

impl Player {
    pub fn new(pos: Vec2) -> Self {
        Self {
            hp: 3,
            pos,
            r: 10.,
            i_frame: 0.,
            col: WHITE,
            cooldown: 0.,
            level: 1,
            points: 0,
            //bomb: Bomb::new(pos),
        }
    }

    pub fn update(&mut self, p_bullets: &mut Vec<Bullet>) {
        let dir: Vec2 = self.read_dir(p_bullets);

        self.pos += dir;

        //i_frame check, make sure it's clamped and decrease it every frame. also check it to assign
        //color (temporary). next add a thing that ALWAYS brings the player back to the center Vec2(HC, STARTY)
        self.i_frame -= 0.01;
        self.i_frame = self.i_frame.clamp(0., 1.);
        if self.i_frame != 0. {
            self.col = RED;
        } else {
            self.col = WHITE;
        }

        //clamp position (acts as out of bounds)
        self.pos = self.pos.clamp(
            vec2(BORDER * 2., 0. + BORDER),
            vec2(SW - CONTENTFIELD, SH - BORDER),
        );

        // bullet cooldown, make sure it's clamped. prob dont need -= 0.99 but whatever
        self.level_check();
        let interval: f32 = {
            match self.level {
                1 => 0.40,
                2 => 0.60,
                3 => 0.90,
                _ => 0.0, //this here because rust will complain if not
            }
        };
        self.cooldown = self.cooldown.clamp(0., 1.);
        if self.cooldown != 0. {
            self.cooldown -= interval;
        }

        self.draw();
    }

    pub fn damage(&mut self) {
        self.hp -= 1;
        self.i_frame = 1.;
        self.points = 0;
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
                Vec2::new(0., -20.),
                bullet::BulletKind::Straight,
                Vec2::ZERO,
                GREEN,
            ));
            p_bullets.push(Bullet::new(
                Vec2::new(self.pos.x - 10., self.pos.y),
                7.,
                Vec2::new(0., -20.),
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

    fn level_check(&mut self) {
        if self.points < 1000 {
            self.level = 1
        }
        if self.points < 10000 {
            self.level = 2
        } else {
            self.level = 3
        }
    }
}
