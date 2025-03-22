#![deny(clippy::all)]
#![deny(clippy::pedantic)]
//to have all Clippy lints see https://github.com/rust-lang/rust-clippy/
mod controls;
mod game;
mod graphics;

// inspiration: https://ratatui.rs/tutorials/hello-world/
use crate::game::Game as Jeu;
use controls::speed::Speed;

use crate::controls::speed::Velocity;
use crate::graphics::graphic_block::Position;
use crate::graphics::snake_body::SnakeBody;
use graphics::map::Map as Carte;

///TODO: clap to get parameters from cmdline
/// test integrated for proba & test integration
/// cargo doc --document-private-items --no-deps --open
/// Edge case and head position management
/// Calculate 60 FPS more accurately (timer in and out)
/// Refactoring, extract 3 mains loop each in one file
/// Refactoring `case_size` (often 2 to render emoji) to `case_width` and case height(often 1 as each line)
fn main() {
    //exemple: https://ratatui.rs/examples/widgets/canvas/ (moche / moins bien que emoji)
    //can capture mouse in terminal
    //idea
    //RwLock + 1 thread (bloquant) input, 1 affichage (60Hz) , 1 déplacement élément (serpent etc) 🐍,
    // 1 game logic en principal time 2x vitesse d'affichage qui joue les actions sur les objects etc  (pourrait être mélange avec le rendering )
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
