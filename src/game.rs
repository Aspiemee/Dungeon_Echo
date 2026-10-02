pub enum Location {
    Entrance, 
    DarkCave, 
    TreasureRoom, 
    TrapCorridor,
    Exit,
}

pub struct Action {
    pub description: String, 
    pub next_location: Location,
    pub required_item: Option<String>, 
    pub health_change: i32,
}

pub struct GameState {
    pub current_location: Location,
    pub inventory: Vec<String>,
    pub health: i32, 
    pub is_finished: bool, 
    pub game_over_message: Option<String>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            current_location: Location::Entrance,
            inventory: Vec::new(),
            health: 100,
            is_finished: false,
            game_over_message: None,
        }
    }
}