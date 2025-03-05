#![deny(clippy::all)]
#![deny(clippy::pedantic)]

mod game;
mod game_state;
mod map;
mod snake;
mod utils;

use snake::speed::{Speed, Velocity};
// inspi: https://ratatui.rs/tutorials/hello-world/
use crate::game::Game as Jeu;

use crate::map::Map as Carte;
use crate::snake::snake_body::SnakeBody;

fn main() {
    //exemple: https://ratatui.rs/examples/widgets/canvas/ (moche / moins bien que emoji)
    //can capture mouse in terminal
    //idea
    //RwLock + 1 thread (bloquant) input, 1 affichage (60Hz) , 1 déplacement élément (serpent etc) 🐍,
    // 1 game logic en principal time 2x vitesse d'affichage qui joue les actions sur les objects etc  (pourrait être mélange avec le rendering )
    let case_size = 2;
    let mut terminal = ratatui::init();
    let map: Carte = Carte::new(case_size, terminal.get_frame().area());
    let speed: Speed = Speed::new(Velocity::Normal, 10);
    //if refacto: builder pattern possible (here we create the snake only once)
    let serpent: SnakeBody = SnakeBody::new("❄️", "🎄", 3, 50, 5, case_size);
    let mut jeu: Jeu = game::Game::new(speed, serpent, map, 3, terminal);
    jeu.start();
    ratatui::restore();
}
