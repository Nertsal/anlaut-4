mod action;
mod camera;
mod components;
mod effect;
mod gen;
mod health;
mod logic;
mod player;
mod position;
mod shake;
mod waves;
mod weapons;

pub use self::{
    action::*, camera::*, components::*, effect::*, health::*, player::*, position::*, shake::*,
    waves::*, weapons::*,
};

use crate::{
    assets::{config::*, theme::Theme, waves::*},
    util::{RealConversions, Vec2RealConversions},
};

use std::collections::VecDeque;

use ecs::{arena::Arena, prelude::*};
use geng::prelude::*;

pub type Color = Rgba<f32>;
pub type Time = R32;
pub type Coord = R32;
pub type Id = ecs::arena::Index;
pub type Lifetime = Health;

#[derive(StructOf, Debug, Clone)]
pub struct Explosion {
    pub position: Position,
    pub max_radius: Coord,
    pub lifetime: Lifetime,
}

#[derive(StructOf, Debug)]
pub struct Particle {
    pub position: Position,
    pub size: Coord,
    pub velocity: vec2<Coord>,
    pub lifetime: Lifetime,
    pub kind: ParticleKind,
}

#[derive(Debug, Clone, Copy)]
pub enum ParticleKind {
    Fire,
}

#[derive(StructOf, Debug)]
pub struct Block {
    #[structof(nested)]
    pub collider: Collider,
    pub health: Option<Health>,
    pub color: Color,
    pub kind: BlockKind,
    pub explosion: Option<ExplosionConfig>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum BlockKind {
    Obstacle,
    Barrel,
}

pub struct Model {
    pub theme: Theme,
    pub time: Time,
    pub config: Config,
    pub screen_shake: ScreenShake,
    pub camera: Camera,
    pub enemies_list: HashMap<String, EnemyConfig>,
    pub wave_manager: WaveManager,
    pub player: Player,
    pub actors: StructOf<Arena<Actor>>,
    pub blocks: StructOf<Arena<Block>>,
    pub projectiles: StructOf<Arena<Projectile>>,
    pub gasoline: StructOf<Arena<Gasoline>>,
    pub fire: StructOf<Arena<Fire>>,
    pub explosions: StructOf<Arena<Explosion>>,
    pub particles: StructOf<Arena<Particle>>,
    pub queued_effects: VecDeque<QueuedEffect>,
}

impl Model {
    pub fn new(
        theme: Theme,
        config: Config,
        level: LevelConfig,
        enemies: HashMap<String, EnemyConfig>,
        waves: WavesConfig,
    ) -> Self {
        let mut actors = StructOf::new();
        let mut model = Self {
            theme,
            time: Time::ZERO,
            screen_shake: ScreenShake::new(),
            camera: Camera::new(config.camera.fov),
            player: Player::init(config.player.clone(), &mut actors),
            actors,
            blocks: StructOf::new(),
            projectiles: StructOf::new(),
            gasoline: StructOf::new(),
            fire: StructOf::new(),
            explosions: StructOf::new(),
            particles: StructOf::new(),
            wave_manager: WaveManager::new(waves),
            enemies_list: enemies,
            queued_effects: VecDeque::new(),
            config,
        };
        model.init(level);
        model
    }

    fn init(&mut self, level: LevelConfig) {
        // TODO: navmesh
        self.generate_level(level);
    }
}
