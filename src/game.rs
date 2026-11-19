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
    bomb: Option<Bomb>,
}

impl Game {
    pub fn new(player: Player, bullets: Vec<Bullet>, enemies: Vec<Enemy>) -> Self {
        Self {
            player,
            bullets,
            enemies,
            bomb: None,
        }
    }

    pub fn update(&mut self, tick: f32) {
        //let mut bomb: Bomb = Bomb;

        self.collision_check();
        self.player.update(tick);

        //now i need to do a match, check if (borrow bomb) bomb
        match &mut self.bomb {
            Some(bomb) => bomb.update(tick),
            None => (),
        }

        if is_key_down(KeyCode::X) {
            // this function should first check if the key z is down of course. Bomb shouldnt be
            // created in the main loop, but i have it as an argument.................. what to do.
            // a new bomb should ONLY be made if z is down, and new bombs shouldn't be able to be
            // made. do i store bomb in a vector? a bomb can be updated outside of this function. i
            // will try making a bomb variable, can i make a public function that just creates a new
            // bomb in main?
            // use a bomb property in the Game object that CAN hold a Bomb, but wont always be :
            // Bomb like i was trying before.
            self.bomb = Some(Bomb::new(self.player.pos));
        }

        for enemy in &mut self.enemies {
            enemy.update(self.player.pos, tick, &mut self.bullets);
        }

        for bullet in &mut self.bullets {
            bullet.pos += bullet.vel * tick;
            bullet.update(tick);
        }

        self.out_of_bounds();
    }

    fn collision_check(&mut self) {
        if self.bomb.is_some() {
            let bomb: Bomb = self.bomb
            if bomb.r >= 100 {}
        }
        for enemy in &self.enemies {
            let enemy_hitbox = Circle::new(enemy.pos.x, enemy.pos.y, enemy.r);
            let player_hitbox: Circle =
                Circle::new(self.player.pos.x, self.player.pos.y, self.player.r);
            let hit: bool = enemy_hitbox.overlaps(&player_hitbox);
            if !self.player.i_frame && hit {
                self.player.hp -= 1;
                self.player.i_frame = true;
                self.player.col = RED;
            }
        }
        for bullet in &self.bullets {
            let bullet_hitbox = Circle::new(bullet.pos.x, bullet.pos.y, bullet.r);
            let player_hitbox: Circle =
                Circle::new(self.player.pos.x, self.player.pos.y, self.player.r);
            let hit: bool = bullet_hitbox.overlaps(&player_hitbox);
            if !self.player.i_frame && hit {
                self.player.hp -= 1;
                self.player.i_frame = true;
                self.player.col = RED;
            }
        }
    }
    fn out_of_bounds(&mut self) {
        self.bullets
            .retain(|b| !b.out_of_bounds() && !b.bombed(&self.bomb));
    }
}
