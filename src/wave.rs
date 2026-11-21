use crate::enemy::AttackType;
pub use crate::*;

pub fn wave_1(
    timer: f32,
    enemies: &mut Vec<Enemy>,
    bullets: &mut Vec<Bullet>,
    boss: &mut Option<Enemy>,
    previous_time: f32,
) {
    if previous_time < 0. && timer >= 0. {
        enemies.clear();
        let new_boss: Enemy = Enemy::new(
            Vec2::new(HC, -10.),
            30.,
            Vec2::new(0., 10.),
            0.,
            false,
            AttackType::Fan,
            enemy::Kind::Boss,
        );
        *boss = Some(new_boss);
    }
}
