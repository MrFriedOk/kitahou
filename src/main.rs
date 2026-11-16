use macroquad::{
    color::*,
    shapes::{draw_circle, draw_line, draw_rectangle},
    text::draw_text,
    window::{clear_background, next_frame, screen_height, screen_width},
};

#[macroquad::main("Hi")]
async fn main() {
    loop {
        clear_background(DARKGRAY);

        next_frame().await
    }
}
