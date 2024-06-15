pub mod board;
pub mod game;
pub mod keyboard;

use game::Game;

// A 2048 game in Rust
fn main() {
    let mut game: Game = Game::new();

    game.init();

    game.play();
}
