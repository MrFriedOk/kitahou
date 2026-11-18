//pub use macroquad::prelude::*;

//pub use crate::config::*;

pub use crate::*;

pub struct Player {
    //r = radius
    pub hp: i8,
    pub pos: Vec2,
    r: f32,
    //slow: bool,
    pub i_frame: bool,
    pub col: Color,
}

impl Player {
    pub fn new(hp: i8, pos: Vec2, r: f32, i_frame: bool, col: Color) -> Self {
        Self {
            hp,
            pos,
            r,
            i_frame,
            col,
        }
    }

    pub fn update(&mut self, dt: f32) {
        let dir: Vec2 = self.read_dir();

        self.pos += dir * dt;

        self.pos = self.pos.clamp(Vec2::ZERO, vec2(SW, SH));

        self.draw();
    }

    pub fn damage(&mut self) {
        self.hp -= 1;
        self.i_frame = true;
    }

    pub fn read_dir(&mut self) -> Vec2 {
        let mut dir: Vec2 = Vec2::new(0., 0.);
        let mut speed: f32 = 600.;

        if is_key_down(KeyCode::LeftShift) {
            speed = 200.0;
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

        if self.pos.x > SW {
            //shut up clippy, i have a clamp!
            self.pos.x = SW;
        }
        if self.pos.x < 0. {
            self.pos.x = 0.;
        }
        if self.pos.y > SH {
            //dir.x = 0.;
            self.pos.y = SH;
        }
        if self.pos.y < 0. {
            self.pos.y = 0.;
        }

        dir
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.r, self.col);
    }
}
