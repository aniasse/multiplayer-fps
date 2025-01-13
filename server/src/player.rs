
use serde::{Serialize, Deserialize};

pub const PLAYER_SPEED: f32 = 0.3;
pub const SHOOT_RANGE: f32 = 10.0;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Player {
    pub name: String,
    pub position: (f32, f32),
    pub is_alive: bool,
    pub points: u32,
    pub  rotation: f32,
    pub elapsed: u64,
}