pub use crate::*;

pub fn wave_1(
    timer: f32,
    enemies: &mut Vec<Enemy>,
    bullets: &mut Vec<Bullet>,
    boss: &mut Option<Enemy>,
) {
    if timer == 0. {
        //initial
        let enemy1: Enemy = Enemy::new(
            Vec2::new(HC + 300., -10.),
            15.,
            Vec2::new(0., 2.),
            0.,
            false,
        );
        let enemy2: Enemy = Enemy::new(
            Vec2::new(HC - 300., -10.),
            15.,
            Vec2::new(0., 2.),
            0.,
            false,
        );

        enemies.push(enemy1);
        enemies.push(enemy2);
    }
    if timer > 2. {
        for enemy in &mut *enemies {
            //enemy.vel = Vec2::new(0., 0.);
            enemy.state = enemy::State::Hovering;
            enemy.attack(bullets);
        }
    }
    if timer > 8. {
        for enemy in &mut *enemies {
            enemy.state = enemy::State::Retreating;
        }
        //println!("what am i doing")
    }
}
