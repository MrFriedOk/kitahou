pub use macroquad::prelude::*;

//pub mod bullet;
//pub mod config;
//pub mod enemy;
pub mod config;
pub mod game;
pub mod player;

//pub use bullet::Bullet;
//pub use enemy::Enemy;
pub use crate::config::*;
pub use game::Game;
pub use player::Player;

enum BulletKind {
    Straight,
    Accelerating { accel: f32 },
    Homing { rate: f32 },
}

struct Enemy {
    pos: Vec2,
    r: f32,
}

struct Bullet {
    pos: Vec2,
    r: f32,
    vel: Vec2,
    kind: BulletKind,
}

impl Bullet {
    fn new(pos: Vec2, r: f32, vel: Vec2, kind: BulletKind) -> Self {
        Self { pos, r, vel, kind }
    }

    fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, 5., WHITE)
    }

    fn update(&mut self, dt: f32) {}
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
    let bullets: Vec<Bullet> = Vec::new();
    let enemies: Vec<Enemy> = Vec::new();
    let mut game: Game = Game::new(player, bullets, enemies);
    loop {
        let dt: f32 = get_frame_time();
        clear_background(DARKGRAY);

        game.update(dt);
        //player.move_input();
        //player.draw();

        next_frame().await //should always be last
    }
}
