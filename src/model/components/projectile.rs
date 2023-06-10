use super::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ProjectileAI {
    Straight,
    ConstantTurn { degrees_per_second: R32 },
}

#[derive(StructOf, Debug)]
pub struct Projectile {
    pub fraction: Fraction,
    #[structof(nested)]
    pub body: Body,
    pub damage: Hp,
    pub target_pos: Option<vec2<Coord>>,
    pub ai: ProjectileAI,
}

impl Projectile {
    pub fn new(
        pos: vec2<Coord>,
        direction: Angle<R32>,
        fraction: Fraction,
        config: ProjectileConfig,
    ) -> Self {
        Self {
            fraction,
            body: Body::new(pos, config.shape).with_velocity(direction.unit_vec() * config.speed),
            damage: config.damage,
            target_pos: None,
            ai: config.ai,
        }
    }

    // TODO: grenades or smth
    // pub fn with_target(self, target_pos: vec2<Coord>) -> Self {
    //     Self {
    //         target_pos: Some(target_pos),
    //         ..self
    //     }
    // }
}

impl Default for ProjectileAI {
    fn default() -> Self {
        Self::Straight
    }
}
