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
//! - **Configurable parameters**: With `clap` for command-line arguments.
//!
//! ## TODO
//! - [ ] Add a save score (local db) with a pseudo got from cmdline
//! - [ ] Use Velocity value in game
//! - [ ] Improve 60 FPS accuracy with precise timing and configuration.
//! - [ ] Add some performance log with tracing for example
//! - [ ] Some tests example
//! - [ ] Fix too much life display outside of screen
//! - [ ] Some performance improvement running [flamegraph](https://github.com/flamegraph-rs/flamegraph)
//!
//!
//! ## References
//! - Clippy lints: <https://github.com/rust-lang/rust-clippy/>
//! - Ratatui tutorial: <https://ratatui.rs/tutorials/hello-world/>
//! - Example: <https://ratatui.rs/examples/widgets/canvas/>
//!
//! ## Architecture
//! - Uses `RwLock` for synchronization.
//! - Spawns separate threads for input handling, rendering (60Hz), and game logic execution.
//!
//! ## Documentation generation
//! - `cargo doc --document-private-items --no-deps --open`
//!
//! ## Tests
//!  - As usual run them with `cargo test`. th eproject is set up with a lib containing co,ntaining all the code, and a main rs just calling it
//!  - as this very common pattern provide full compliance with rust test ecosystem, allowing doc comment to be automatically test for example.

mod cli;
pub mod controls;
pub mod game;
pub mod graphics;

use crate::game::Game as Jeu;
use clap::Parser;
use controls::speed::Speed;

use crate::graphics::graphic_block::Position;
use crate::graphics::snake_body::SnakeBody;
use cli::Cli;
use graphics::map::Map as Carte;

pub fn start_snake() {
    let args = Cli::parse();

    let case_size = 2;
    let velocity = args.velocity;
    let uncaps_fps = args.uncaps_fps;
    let classic = args.classic;

    let ini_position = Position { x: 50, y: 5 };
    let mut terminal = ratatui::init();
    let map: Carte = Carte::new(case_size, terminal.get_frame().area());
    let speed: Speed = Speed::new(&velocity);

    let serpent: SnakeBody = SnakeBody::new(
        &args.body_symbol,
        &args.head_symbol,
        args.snake_length,
        ini_position,
        case_size,
    );
    let mut jeu: Jeu = game::Game::new(
        (classic, uncaps_fps, args.life, args.nb_of_fruit),
        speed,
        serpent,
        map,
        terminal,
    );

    jeu.start();
    ratatui::restore();
}
