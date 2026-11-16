use macroquad::prelude::*;

const SH: f32 = 1080.0;
const SW: f32 = 1440.0;

const HC: f32 = SW / 2.0; //horizontal center
const VC: f32 = SH / 2.0;

const START: Vec2 = Vec2::new(HC, VC);
const STARTVEL: Vec2 = Vec2::new(400., 400.);

const SLOWVEL: Vec2 = Vec2::new(200., 200.);

struct Game {
    player: Player,
    //bullets
}

struct Player {
    hp: i8,
    pos: Vec2,
    vel: Vec2,
    r: f32,
    //slow: bool,
}

impl Game {
    fn new(player: Player) -> Self {
        Self { player }
    }

    fn update(&mut self, dt: f32) {
        self.player.move_input(dt);
        self.player.slow_input(dt);
        self.player.draw();
    }
}

impl Player {
    fn new(hp: i8, pos: Vec2, vel: Vec2, r: f32) -> Self {
        Self { hp, pos, vel, r }
    }

    fn damage(&mut self) {
        todo!()
    }

    fn slow_input(&mut self, dt: f32) {
        if is_key_down(KeyCode::RightShift) {
            self.vel = SLOWVEL;
        }
        if is_key_down(KeyCode::LeftShift) {
            self.vel = SLOWVEL;
        } else {
            self.vel = STARTVEL;
        }
    }

    fn move_input(&mut self, dt: f32) {
        if is_key_down(KeyCode::Left) {
            self.pos.x -= 1.0 * (self.vel.x * dt);
        }
        if is_key_down(KeyCode::Right) {
            self.pos.x += 1.0 * (self.vel.x * dt);
        }
        if is_key_down(KeyCode::Up) {
            self.pos.y -= 1.0 * (self.vel.y * dt);
        }
        if is_key_down(KeyCode::Down) {
            self.pos.y += 1.0 * (self.vel.y * dt);
        }
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
    let player: Player = Player::new(3, START, STARTVEL, 10.0);
    let mut game: Game = Game::new(player);
    loop {
        let dt: f32 = get_frame_time();
        clear_background(DARKGRAY);

        game.update(dt);
        //player.move_input();
        //player.draw();

        next_frame().await //should always be last
    }
}
