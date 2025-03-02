use crate::game_state::{GameState, GameStatus};
use crate::map::Map;
use crate::snake::direction::Direction;
use crate::snake::snake_moving::SnakeMoving;
use crate::snake::speed::Speed;
use crate::utils;
use crate::utils::greeting;
use crossterm::event;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::Rect;
use ratatui::style::Modifier;
use ratatui::widgets::Paragraph;
use ratatui::DefaultTerminal;
use std::cmp::PartialEq;
use std::sync::{Arc, RwLock};
use std::thread::sleep;
use std::time::Duration;
use std::{io, thread};

const QUIT_KEYS: [KeyCode; 2] = [KeyCode::Char('q'), KeyCode::Char('Q')];
const PAUSE_KEYS: [KeyCode; 3] = [KeyCode::Char('p'), KeyCode::Char('P'), KeyCode::Char(' ')];
const RESET_KEYS: [KeyCode; 2] = [KeyCode::Char('r'), KeyCode::Char('R')];
pub struct Game {
    speed: Speed,
    serpent: Arc<RwLock<SnakeMoving<'static>>>,
    direction: Arc<RwLock<Direction>>,
    //NB: if does not want to clone later, use only Arc<Map> (immuable)
    carte: Arc<Map<'static>>,
    // states and game metrics (life etc.)
    state: Arc<RwLock<GameState>>,
    terminal: DefaultTerminal,
}

impl Game {
    pub fn new(
        speed: Speed,
        serpent: SnakeMoving<'static>,
        carte: Map<'static>,
        life: u16,
        terminal: DefaultTerminal,
    ) -> Game {
        Game {
            speed,
            serpent: Arc::new(RwLock::new(serpent)),
            direction: Arc::new(RwLock::new(Direction::Left)),
            carte: Arc::new(carte),
            state: Arc::new(RwLock::new(GameState::new(life))),
            terminal,
        }
    }
    pub fn render(&mut self) {
        let mut rendering_break = false;
        'render_loop: loop {
            //if self.serpent.read().unwrap().is_alive {
            //for text display: https://ratatui.rs/recipes/render/display-text/
            self.terminal
                .draw(|frame| {
                    //maps
                    frame.render_widget(self.carte.get_widget(), *self.carte.area());
                    //score
                    frame.render_widget(
                        Paragraph::new(format!(" Score: {} ", self.state.read().unwrap().score)),
                        Rect::new(20, 0, 15, 1),
                    );
                    //life
                    let life = self.state.read().unwrap().life as usize;
                    frame.render_widget(
                        Paragraph::new(format!(" life: {} ", "❤️ ".repeat(life))),
                        #[allow(clippy::cast_possible_truncation)]
                        Rect::new(40, 0, (7 * life) as u16, 1),
                    );
                    //serpent //circle bad on not squared terminal => use emoji with position
                    frame.render_widget(self.serpent.read().unwrap().get_widget(), frame.area());
                    // And game status
                    match self.state.read().unwrap().status {
                        GameStatus::Paused => {
                            frame.render_widget(utils::pause_paragraph(), frame.area());
                        }
                        GameStatus::GameOver => {
                            frame.render_widget(utils::game_over_paragraph(), frame.area());
                        }
                        GameStatus::ByeBye => {
                            frame.render_widget(utils::byebye_paragraph(), frame.area());
                            rendering_break = true;
                        }
                        GameStatus::Playing => (),
                    }
                })
                .expect("bad rendering, check sprites position");
            if rendering_break {
                sleep(Duration::from_millis(2000));
                //nice labeled loop :)
                break 'render_loop;
            }
        }
    }
    pub fn greeting(&mut self) -> bool {
        greeting(&mut self.terminal);
        let mut rules_understood = false;
        while !rules_understood {
            if let event::Event::Key(key) = event::read().expect("Error reading key event") {
                if key.kind == KeyEventKind::Press {
                    if key.code == KeyCode::Left
                        || key.code == KeyCode::Right
                        || key.code == KeyCode::Down
                        || key.code == KeyCode::Up
                    {
                        rules_understood = true;
                    } else if key.code == KeyCode::Char('q') || key.code == KeyCode::Char('Q') {
                        return false;
                    }
                }
            }
        }
        true
    }
    pub fn start(&mut self) {
        //except if gamer wanna quit from menu screen we continue
        if !self.greeting() {
            return;
        }
        // be careful not all thread on snake or same structure and do not keep them too much => deadlock otherwise
        // Prepare thread use of variable
        //For logical thread
        let logic_snake = Arc::clone(&self.serpent);
        let logic_gs = Arc::clone(&self.state);
        let logic_dir = Arc::clone(&self.direction);
        let carte = Arc::clone(&self.carte);

        // For input management thread
        let input_gs = Arc::clone(&self.state);
        let input_dir = Arc::clone(&self.direction);

        //if we want to have a variable speed put it under an Arc<Rw>
        let game_speed = self.speed.get_speed();

        //In a scope to have auto cleaning by auto join at end of main thread
        thread::scope(|s| {
            // Game logic thread
            s.spawn(move || {
                game_logic_loop(&logic_dir, &logic_snake, &logic_gs, &carte, game_speed)
            });
            // input logic thread
            s.spawn(move || {
                game_input_loop(&input_dir, &input_gs);
            });
            // Graphical thread (current so no shared variable)
            self.render();
        });
    }
}
pub fn game_input_loop(direction: &Arc<RwLock<Direction>>, gs: &Arc<RwLock<GameState>>) {
    loop {
        if let Ok(event::Event::Key(key)) = event::read() {
            if key.kind == KeyEventKind::Press {
                //PAUSE
                if PAUSE_KEYS.contains(&key.code) {
                    let mut gs_guard = gs.write().unwrap();
                    //to avoid to list all status or set a default one we use matches!
                    gs_guard.status = if matches!(gs_guard.status, GameStatus::Paused) {
                        GameStatus::Playing
                    } else {
                        GameStatus::Paused
                    };
                //QUIT
                } else if QUIT_KEYS.contains(&key.code) {
                    gs.write().unwrap().status = GameStatus::ByeBye;
                    break;
                //RESTART
                } else if RESET_KEYS.contains(&key.code) {
                    gs.write().unwrap().reset();
                //Arrow input
                } else {
                    let direction_input = match key.code {
                        KeyCode::Left => Some(Direction::Left),
                        KeyCode::Right => Some(Direction::Right),
                        KeyCode::Down => Some(Direction::Down),
                        KeyCode::Up => Some(Direction::Up),
                        _ => None,
                    };
                    if let Some(dir) = direction_input {
                        *direction.write().unwrap() = dir;
                    }
                }
            }
        }
    }
}
pub fn game_logic_loop(
    direction: &Arc<RwLock<Direction>>,
    snake: &Arc<RwLock<SnakeMoving>>,
    gs: &Arc<RwLock<GameState>>,
    carte: &Arc<Map>,
    game_speed: u64,
) {
    loop {
        //dead snakes tell no tales, nor move :p
        if gs.read().unwrap().status == GameStatus::Playing {
            //Check if we have move without biting ourselves (Err), and getting head position after the move
            if let Ok((x, y)) = snake
                .write()
                .unwrap()
                .ramp(&direction.read().unwrap(), &carte)
            {
                //did we find out a fruit ?
            } else {
                //ouch bite ourselves ! Or go out of map
                let mut state_guard = gs.write().unwrap();
                if (state_guard.life) >= 1 {
                    state_guard.life -= 1;
                }
                if state_guard.life == 0 {
                    state_guard.status = GameStatus::GameOver;
                }
            }
        } else if gs.read().unwrap().status == GameStatus::ByeBye {
            break;
        }
        sleep(Duration::from_millis(game_speed));
    }
}
