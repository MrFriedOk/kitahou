pub use crate::*;

pub enum State {
    Hovering,
    Moving,
    Retreating,
}

pub enum AttackType {
    Circle,
    Flower,
    Straight,
    Homing,
    Fan,
}

pub struct Enemy {
    pub pos: Vec2,
    pub r: f32,
    pub vel: Vec2,
    pub cooldown: f32,
    rotate: bool,
    pub state: State,
    pub attack_type: AttackType,
}

impl Enemy {
    pub fn new(
        pos: Vec2,
        r: f32,
        vel: Vec2,
        cooldown: f32,
        rotate: bool,
        attack_type: AttackType,
    ) -> Self {
        Self {
            pos,
            r,
            vel,
            cooldown,
            rotate,
            state: State::Moving,
            attack_type,
        }
    }

    pub fn update(&mut self, player_pos: Vec2, bullets: &mut Vec<Bullet>) {
        //self.pos += Pattern::new(12., 5., 1., 1., 5.).flower()

        match self.state {
            State::Moving => self.pos += self.vel,
            State::Hovering => self.attack(bullets, player_pos),
            State::Retreating => self.pos -= self.vel,
        }

        //let rotate: bool = if self.cooldown <= 0.5 { true } else { false };

        //self.attack()

        self.draw();
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.r, RED);
    }

    pub fn attack(&mut self, bullets: &mut Vec<Bullet>, player_pos: Vec2) {
        match self.attack_type {
            AttackType::Circle => {
                if self.cooldown <= 0. {
                    Pattern::new(12).circle(self.pos, bullets, self.rotate);

                    self.cooldown = 1.;
                    self.rotate = !self.rotate;
                } else {
                    self.cooldown -= 0.02;
                }
            }
            AttackType::Flower => (),
            AttackType::Straight => (),
            AttackType::Homing => (),
            AttackType::Fan => {
                if self.cooldown <= 0. {
                    Pattern::new(10).fan(self.pos, bullets, player_pos);
                    self.cooldown = 1.;
                } else {
                    self.cooldown -= 0.01;
                }
            }
        }
    }
}
