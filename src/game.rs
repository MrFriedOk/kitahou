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
            enemy.update(dt, self.player.pos, &mut self.bullets);
        }

        for bullet in &mut self.bullets {
            bullet.pos += bullet.vel;
            bullet.update(dt);
        }

        self.out_of_bounds();

        self.collision_check();
    }

    fn collision_check(&mut self) {
        for enemy in &self.enemies {
            if !self.player.i_frame && enemy.pos.x - 20. >= self.player.pos.x {
                self.player.hp -= 1;
                self.player.i_frame = true;
                self.player.col = RED;
            }
        }
    }
    fn out_of_bounds(&mut self) {
        // self.bullets.retain(|b| b.pos.x <= SW) || b.pos.x <= 20.);
    }
}
