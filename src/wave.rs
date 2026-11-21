use crate::enemy::AttackType;
pub use crate::*;

pub fn wave_1(
    timer: f32,
    enemies: &mut Vec<Enemy>,
    bullets: &mut Vec<Bullet>,
    boss: &mut Option<Enemy>,
    previous_time: f32,
) {
    if timer == 0. {
        enemies.clear();
        let new_boss: Enemy = Enemy::new(
            Vec2::new(HC, -10.),
            30.,
            Vec2::new(0., 4.), //.lerp(Vec2::new(0., 8.), 5.),
            0.,
            false,
            AttackType::Fan,
            enemy::Kind::Boss,
        );
        *boss = Some(new_boss);
    }
}
