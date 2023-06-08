mod collider;
mod logic;
mod shape;

pub use self::{collider::*, shape::*};

use crate::{
    assets::config::*,
    util::{Mat3RealConversions, RealConversions, Vec2RealConversions},
};

use ecs::{arena::Arena, prelude::*};
use geng::prelude::*;

pub type Color = Rgba<f32>;
pub type Time = R32;
pub type Coord = R32;
pub type Id = ecs::arena::Index;

#[derive(Debug)]
pub struct Player {
    pub body: Id,
    pub player_direction: vec2<Coord>,
    pub target_velocity: vec2<Coord>,
    pub out_of_view: bool,
}

#[derive(StructOf, Debug)]
pub struct Body {
    #[structof(nested)]
    pub collider: Collider,
    pub velocity: vec2<Coord>,
}

#[derive(Debug)]
pub struct Camera {
    pub center: vec2<Coord>,
    pub fov: Coord,
    pub target_position: vec2<Coord>,
}

impl Camera {
    pub fn new(fov: impl Float) -> Self {
        Self {
            center: vec2::ZERO,
            fov: fov.as_r32(),
            target_position: vec2::ZERO,
        }
    }

    pub fn to_camera2d(&self) -> geng::Camera2d {
        geng::Camera2d {
            center: self.center.as_f32(),
            rotation: 0.0,
            fov: self.fov.as_f32(),
        }
    }
}

pub struct Model {
    pub config: Config,
    pub camera: Camera,
    pub player: Player,
    pub bodies: StructOf<Arena<Body>>,
}

impl Model {
    pub fn new(config: Config) -> Self {
        let mut bodies = StructOf::<Arena<Body>>::new();
        let player_body = bodies.insert(Body {
            collider: Collider::new(vec2::ZERO, Shape::Circle { radius: r32(1.0) }),
            velocity: vec2::ZERO,
        });

        Self {
            camera: Camera::new(config.camera.fov),
            player: Player {
                body: player_body,
                player_direction: vec2::ZERO,
                target_velocity: vec2::ZERO,
                out_of_view: false,
            },
            bodies,
            config,
        }
    }
}