pub use crate::*;

pub fn wave_1(
    timer: f32,
    enemies: &mut Vec<Enemy>,
    bullets: &mut Vec<Bullet>,
    boss: &mut Option<Enemy>,
    previous_time: f32,
) {
    if timer == 0. {
        //initial
        let enemy1: Enemy = Enemy::new(
            Vec2::new(HC + 300., -10.),
            15.,
            Vec2::new(0., 2.),
            0.,
            false,
            enemy::AttackType::Circle,
        );
        let enemy2: Enemy = Enemy::new(
            Vec2::new(HC - 300., -10.),
            15.,
            Vec2::new(0., 2.),
            0.,
            false,
            enemy::AttackType::Circle,
        );

        enemies.push(enemy1);
        enemies.push(enemy2);
    }
    if previous_time < 2. && timer > 2. {
        for enemy in &mut *enemies {
            //enemy.vel = Vec2::new(0., 0.);
            enemy.state = enemy::State::Hovering;
            //enemy.attack(bullets);
        }
    }
    if previous_time < 8. && timer >= 8. {
        for enemy in &mut *enemies {
            enemy.state = enemy::State::Retreating;
        }
        //println!("what am i doing")
    }
    if previous_time < 10. && timer >= 10. {
        let enemy: Enemy = Enemy::new(
            Vec2::new(HC, -10.),
            15.,
            Vec2::new(0., -1.),
            0.,
            false,
            enemy::AttackType::Fan,
        );
        enemies.push(enemy);
    }
}
