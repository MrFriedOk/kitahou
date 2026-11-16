use macroquad::prelude::*;

const SH: f32 = 1080.0;
const SW: f32 = 1440.0;

const HC: f32 = SW / 2.0; //horizontal center
const VC: f32 = SH / 2.0;

const START: Vec2 = Vec2::new(HC, VC);
const STARTVEL: Vec2 = Vec2::new(10., 10.);

struct Player {
    hp: i8,
    pos: Vec2,
    vel: Vec2,
    r: f32,
    slow: bool,
}

impl Player {
    fn new(hp: i8, pos: Vec2, vel: Vec2, r: f32,slow: bool) -> Self {
        Self { hp, pos, vel, r, slow }
    }

    fn damage(&mut self) {

    }

    fn move(&mut self) {
        
    }

    fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.r, WHITE);
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "kitahou".to_owned(),
        window_width: SW as i32,
        window_height: SH as i32,
        window_resizable: false,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    //initialize all the things
    let mut player = Player::new(3, START, STARTVEL, 10.0,false);
    loop {
        let dt: f32 = get_frame_time();
        clear_background(DARKGRAY);

        player.draw();

        next_frame().await //should always be last
    }
}
