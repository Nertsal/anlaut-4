mod collider;

pub use self::collider::*;

use super::*;

#[derive(StructOf, Debug)]
pub struct Body {
    #[structof(nested)]
    pub collider: Collider,
    pub velocity: vec2<Coord>,
}

#[derive(StructOf, Debug)]
pub struct Projectile {
    #[structof(nested)]
    pub body: Body,
    pub damage: Hp,
}

#[derive(StructOf, Debug)]
pub struct Actor {
    #[structof(nested)]
    pub body: Body,
    pub health: Health,
    // #[structof(nested)] // TODO: optional nesting
    pub gun: Option<Gun>,
}

impl Body {
    pub fn new(pos: vec2<Coord>, shape: Shape) -> Self {
        Self {
            collider: Collider::new(pos, shape),
            velocity: vec2::ZERO,
        }
    }

    pub fn with_velocity(self, velocity: vec2<Coord>) -> Self {
        Self { velocity, ..self }
    }
}

impl Projectile {
    pub fn new(pos: vec2<Coord>, target: vec2<Coord>, config: ProjectileConfig) -> Self {
        Self {
            body: Body::new(pos, config.shape)
                .with_velocity((target - pos).normalize_or_zero() * config.speed),
            damage: config.damage,
        }
    }
}

impl Actor {
    pub fn new(body: Body, hp: Hp, gun: GunConfig) -> Self {
        Self {
            body,
            health: Health::new(hp),
            gun: Some(Gun::new(gun)),
        }
    }
}
