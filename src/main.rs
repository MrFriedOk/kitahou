pub use macroquad::prelude::*;
use std::time::{Duration, Instant};

//pub mod bullet;
pub mod bomb;
pub mod bullet;
pub mod config;
pub mod enemy;
pub mod game;
pub mod pattern;
pub mod player;

pub use crate::config::*;
pub use bomb::Bomb;
pub use bullet::Bullet;
pub use enemy::Enemy;
pub use game::Game;
pub use pattern::Pattern;
pub use player::Player;

const TICK: f32 = 1. / 60.;

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
    //let mut accumulator: f32 = 0.;
    //initialize all the things
    let player: Player = Player::new(3, START, 10.0, WHITE);
    let bullets: Vec<Bullet> = Vec::new();
    let mut enemies: Vec<Enemy> = Vec::new();
    let mut next_tick = Instant::now();
    let duration = Duration::from_secs_f32(TICK); //f32 = get_frame_time();

    enemies.push(Enemy::new(
        Vec2::new(500., 500.),
        20.,
        Vec2::new(10., 10.),
        player.pos,
        0.,
        false,
    ));
    let mut game: Game = Game::new(player, bullets, enemies);

    loop {
        clear_background(DARKGRAY);

        //accumulator += dt;
        //while accumulator >= dt {
        //    game.update(dt);

        //    accumulator -= dt;
        //}

        game.update();

        next_tick += duration;
        let now = Instant::now();
        if next_tick > now {
            spin_sleep::sleep(next_tick - now);
        } else {
            next_tick = now;
        }

        //clear_background(DARKGRAY);

        //game.update(dt);
        //player.move_input();
        //player.draw();

        next_frame().await //should always be last
    }
}
