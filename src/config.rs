pub use macroquad::prelude::*;

pub const SH: f32 = 1080.0;
pub const SW: f32 = 1440.0;

pub const HC: f32 = SW / 2.0; //horizontal center
pub const VC: f32 = SH / 2.0;

pub const STARTY: f32 = VC + (VC / 2.0);

pub const START: Vec2 = Vec2::new(HC, STARTY);

pub struct Conf {
    window_title: String,
    window_width: i32,
    window_height: i32,
    fullscreen: bool,
    ..Default::default()
}

impl Conf {
    pub fn new(&mut self) -> Self{
        Self{
            self.window_title: String,
            window_width: SW as i32,
            window_height: SH as i32,
            window_resizable: false,
            fullscreen: false,
            ..Default::default()
        }
    }
}
