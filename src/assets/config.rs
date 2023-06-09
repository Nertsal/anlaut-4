use super::*;

use crate::model::{Coord, Hp, Shape, Time};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Config {
    pub player: PlayerConfig,
    pub camera: CameraConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct CameraConfig {
    pub fov: Coord,
    pub speed: Coord,
    /// Radius in which the camera allows the target to move without affecting the camera.
    pub dead_zone: Coord,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct PlayerConfig {
    pub human_state: PlayerStateConfig,
    pub barrel_state: PlayerStateConfig,
    pub speed: Coord,
    pub acceleration: Coord,
    pub hp: Hp,
    pub gun: GunConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct PlayerStateConfig {
    pub shape: Shape,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct GunConfig {
    /// Delay between shots.
    pub shot_delay: Time,
    pub projectile: ProjectileConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct ProjectileConfig {
    pub speed: Coord,
    pub damage: Hp,
    pub shape: Shape,
}

impl Config {
    pub async fn load(path: impl AsRef<std::path::Path>) -> anyhow::Result<Self> {
        let config = file::load_detect(path).await?;
        Ok(config)
    }
}
