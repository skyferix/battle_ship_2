pub enum Difficulty {
    Easy,
    Hard,
}

impl Difficulty {
    pub fn from_str(input: &str) -> Option<Self> {
        match input.to_lowercase().as_str() {
            "easy" => Some(Difficulty::Easy),
            "hard" => Some(Difficulty::Hard),
            _ => None,
        }
    }
}
