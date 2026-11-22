pub use crate::*;

pub enum ItemType {
    Point,
    Bomb,
    Life,
}

pub struct Item {
    vel: Vec2,
    pub pos: Vec2,
    pub kind: ItemType,
    pub width: f32,
    pub height: f32,
}

impl Item {
    pub fn new(pos: Vec2, kind: ItemType) -> Self {
        Self {
            vel: Vec2::new(0., 1.),
            pos,
            kind,
            width: 15.,
            height: 15.,
        }
    }
    pub fn update(&mut self) {
        self.pos += self.vel;
        self.draw();
    }
    fn draw(&self) {
        match self.kind {
            ItemType::Point => {
                draw_rectangle(self.pos.x, self.pos.y, self.width, self.height, DARKBLUE)
            }
            ItemType::Bomb => draw_rectangle(self.pos.x, self.pos.y, self.width, self.height, RED),
            ItemType::Life => {
                draw_rectangle(self.pos.x, self.pos.y, self.width, self.height, GREEN)
            }
        }
    }
}
