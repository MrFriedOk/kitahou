pub use macroquad::prelude::*;

//pub mod bullet;
//pub mod config;
//pub mod enemy;
pub mod config;
pub mod game;
pub mod player;

pub use crate::config::*;
pub use bullet::Bullet;
pub use enemy::Enemy;
pub use game::Game;
pub use player::Player;

pub enum BulletKind {
    Straight,
    Accelerating { accel: f32 },
    Homing { rate: f32 },
}

pub struct Enemy {
    pos: Vec2,
    r: f32,
    vel: Vec2,
}

pub struct Bullet {
    pos: Vec2,
    r: f32,
    vel: Vec2,
    kind: BulletKind,
}

impl Enemy {
    pub fn new(pos: Vec2, r: f32, vel: Vec2) -> Self {
        Self { pos, r, vel }
    }
}

impl Bullet {
    pub fn new(pos: Vec2, r: f32, vel: Vec2, kind: BulletKind) -> Self {
        Self { pos, r, vel, kind }
    }

    pub fn move_bullet(&mut self, target: Vec2) {}

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, 5., RED)
    }

    pub fn update(&mut self, dt: f32) {
        self.draw();
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
    let bullets: Vec<Bullet> = Vec::new();
    let mut enemies: Vec<Enemy> = Vec::new();
    let mut game: Game = Game::new(player, bullets, enemies);

    enemies.push(Enemy::new(Vec2::new(500., 500.), 20., Vec2::new(10., 10.)));
    loop {
        let dt: f32 = get_frame_time();
        clear_background(DARKGRAY);

        game.update(dt);
        //player.move_input();
        //player.draw();

        next_frame().await //should always be last
    }
}
