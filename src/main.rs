#![deny(clippy::all)]
#![deny(clippy::pedantic)]
//to have all Clippy lints see https://github.com/rust-lang/rust-clippy/
//! # Snake Game using Ratatui
//!
//! This module implements a terminal-based snake game using the Ratatui crate for rendering.
//!
//! ## Features
//! - **Terminal UI**: Uses Ratatui for rendering a grid-based game.
//! - **Game Logic**: Manages snake movement, collisions, and scoring.
//! - **Multithreading**: Uses multiple threads for input handling, rendering at 60 FPS, and game logic execution.
//! - **Emoji-based graphics**: Supports rendering the snake using emojis instead of ASCII.
//! - **Configurable parameters**: Plans to integrate `clap` for command-line arguments.
//!
//! ## TODO
//! - [ ] Implement `clap` for command-line argument parsing.
//! - [ ] Improve 60 FPS accuracy with precise timing and configuration.
//! - [ ] Adjust rendering logic to distinguish `case_width` and `case_height`.
//! - [ ] Some tests example
//!
//! ## References
//! - Clippy lints: <https://github.com/rust-lang/rust-clippy/>
//! - Ratatui tutorial: <https://ratatui.rs/tutorials/hello-world/>
//! - Example: <https://ratatui.rs/examples/widgets/canvas/>
//!
//! ## Architecture
//! - Uses `RwLock` for synchronization.
//! - Spawns separate threads for input handling, rendering (60Hz), and game logic execution.
//! - The game logic thread runs at twice the rendering speed to maintain responsiveness.
//! ## Documentation generation
//! - `cargo doc --document-private-items --no-deps --open`

mod controls;
mod game;
mod graphics;

use crate::game::Game as Jeu;
use controls::speed::Speed;

use crate::controls::speed::Velocity;
use crate::graphics::graphic_block::Position;
use crate::graphics::snake_body::SnakeBody;
use graphics::map::Map as Carte;

fn main() {
    let case_size = 2;
    let ini_position = Position { x: 80, y: 5 };
    let mut terminal = ratatui::init();

    let map: Carte = Carte::new(case_size, terminal.get_frame().area());
    let speed: Speed = Speed::new(&Velocity::Normal);
    //if refactoring: builder pattern possible (here we create the graphics only once)
    let serpent: SnakeBody = SnakeBody::new("❄️", "🎄", 10, ini_position, case_size);
    let mut jeu: Jeu = game::Game::new(speed, serpent, map, 3, 5, terminal);
    jeu.start();
    ratatui::restore();
}
