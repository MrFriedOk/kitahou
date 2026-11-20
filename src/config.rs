//pub use macroquad::prelude::*;
pub use crate::*;

pub const SH: f32 = 1080.0;
pub const SW: f32 = 1440.0;

//pub const PH: f32

pub const HC: f32 = SW / 2.0; //horizontal center
pub const VC: f32 = SH / 2.0;

pub const STARTY: f32 = VC + (VC / 2.0);

pub const START: Vec2 = Vec2::new(HC, STARTY);

//pub const FLOWER: Vec = ()
