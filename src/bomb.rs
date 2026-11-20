use crate::*;
#[derive(Debug)]
pub struct Bomb {
    pub pos: Vec2,
    pub r: f32,
}

impl Bomb {
    pub fn new(pos: Vec2) -> Self {
        Self { pos, r: 0. }
    }

    pub fn update(&mut self, tick: f32) {
        self.r += 175. * tick;
        self.draw()
    }

    fn draw(&self) {
        // will not be needed unless debugging
        //draw_circle(self.pos.x, self.pos.y, self.r)
    }
}
