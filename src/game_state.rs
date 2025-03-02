use std::sync::{Arc, RwLock};

// Regroupement des drapeaux (game state flags) dans une struct
pub struct GameState {
    pub life: u16,
    pub score: u32,
    pub status: GameStatus,
    life_ini: u16,
}
#[derive(Debug, PartialEq)]
pub enum GameStatus {
    Paused,
    GameOver,
    ByeBye,
    Playing,
}

impl GameState {
    // Constructeur pratique pour initialiser les valeurs par défaut
    pub fn new(life: u16) -> Self {
        Self {
            life,
            score: 0,
            status: GameStatus::Playing,
            life_ini: life,
        }
    }
    pub fn reset(&mut self) {
        self.score = 0;
        self.status = GameStatus::Playing;
        self.life = self.life_ini;
    }
}
