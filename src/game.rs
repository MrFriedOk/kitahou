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
    bomb: Option<Bomb>,
    player_bullets: Vec<Bullet>,
    boss: Option<Enemy>,
    timer: f32,
    previous_time: f32,
    //wave: Option<Wave>,
}

impl Game {
    pub fn new(player: Player, bullets: Vec<Bullet>, enemies: Vec<Enemy>) -> Self {
        Self {
            player,
            bullets,
            enemies,
            bomb: None,
            player_bullets: Vec::new(),
            boss: None,
            timer: 0.,
            previous_time: 0.,
            //wave: Some(Wave),
        }
    }

    pub fn update(&mut self) {
        //println!("{}", self.timer.round());
        //let mut bomb: Bomb = Bomb;

        if self.timer < 300. && self.boss.is_none() {
            wave::wave_1(
                self.timer,
                &mut self.enemies,
                &mut self.bullets,
                &mut self.boss,
                self.previous_time,
            )
        }

        self.collision_check();
        self.player.update(&mut self.player_bullets);

        //now i need to do a match, check if (borrow bomb) bomb
        if let Some(bomb) = &mut self.bomb {
            bomb.update();
        }

        if is_key_down(KeyCode::X) && self.bomb.is_none() && self.player.bombs > 0 {
            // this function should first check if the key z is down of course. Bomb shouldnt be
            // created in the main loop, but i have it as an argument.................. what to do.
            // a new bomb should ONLY be made if z is down, and new bombs shouldn't be able to be
            // made. do i store bomb in a vector? a bomb can be updated outside of this function. i
            // will try making a bomb variable, can i make a public function that just creates a new
            // bomb in main?
            // use a bomb property in the Game object that CAN hold a Bomb, but wont always be :
            // Bomb like i was trying before.
            // lets print bomb to see how options work
            self.bomb = Some(Bomb::new(self.player.pos));
            self.player.bombs -= 1;
        }

        //let mut new_enemies: Vec = Vec::new();
        for enemy in self.enemies.iter_mut() {
            enemy.update(self.player.pos, &mut self.bullets);
        }

        //let mut new_bullets: Vec = Vec::new();
        for bullet in self.bullets.iter_mut() {
            bullet.pos += bullet.vel;
            bullet.update();
        }

        if let Some(boss) = &mut self.boss {
            let mut new: Vec<Enemy> = Vec::new();
            if let Some(enemies) = boss.update(self.player.pos, &mut self.bullets) {
                new.extend(enemies);
                for enemy in new {
                    self.enemies.push(enemy);
                }
            };
        }

        for p_bullet in self.player_bullets.iter_mut() {
            p_bullet.pos += p_bullet.vel;
            p_bullet.update()
        }

        self.out_of_bounds();

        self.draw();

        self.previous_time = self.timer;
        self.timer += TICK;
    }

    fn collision_check(&mut self) {
        let player_hitbox: Circle =
            Circle::new(self.player.pos.x, self.player.pos.y, self.player.r);

        if let Some(enemy) = &mut self.boss {
            let boss_hitbox: Circle = Circle::new(enemy.pos.x, enemy.pos.y, enemy.r);
            if boss_hitbox.overlaps(&player_hitbox) && self.player.i_frame == 0. {
                self.player.damage();
            };
            self.player_bullets.retain(|bullet| {
                let p_bullet_hitbox: Circle = Circle::new(bullet.pos.x, bullet.pos.y, bullet.r);
                let hit: bool = p_bullet_hitbox.overlaps(&boss_hitbox);
                if hit {
                    enemy.damage()
                };
                !hit
            });
            if enemy.hp <= 0 {
                self.boss = None;
            }
        }

        if let Some(bomb) = &mut self.bomb
            && bomb.r > SH
        {
            self.bomb = None
        }

        for enemy in &mut self.enemies {
            let enemy_hitbox = Circle::new(enemy.pos.x, enemy.pos.y, enemy.r);

            let hit: bool = enemy_hitbox.overlaps(&player_hitbox);
            if self.player.i_frame == 0. && hit {
                self.player.damage();
            }
            self.player_bullets.retain(|bullet| {
                let p_bullet_hitbox: Circle = Circle::new(bullet.pos.x, bullet.pos.y, bullet.r);
                let hit: bool = p_bullet_hitbox.overlaps(&enemy_hitbox);
                if hit {
                    enemy.damage();
                }
                !hit
            });
        }

        for bullet in &self.bullets {
            let bullet_hitbox = Circle::new(bullet.pos.x, bullet.pos.y, bullet.r);
            let hit: bool = bullet_hitbox.overlaps(&player_hitbox);
            if self.player.i_frame == 0. && hit {
                self.player.damage();
            };
        }

        //hp checks

        self.enemies
            .retain(|enemy| if enemy.hp <= 0 { false } else { true });
    }
    fn out_of_bounds(&mut self) {
        self.bullets
            .retain(|b| !b.out_of_bounds() && !b.bombed(&self.bomb));
        self.player_bullets.retain(|b| !b.out_of_bounds());
    }

    fn draw(&self) {
        draw_rectangle(0., 0., BORDER * 2., SH, BLACK);
        draw_rectangle(0., 0., SH, BORDER, BLACK);
        draw_rectangle(0., SH - BORDER, SW, BORDER, BLACK);
        draw_rectangle(SW - CONTENTFIELD, 0., CONTENTFIELD, SH, BLACK);

        self.draw_level();
        self.draw_bomb();
        self.draw_hp();

        if let Some(enemy) = &self.boss {
            let percent: f32 = enemy.hp as f32 / BOSS_HP as f32;
            draw_arc(
                enemy.pos.x,
                enemy.pos.y,
                64,
                enemy.r + 20.,
                90.,
                15.,
                360. * percent,
                DARKPURPLE,
            )
        }

        draw_fps();
    }
    fn draw_level(&self) {
        let r: f32 = 20.;
        let d: f32 = (r * 2.) + 5.;
        let y: f32 = 250.;
        for c in 1..=9 {
            let filled: bool = self.player.level >= c;

            if filled {
                draw_circle(
                    CONTENTFIELDHC + (c as f32 * d) + (GRIDW * 2.5),
                    y,
                    r,
                    YELLOW,
                );
            } else {
                draw_circle(CONTENTFIELDHC + (c as f32 * d) + (GRIDW * 2.5), y, r, GRAY);
            }
        }
    }

    fn draw_bomb(&self) {
        let r: f32 = 20.;
        let d: f32 = (r * 2.) + 5.;
        let y: f32 = 450.;
        for c in 1..=9 {
            let filled: bool = self.player.bombs >= c;

            if filled {
                draw_circle(CONTENTFIELDHC + (c as f32 * d) + (GRIDW * 2.5), y, r, RED);
            } else {
                draw_circle(CONTENTFIELDHC + (c as f32 * d) + (GRIDW * 2.5), y, r, GRAY);
            }
        }
    }
    fn draw_hp(&self) {
        let r: f32 = 20.;
        let d: f32 = (r * 2.) + 5.;
        let y: f32 = 650.;
        for c in 1..=9 {
            let filled: bool = self.player.hp >= c;

            if filled {
                draw_circle(CONTENTFIELDHC + (c as f32 * d) + (GRIDW * 2.5), y, r, GREEN);
            } else {
                draw_circle(CONTENTFIELDHC + (c as f32 * d) + (GRIDW * 2.5), y, r, GRAY);
            }
        }
    }
}
