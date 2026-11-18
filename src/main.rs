pub use macroquad::prelude::*;

//pub mod bullet;
pub mod bullet;
pub mod config;
pub mod enemy;
pub mod game;
pub mod pattern;
pub mod player;

pub use crate::config::*;
pub use bullet::Bullet;
pub use enemy::Enemy;
pub use game::Game;
pub use pattern::Pattern;
pub use player::Player;

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
    let player: Player = Player::new(3, START, 10.0, false, WHITE);
    let bullets: Vec<Bullet> = Vec::new();
    let mut enemies: Vec<Enemy> = Vec::new();

    enemies.push(Enemy::new(
        Vec2::new(500., 500.),
        20.,
        Vec2::new(10., 10.),
        player.pos,
        0,
    ));
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
