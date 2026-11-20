//pub use macroquad::prelude::*;
pub use crate::*;

pub const SH: f32 = 1080.0;
pub const SW: f32 = 1440.0;

//pub const PH: f32

pub const GRIDW: f32 = 40.;
pub const GRIDH: f32 = 40.;

pub const CONTENTFIELD: f32 = GRIDW * 14.;
pub const CONTENTFIELDHC: f32 = (CONTENTFIELD + (CONTENTFIELD / 2.));

pub const BORDER: f32 = GRIDH;

pub const HC: f32 = (SW - CONTENTFIELD - BORDER * 10.); //horizontal center ok ngl i totally brute
//forced "10". if i had actually used a gride system, it'd be easier but that's totally ok.
pub const VC: f32 = SH / 2.0;

pub const STARTY: f32 = VC + (VC / 2.0) + BORDER * 2.;

pub const START: Vec2 = Vec2::new(HC, STARTY);

//pub const FLOWER: Vec = ()
