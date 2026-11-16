use macroquad::prelude::*;

const SH: f32 = 1080.0;
const SW: f32 = 1440.0;

const HC: f32 = SW / 2.0; //horizontal center
const VC: f32 = SH / 2.0;

const STARTY: f32 = VC / 2.0;

const START: Vec2 = Vec2::new(HC, STARTY);

struct Game {
    player: Player,
    //bullets
}

struct Player {
    hp: i8,
    pos: Vec2,
    r: f32,
    //slow: bool,
    i_frame: bool,
}

impl Game {
    fn new(player: Player) -> Self {
        Self { player }
    }

    fn update(&mut self, dt: f32) {
        self.player.update(dt);
    }
}

impl Player {
    fn new(hp: i8, pos: Vec2, r: f32, i_frame: bool) -> Self {
        Self {
            hp,
            pos,
            r,
            i_frame,
        }
    }

    fn update(&mut self, dt: f32) {
        let dir: Vec2 = self.read_dir();

        self.pos += dir * dt;

        self.pos = self.pos.clamp(Vec2::ZERO, vec2(SW, SH));

        self.draw();
    }

    fn damage(&mut self) {
        self.hp -= 1;
        self.i_frame = true;
    }

    fn read_dir(&mut self) -> Vec2 {
        let mut dir: Vec2 = Vec2::new(0., 0.);
        let speed: f32 = if is_key_down(KeyCode::LeftShift) {
            300.0
        } else {
            600.0
        };
        if is_key_down(KeyCode::Up) {
            dir.y -= speed
        }
        if is_key_down(KeyCode::Down) {
            dir.y += speed
        }
        if is_key_down(KeyCode::Left) {
            dir.x -= speed
        }
        if is_key_down(KeyCode::Right) {
            dir.x += speed
        }

        if self.pos.x > SW {
            //dir.x = 0.;
            self.pos.x = SW;
        }
        if self.pos.x < 0. {
            self.pos.x = 0.;
        }

        dir
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
    let player: Player = Player::new(3, START, 10.0, false);
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
