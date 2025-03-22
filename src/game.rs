use crate::controls::direction::Direction;
use crate::controls::fruits_manager::FruitsManager;
use crate::controls::game_state::{GameState, GameStatus};
use crate::controls::speed::Speed;
use crate::graphics::map::Map;
use crate::graphics::snake_body::SnakeBody;
use crate::graphics::utils;
use crate::graphics::utils::greeting;
use crossterm::event;
use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::layout::Rect;
use ratatui::widgets::Paragraph;
use ratatui::DefaultTerminal;
use std::sync::{Arc, RwLock};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

const QUIT_KEYS: [KeyCode; 2] = [KeyCode::Char('q'), KeyCode::Char('Q')];
const PAUSE_KEYS: [KeyCode; 3] = [KeyCode::Char('p'), KeyCode::Char('P'), KeyCode::Char(' ')];
const RESET_KEYS: [KeyCode; 2] = [KeyCode::Char('r'), KeyCode::Char('R')];
pub struct Game<'a, 'b, 'c> {
    speed: Speed,
    serpent: Arc<RwLock<SnakeBody<'a>>>,
    direction: Arc<RwLock<Direction>>,
    //NB: if does not want to clone later, use only Arc<Map> (immuable)
    carte: Arc<Map<'b>>,
    // states and game metrics (life etc.)
    state: Arc<RwLock<GameState>>,
    //Fruits
    fruits_manager: Arc<RwLock<FruitsManager<'c, 'b>>>,
    terminal: DefaultTerminal,
}

impl<'a, 'b, 'c> Game<'a, 'b, 'c> {
    pub fn new(
        speed: Speed,
        serpent: SnakeBody<'a>,
        carte: Map<'b>,
        life: u16,
        fruits_nb: u16,
        terminal: DefaultTerminal,
    ) -> Game<'a, 'b, 'c> {
        let arc_carte = Arc::new(carte);
        Game {
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
    pub fn render(&mut self) {
        let mut rendering_break = false;
        let mut frame_count = 0f64;
        let mut start_time = std::time::Instant::now();
        'render_loop: loop {
            frame_count += 1.0;
            //windows for frame calcul
            if frame_count >= 1_000.0 {
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
                    //FPS
                    frame.render_widget(
                        Paragraph::new(format!(
                            " Mean FPS: {} ",
                            (frame_count / start_time.elapsed().as_secs_f64()).floor()
                        )),
                        Rect::new(120, 0, 25, 1),
                    );
                    //sub scope to release the lock faster
                    {
                        let state_guard = self.state.read().unwrap();
                        //score
                        frame.render_widget(
                            Paragraph::new(format!(" Score: {} ", state_guard.score)),
                            Rect::new(20, 0, 15, 1),
                        );
                        //life
                        let life = state_guard.life as usize;
                        frame.render_widget(
                            Paragraph::new(format!(" life: {} ", "❤️ ".repeat(life))),
                            #[allow(clippy::cast_possible_truncation)]
                            Rect::new(40, 0, (10 * life) as u16, 1),
                        );
                    }
                    //serpent //circle bad on not squared terminal => use emoji with position
                    {
                        //NB: to have lighter code we could implement Widget on custom Type wrapper over RwLock using the NewType Pattern to overcome the Orphan Rule
                        let snake_read = self.serpent.read().unwrap(); // Read lock
                        frame.render_widget(&*snake_read, frame.area());
                    }
                    {
                        //NB: to have lighter code we could implement Widget on custom Type wrapper over RwLock using the NewType Pattern to overcome the Orphan Rule
                        let fruits_manager_read = self.fruits_manager.read().unwrap(); // Read lock
                        frame.render_widget(&*fruits_manager_read, frame.area());
                    }
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
            //thread::sleep(Duration::from_millis(12));
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
        //except if gamer want to quit from menu screen we continue
        if !self.greeting() {
            return;
        }
        // be careful not all thread on graphics or same structure and do not keep them too much => deadlock otherwise
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

        //if we want to have a variable speed put it under an Arc<Rw>
        let game_speed = self.speed.get_speed();

        //In a scope to have auto cleaning by auto join at end of main thread
        thread::scope(|s| {
            // Game logic thread
            s.spawn(move || {
                logic_loop(
                    &logic_dir,
                    &logic_snake,
                    &logic_gs,
                    &carte,
                    &fruits_manager,
                    game_speed,
                );
            });
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
    fruits_manager: &Arc<RwLock<FruitsManager>>,
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
                let mut write_guard = snake.write().unwrap();
                let movement = write_guard.ramp(&direction.read().unwrap(), carte);
                if let Ok(position) = movement {
                    //In two steps to leverage the power of multiple read in // whereas we have only one write
                    let fruits = fruits_manager.read().unwrap().eat_some_fruits(position);
                    if let Some(fruits) = fruits {
                        let score_fruits = fruits
                            .iter()
                            .map(super::graphics::fruit::Fruit::get_score)
                            .sum::<i32>();
                        let size_effect = fruits
                            .iter()
                            .map(super::graphics::fruit::Fruit::get_size_effect)
                            .sum::<i16>();
                        if score_fruits >= 0 {
                            write_guard.relative_size_change(size_effect);
                        }
                        gs.write().unwrap().score += score_fruits;
                        fruits_manager.write().unwrap().replace_fruits(&fruits);
                    }
                    //did you find out a fruit ?
                    // FruitManager:Inspired by BodySnake, for managing fruits
                } else {
                    //Ouch. You bite yourself
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
                //let some time for the restarting screen to appear
                sleep(Duration::from_millis(1000));
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
