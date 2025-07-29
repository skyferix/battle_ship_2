pub enum Player {
    Computer,
    Player,
}

impl Player {
    pub fn from_str(input: &str) -> Option<Self> {
        match input.to_lowercase().as_str() {
            "c" => Some(Player::Computer),
            "p" => Some(Player::Player),
            _ => None,
        }
    }
}