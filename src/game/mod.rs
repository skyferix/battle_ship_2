pub mod player;
pub mod difficulty;

pub use player::Player;
pub use difficulty::Difficulty;

use crate::io;

pub fn run_game() {
    let player = io::ask("Choose player (c/p):", Player::from_str);
    let difficulty = io::ask("Choose difficulty (easy/hard):", Difficulty::from_str);
}