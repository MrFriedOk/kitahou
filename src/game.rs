//pub use macroquad::{Bullet,Enemy,};

//pub mod player;

//pub use crate::bullet::Bullet;
//pub use crate::player::Player;

pub use crate::*;

//pub use crate::config::*;

pub struct Game {
    player: Player,
    bullets: Vec<Bullet>,
    enemies: Vec<Enemy>,
    //bullets
}

impl Game {
    pub fn new(player: Player, bullets: Vec<Bullet>, enemies: Vec<Enemy>) -> Self {
        Self {
            player,
            bullets,
            enemies,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.player.update(dt);

        let mut bullet: Bullet = Bullet::new(
            Vec2::new(SW, SH),
            5.,
            Vec2::new(10., 10.),
            BulletKind::Straight,
        );

        self.bullets.push(bullet);

        for bullet in &mut self.bullets {
            bullet.update(dt);
        }
    }
}
