pub use crate::controls::direction::Direction;
use crate::controls::input::playing_input_loop;
pub use crate::controls::speed::Speed;
pub use crate::game_logic::fruits_manager::FruitsManager;
use crate::game_logic::playing_logic::playing_logic_loop;
pub use crate::game_logic::state::{GameState, GameStatus};
use crate::graphics::map::Map;
use crate::graphics::playing_render::playing_render_loop;
use crate::graphics::snake_body::SnakeBody;
use ratatui::DefaultTerminal;
use std::sync::{Arc, RwLock};
use std::thread;

/// Represent a Game data structure
/// 'c must outlive 'b as, 'c (fruits manager) uses in intern the map with lock on it.
pub struct Game<'a, 'b, 'c: 'b> {
    /// Game without shortening snake bonus is classic
    classic_mode: bool,
    /// If set to true, render graphic as fast as possible, without limiting FPS
    uncaps_fps: bool,
    /// The game logic speed, linked to the snake movements
    speed: Speed,
    /// Represents the snake moving around
    serpent: Arc<RwLock<SnakeBody<'a>>>,
    /// The direction chosen by player for the snake
    direction: Arc<RwLock<Direction>>,
    /// The game logic map where items/snake are displayed
    /// NB: As we want resizable map ``RwLock``, otherwise use only Arc<Map> (immuable)
    carte: Arc<RwLock<Map<'b>>>,
    /// Game states and metrics (life etc.)
    state: Arc<RwLock<GameState>>,
    /// Manage fruits (popping, eaten etc.)
    fruits_manager: Arc<RwLock<FruitsManager<'c, 'b>>>,
    /// The current terminal
    terminal: DefaultTerminal,
}
impl<'a, 'b, 'c> Game<'a, 'b, 'c> {
    #[must_use]
    pub fn new(
        (classic_mode, uncaps_fps, life, fruits_nb): (bool, bool, u16, u16),
        speed: Speed,
        serpent: SnakeBody<'a>,
        carte: Map<'b>,
        terminal: DefaultTerminal,
    ) -> Game<'a, 'b, 'c> {
        let arc_carte = Arc::new(RwLock::new(carte));
        Game {
            classic_mode,
            uncaps_fps,
            speed,
            serpent: Arc::new(RwLock::new(serpent)),
            direction: Arc::new(RwLock::new(Direction::Right)),
            carte: arc_carte.clone(),
            state: Arc::new(RwLock::new(GameState::new(life))),
            fruits_manager: Arc::new(RwLock::new(FruitsManager::new(
                fruits_nb,
                arc_carte.clone(),
            ))),
            terminal,
        }
    }
    /// Start the main Game threads: input, rendering, logic
    pub fn start(&mut self) {
        // be careful not all thread on graphics or same structure and do not keep them too much
        // => performance issue otherwise
        // Prepare thread use of variable

        //For logical thread
        let logic_snake = Arc::clone(&self.serpent);
        let logic_gs = Arc::clone(&self.state);
        let logic_dir = Arc::clone(&self.direction);
        let carte = Arc::clone(&self.carte);
        let fruits_manager = Arc::clone(&self.fruits_manager);

        // For input management thread
        let input_gs = Arc::clone(&self.state);
        let input_dir = Arc::clone(&self.direction);

        //if we want to have a variable speed put it under an Arc<Rw>, constant can directly be put under an Arc
        // or share as normal variable by copy
        let game_speed = self.speed.get_speed();
        let speed_score_modifier = self.speed.get_score_modifier();
        let classic = self.classic_mode;

        //In a scope to have auto cleaning by auto join at the end of the main thread
        thread::scope(|s| {
            // Game logic thread
            s.spawn(move || {
                playing_logic_loop(
                    &logic_dir,
                    &logic_snake,
                    &logic_gs,
                    &carte,
                    &fruits_manager,
                    (game_speed, speed_score_modifier, classic),
                );
            });
            // input logic thread
            s.spawn(move || {
                playing_input_loop(&input_dir, &input_gs);
            });

            // Graphical thread (last one, reusing the main thread)
            playing_render_loop(
                &Arc::clone(&self.carte),
                &Arc::clone(&self.fruits_manager),
                &Arc::clone(&self.state),
                &Arc::clone(&self.serpent),
                self.uncaps_fps,
                self.speed.get_symbol(),
                &mut self.terminal,
            );
        });
    }
}
