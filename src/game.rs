//pub use macroquad::{Bullet,Enemy,};

//pub mod player;

//pub use crate::bullet::Bullet;
//pub use crate::player::Player;

use crate::bullet::BulletKind;
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

        for enemy in &mut self.enemies {
            enemy.update(dt, self.player.pos);
            Pattern::new(12, 1., 1., 5., self.bullets.clone()).flower(enemy.pos)
        }

        for bullet in &mut self.bullets {
            bullet.update(dt);
        }

        self.collision_check();
    }

    fn collision_check(&mut self) {
        for enemy in &self.enemies {
            if enemy.pos.x - 20. >= self.player.pos.x {
                self.player.hp -= 1;
                self.player.col = RED;
            }
        }
    }
}
