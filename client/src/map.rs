use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Map {
    pub cells: Vec<Vec<bool>>,
    pub internal_wall_count: usize, 
    pub map_width: usize,
    pub map_height: usize,
}
