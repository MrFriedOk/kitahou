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
    if previous_time < 1. && timer >= 1. {
        items.extend(vec![Item::new(
            Vec2::new(-200., -20.),
            ItemType::Power { value: 500. },
        )])
    }
}
