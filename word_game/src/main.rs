use std::error;
use word_game::game::Wordgame;

fn main() -> Result<(), Box<dyn error::Error>> {
    Wordgame::build()
}
