#![deny(clippy::all)]
#![deny(clippy::pedantic)]

mod game;
mod map;
mod snake;
mod utils;

// inspi: https://ratatui.rs/tutorials/hello-world/
use crate::game::Game as Jeu;

use crate::map::Map as Carte;
use crate::snake::Snake;
use crate::snake::{Direction, PredDefinedSpeed, Speed};

fn main() {
    //exemple: https://ratatui.rs/examples/widgets/canvas/
    /*
               ball: Circle {
               x: 20.0,
               y: 40.0,
               radius: 10.0,
               color: Color::Yellow,
           }
               let block = Block::bordered()
        .border_type(border_type)
        .title(format!("BorderType::{border_type:#?}"));
    frame.render_widget(paragraph.clone().block(block), area);
    */
    //can capture mouse in terminal
    //use block for map : https://ratatui.rs/examples/widgets/block/
    let case_size = 2;
    let terminal = ratatui::init();
    let map: Carte = Carte::new(180, 17, case_size);
    let speed: Speed = Speed::new(PredDefinedSpeed::Normal, 10f32);
    let serpent: Snake = Snake::new((50, 5), Direction::Left, speed, case_size, "❄️", 10);
    let mut jeu: Jeu = game::Game::new(serpent, map, 3, terminal);
    jeu.start();
    ratatui::restore();
}
