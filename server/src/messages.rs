use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::map::Map;

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientMessage {
    Join { name: String },
    Move { direction: (f32, f32) },
    Shoot { direction: (f32, f32) },
    Ping,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMessage {
    Welcome { map: Map, player_id: String, difficulty: u8 },
    GameState { players: HashMap<String, (f32, f32, f32, bool)> },  // x, y, rotation, is_alive
    PlayerShot { shooter: String, target: String },
    PlayerDied { player: String },
    GameOver { winner: String, scores: Vec<(String, u32)> },
    Pong
}
