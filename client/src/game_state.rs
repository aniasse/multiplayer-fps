use bevy::prelude::*;
use std::collections::HashMap;
use crate::map::Map;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum AppState {
    #[default]
    Loading,
    RenderMap,
    Playing,
    GameOver,
}

#[derive(Resource)]
pub struct GameState {
    pub player_name: String,
    pub player_id: Option<String>,
    pub players: HashMap<String, (f32, f32, f32, bool)>,  // x, y, rotation, is_alive
    pub map: Option<Map>,
    pub map_rendered: bool,
    pub last_shoot_time: f32,
    pub is_alive: bool,
    pub game_over_results: Option<(String, Vec<(String, u32)>)>,
}
        
impl GameState {
    pub fn new(player_name: String) -> Self {
        Self {
            player_name,
            player_id: None,
            players: HashMap::new(),
            map: None,
            map_rendered: false,
            last_shoot_time: 0.0,
            is_alive: true,
            game_over_results: None,
        }
    }
}

