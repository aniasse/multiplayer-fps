use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::{Instant, Duration};
use crate::map::Map;
use crate::player::Player;

pub struct GameState {
    pub players: HashMap<SocketAddr, Player>,
    pub map: Map,
    pub difficulty: u8,
    pub game_start_time: Instant,
    pub game_duration: Duration,
    pub rotation: f32,
}

impl GameState {
    pub fn new(difficulty: u8) -> Self {
        Self {
            players: HashMap::new(),
            map: Map::new(difficulty),
            difficulty,
            game_start_time: Instant::now(),    
            game_duration: Duration::from_secs(300), // 5 minutes
            rotation: 0.0,
        }
    }

    pub fn is_game_over(&self) -> bool {
        self.game_start_time.elapsed() >= self.game_duration
    }
    
}