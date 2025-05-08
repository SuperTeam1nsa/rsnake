#![forbid(unsafe_code)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
// Documentation for all Clippy lints: https://github.com/rust-lang/rust-clippy/
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
//! - [ ] Add some performance log with tracing for example
//! - [ ] Fix too much life display outside of screen
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
//!  - As usual run them with `cargo test` the project is set up with a lib containing all the code, and a main.rs just calling it
//!  - As this is a widespread pattern providing full compliance with the Rust test ecosystem, allowing doc comment to be automatically tested, for example.

pub mod controls;
pub mod game_logic;
pub mod graphics;

use crate::game_logic::playing_thread_manager::Game;
use crate::graphics::graphic_block::Position;
use crate::graphics::sprites::snake_body::SnakeBody;
use clap::Parser;
use controls::cli::Cli;
use controls::speed::Speed;
use graphics::sprites::map::Map as Carte;
use ratatui::text::Span;
use std::cmp::max;

const INI_POSITION: Position = Position { x: 50, y: 5 };
/// # Panics
/// If bad characters (invalid size) are provided for snake body or head
pub fn start_snake() {
    // get command line options and parsed them to check for errors
    let args = Cli::parse();
    // If everything is OK, inits terminal for rendering
    let mut terminal = ratatui::init();
    //ratatui using UnicodeWidthStr crates as dep
    // get the correct case size for display
    let case_size = u16::try_from(max(
        Span::raw(&args.body_symbol).width(),
        Span::raw(&args.head_symbol).width(),
    ))
    .expect("Bad symbol size, use a real character");
    //except if gamer want to quit from menu screen, we continue
    // set up parameters from option parsing
    let velocity = args.velocity;
    let uncaps_fps = args.uncaps_fps;
    let classic = args.classic;
    let map: Carte = Carte::new(case_size, terminal.get_frame().area());
    let speed: Speed = Speed::new(&velocity);

    let serpent: SnakeBody = SnakeBody::new(
        &args.body_symbol,
        &args.head_symbol,
        args.snake_length,
        INI_POSITION,
        case_size,
    );
    // Display greeting screen
    let mut jeu = Game::new(
        (classic, uncaps_fps, args.life, args.nb_of_fruit),
        speed,
        serpent,
        map,
        terminal,
    );
    jeu.menu();

    //in all cases, restore
    ratatui::restore();
}
