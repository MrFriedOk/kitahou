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

#[derive(Clone)]
pub enum Kind {
    Boss,
    Enemy,
}

pub struct Enemy {
    pub pos: Vec2,
    pub r: f32,
    pub vel: Vec2,
    pub cooldown: f32,
    rotate: bool,
    pub state: State,
    pub attack_type: AttackType,
    pub kind: Kind,
    pub hp: i32,
    p: f32,
}

impl Enemy {
    pub fn new(
        pos: Vec2,
        r: f32,
        vel: Vec2,
        cooldown: f32,
        rotate: bool,
        attack_type: AttackType,
        kind: Kind,
    ) -> Self {
        Self {
            pos,
            r,
            vel,
            cooldown,
            rotate,
            state: State::Moving,
            attack_type,
            kind: kind.clone(),
            hp: match kind {
                Kind::Boss => 10000,
                Kind::Enemy => 10,
            },
            p: 0.,
        }
    }

    pub fn update(&mut self, player_pos: Vec2, bullets: &mut Vec<Bullet>) {
        //self.pos += Pattern::new(12., 5., 1., 1., 5.).flower()

        self.p += 0.01;

        match self.state {
            State::Moving => self.pos += self.vel,
            State::Hovering => self.attack(bullets, player_pos),
            State::Retreating => self.pos -= self.vel,
        }

        if let Kind::Boss = self.kind {
            self.movement();
        }

        //let rotate: bool = if self.cooldown <= 0.5 { true } else { false };

        //self.attack()

        self.draw();
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.r, RED);
    }

    fn movement(&mut self) {
        if self.pos.x > HC - 100. && self.pos.x < HC + 100. {
            // move left or right
            //self.pos = self.pos.lerp(Vec2::new(6., 5.), 0.5);
            self.pos = self
                .pos
                .lerp(Vec2::new(self.pos.x, self.pos.y + 8.), self.p);
        }
    }

    pub fn attack(&mut self, bullets: &mut Vec<Bullet>, player_pos: Vec2) {
        match self.attack_type {
            AttackType::Circle => {
                if self.cooldown <= 0. {
                    Pattern::new(20).circle(self.pos, bullets, self.rotate);

                    self.cooldown = 1.;
                    self.rotate = !self.rotate;
                } else {
                    self.cooldown -= 0.03;
                }
            }
            AttackType::Flower => (),
            AttackType::Straight => (),
            AttackType::Homing => (),
            AttackType::Fan => {
                if self.cooldown <= 0. {
                    Pattern::new(14).fan(self.pos, bullets, player_pos);
                    self.cooldown = 1.;
                } else {
                    self.cooldown -= 0.06;
                }
            }
        }
    }
}
