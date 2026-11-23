use crate::enemy::AttackType;
use crate::item::ItemType;
pub use crate::*;

pub fn wave_1(
    timer: f32,
    enemies: &mut Vec<Enemy>,
    bullets: &mut Vec<Bullet>,
    boss: &mut Option<Enemy>,
    previous_time: f32,
    items: &mut Vec<Item>,
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
            enemy::Kind::Enemy,
        );
        let enemy2: Enemy = Enemy::new(
            Vec2::new(HC - 300., -10.),
            15.,
            Vec2::new(0., 2.),
            0.,
            false,
            enemy::AttackType::Circle,
            enemy::Kind::Enemy,
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
    if previous_time < 9. && timer >= 9. {
        let enemy: Enemy = Enemy::new(
            Vec2::new(HC, -10.),
            15.,
            Vec2::new(0., 6.),
            0.,
            false,
            enemy::AttackType::Fan,
            enemy::Kind::Enemy,
        );
        enemies.push(enemy);
    }
    if previous_time < 10. && timer >= 10. {
        //retain only the fan enemies ig?
        enemies.retain(|e| matches!(e.attack_type, AttackType::Fan));
        for enemy in &mut *enemies {
            enemy.state = enemy::State::Hovering;
        }
    }
    if previous_time < 11. && timer >= 11. {
        //retain only the fan enemies ig?
        enemies.retain(|e| matches!(e.attack_type, AttackType::Fan));
        for enemy in &mut *enemies {
            enemy.state = enemy::State::Retreating;
        }
    }
    if previous_time < 15. && timer >= 15. {
        enemies.clear();
        let new_boss: Enemy = Enemy::new(
            Vec2::new(HC, -10.),
            70.,               //boss' hitboxes are massive
            Vec2::new(0., 1.), //.lerp(Vec2::new(0., 8.), 5.),
            0.,
            false,
            AttackType::Fan,
            enemy::Kind::Boss,
        );
        *boss = Some(new_boss);
    }
    if previous_time < 16. && timer >= 16. {
        items.extend(vec![
            Item::new(
                Vec2::new(HC - 200., -20.),
                ItemType::Power { value: 100000. },
            ),
            Item::new(Vec2::new(HC + 200., -20.), ItemType::Power { value: 500. }),
        ]);
        //if let Some(boss) = boss {
        //    boss.attack(bullets,)
        //}
    }
    if previous_time < 18. && timer >= 18. {
        if let Some(boss) = boss {
            boss.speed = 0.02;
            boss.target = Vec2::new(BOSS_X_MIN + 50., BOSS_Y_MIN + 50.)
        }
    }
}
