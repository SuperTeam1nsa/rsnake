use crate::game_state::{GameState, GameStatus};
use crate::map::Map;
use crate::snake::direction::Direction;
use crate::snake::snake_body::SnakeBody;
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
    serpent: Arc<RwLock<SnakeBody<'static>>>,
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
        serpent: SnakeBody<'static>,
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
        let mut frame_count = 0f64;
        let mut start_time = std::time::Instant::now();
        'render_loop: loop {
            frame_count += 1.0;
            //windows for frame calcul
            if (frame_count >= 1_000.0) {
                frame_count = 1.0;
                start_time = std::time::Instant::now();
            }
            //
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
                    frame.render_widget(
                        Paragraph::new(format!(
                            " Mean FPS: {} ",
                            (frame_count / start_time.elapsed().as_secs_f64()).floor()
                        )),
                        Rect::new(120, 0, 25, 1),
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
                        GameStatus::Restarting => {
                            frame.render_widget(utils::restart_paragraph(), frame.area());
                        }
                    }
                })
                .expect("bad rendering, check sprites position");
            if rendering_break {
                //let time for user to see the farewell screen
                sleep(Duration::from_millis(1000));
                //nice labeled loop :)
                break 'render_loop;
            }
            //If you want to reduce CPU usage, caps to approx 60 FPS (some ms reserved for processing rendering)
            thread::sleep(Duration::from_millis(12));
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
            s.spawn(move || logic_loop(&logic_dir, &logic_snake, &logic_gs, &carte, game_speed));
            // input logic thread
            s.spawn(move || {
                input_loop(&input_dir, &input_gs);
            });
            // Graphical thread (current so no shared variable)
            self.render();
        });
    }
}
pub fn input_loop(direction: &Arc<RwLock<Direction>>, gs: &Arc<RwLock<GameState>>) {
    loop {
        if let Ok(event::Event::Key(key)) = event::read() {
            if key.kind == KeyEventKind::Press {
                //PAUSE
                if PAUSE_KEYS.contains(&key.code) {
                    let mut gs_guard = gs.write().unwrap();
                    //in others state does nothing to not break game logic
                    if gs_guard.status == GameStatus::Playing {
                        gs_guard.status = GameStatus::Paused;
                    } else if gs_guard.status == GameStatus::Paused {
                        gs_guard.status = GameStatus::Playing;
                    }
                //QUIT
                } else if QUIT_KEYS.contains(&key.code) {
                    gs.write().unwrap().status = GameStatus::ByeBye;
                    break;
                //RESTART
                } else if RESET_KEYS.contains(&key.code) {
                    gs.write().unwrap().status = GameStatus::Restarting;
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
pub fn logic_loop(
    direction: &Arc<RwLock<Direction>>,
    snake: &Arc<RwLock<SnakeBody>>,
    gs: &Arc<RwLock<GameState>>,
    carte: &Arc<Map>,
    game_speed: u64,
) {
    let mut gsc;
    loop {
        //do not want to keep the lock for too long + cannot hold in same thread 2 time the same hold
        // so match a clone, or use a let
        gsc = gs.read().unwrap().status.clone();
        //dead snakes tell no tales, nor move :p
        match gsc {
            GameStatus::Playing => {
                //Check if we have move without biting ourselves (Err), and getting head position after the move
                if let Ok((x, y)) = snake
                    .write()
                    .unwrap()
                    .ramp(&direction.read().unwrap(), carte)
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
            }
            GameStatus::Restarting => {
                //let some time for restarting screen to appear
                thread::sleep(Duration::from_millis(1000));
                gs.write().unwrap().reset();
                snake.write().unwrap().reset();
                *direction.write().unwrap() = Direction::Left;
            }
            GameStatus::ByeBye => break,
            _ => {}
        }
        sleep(Duration::from_millis(game_speed));
    }
}
