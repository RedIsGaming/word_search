use std::error;
use word_game::game::Wordgame;

fn main() -> Result<(), Box<dyn error::Error>> {
    let game = Wordgame::new();
    game
}
