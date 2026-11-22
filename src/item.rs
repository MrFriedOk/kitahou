pub use crate::*;

pub enum ItemType {
    Point,
    Bomb,
    Life,
}

pub struct Item {
    vel: Vec2
    pos: Vec2
    kind: ItemType

}
