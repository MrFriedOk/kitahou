pub use crate::*;

pub enum ItemType {
    Point,
    Bomb,
    Life,
}

pub struct Item {
    vel: Vec2,
    pos: Vec2,
    kind: ItemType,
}

impl Item {
    pub fn new(pos: Vec2, kind: ItemType) -> Self {
        Self {
            vel: Vec2::new(0., 1.),
            pos,
            kind,
        }
    }
    pub fn update(&mut self) {
        self.pos += self.vel;
        self.draw();
    }
    fn draw(&self) {
        draw_rectangle(self.pos.x, self.pos.y, 20., 40., DARKBLUE);
    }
}
