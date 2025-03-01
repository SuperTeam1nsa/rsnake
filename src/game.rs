use crate::map::Map;
use crate::snake::direction::Direction;
use crate::snake::snake::Snake;
use crate::snake::speed::Speed;
use crate::utils::greeting;
use crossterm::event;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::Rect;
use ratatui::text::Span;
use ratatui::widgets::Paragraph;
use ratatui::DefaultTerminal;
use std::sync::{Arc, RwLock};
use std::thread::sleep;
use std::time::Duration;
use std::{io, thread};

const QUIT_KEYS: [KeyCode; 2] = [KeyCode::Char('q'), KeyCode::Char('Q')];
/// Displays a blinking "Game Over" message using ratatui.
pub fn game_over_blinking(terminal: &mut DefaultTerminal) -> io::Result<()> {
    use ratatui::{
        layout::{Alignment, Rect},
        style::{Color, Style},
        text::{Span, Text},
        widgets::Paragraph,
    };
    use std::{thread, time::Duration};

    let game_over_msg = "GAME OVER";
    let blink_duration = Duration::from_millis(500);
    let max_blinks = 10;

    for i in 0..max_blinks {
        let is_visible = i % 2 == 0;
        terminal.draw(|frame| {
            let text = if is_visible {
                Text::styled(
                    game_over_msg,
                    Style::default()
                        .fg(Color::Red)
                        .add_modifier(ratatui::style::Modifier::BOLD),
                )
            } else {
                Text::raw("") // Keep it empty for blinking effect
            };

            let block = Paragraph::new(text)
                .alignment(Alignment::Center)
                .block(ratatui::widgets::Block::default());

            // Center the paragraph on the terminal
            let area = Rect {
                x: frame.size().width / 4,
                y: frame.size().height / 3,
                width: frame.size().width / 2,
                height: 3,
            };

            frame.render_widget(block, area);
        })?;

        thread::sleep(blink_duration);
    }

    Ok(())
}

pub struct Game {
    speed: Speed,
    serpent: Arc<RwLock<Snake<'static>>>,
    direction: Arc<RwLock<Direction>>,
    //NB: if does not want to clone later, use only Arc<Map> (immuable)
    carte: Map<'static>,
    life: Arc<RwLock<u16>>,
    score: Arc<RwLock<u32>>,
    terminal: DefaultTerminal,
}
impl Game {
    pub fn new(
        speed: Speed,
        serpent: Snake<'static>,
        carte: Map<'static>,
        life: u16,
        terminal: DefaultTerminal,
    ) -> Game {
        Game {
            speed,
            serpent: Arc::new(RwLock::new(serpent)),
            direction: Arc::new(RwLock::new(Direction::Left)),
            carte,
            life: Arc::new(RwLock::new(life)),
            score: Arc::new(RwLock::new(0)),
            terminal,
        }
    }
    pub fn run(&mut self) -> io::Result<()> {
        loop {
            //if self.serpent.read().unwrap().is_alive {
            //for text display: https://ratatui.rs/recipes/render/display-text/
            self.terminal
                .draw(|frame| {
                    //maps
                    frame.render_widget(self.carte.get_widget(), *self.carte.area());
                    //score
                    frame.render_widget(
                        Paragraph::new(format!(" Score: {} ", *self.score.read().unwrap())),
                        Rect::new(20, 0, 15, 1),
                    );
                    //life

                    let life = *self.life.read().unwrap() as usize;
                    frame.render_widget(
                        Paragraph::new(format!(" life: {} ", "❤️ ".repeat(life))),
                        Rect::new(40, 0, u16::try_from(7 * life).unwrap_or(21), 1),
                    );
                    //serpent //circle bad on not squared terminal => use emoji with position
                    frame.render_widget(self.serpent.read().unwrap().get_widget(), frame.area());
                })
                .expect("TODO: panic message");
            /*} else {
                game_over_blinking(&mut self.terminal).expect("TODO: panic message");
                sleep(Duration::from_millis(100));
            }*/
            //sleep(Duration::from_millis(10));
        }
    }
    pub fn greeting(&mut self) {
        greeting(&mut self.terminal);
        let mut rules_understood = false;
        while !rules_understood {
            if let event::Event::Key(key) = event::read().expect("Error reading key event") {
                if key.kind == KeyEventKind::Press {
                    if key.code == KeyCode::Char('q') || key.code == KeyCode::Char('Q') {
                        return;
                    }
                    if key.code == KeyCode::Left
                        || key.code == KeyCode::Right
                        || key.code == KeyCode::Down
                        || key.code == KeyCode::Up
                    {
                        rules_understood = true;
                    }
                }
            }
        }
    }
    pub fn start(&mut self) {
        self.greeting();

        // be carreful not all thread on snake or deadlock => put a max of variable out of snake
        // Prepare thread use of variable
        let thread_snake = Arc::clone(&self.serpent);
        let game_logic_snake = Arc::clone(&self.serpent);
        let game_life = Arc::clone(&self.life);
        let game_score = Arc::clone(&self.score);
        let game_direction_logic = Arc::clone(&self.direction);
        let game_direction_input = Arc::clone(&self.direction);
        //if we want to have a variable speed put it under an Arc
        let game_speed = self.speed.get_speed();
        let carte = self.carte.clone();
        // game logic
        thread::spawn(move || loop {
            match game_logic_snake
                .write()
                .unwrap()
                .ramp(&game_direction_logic.read().unwrap())
            {
                Ok((x, y)) => {
                    if (carte.out_of_map(x, y)) {
                        *game_life.write().unwrap() -= 1;
                    }
                }
                Err(()) => {
                    *game_life.write().unwrap() -= 1;
                }
            }

            sleep(Duration::from_millis(game_speed));
        });

        // input logic
        thread::spawn(move || loop {
            if let Ok(event::Event::Key(key)) = event::read() {
                handle_key_event(key, &game_direction_input);
            }
        });

        // Graphical thread
        if let Err(_e) = self.run() {}
    }
}
// Gestion des événements clavier extraite dans une fonction
pub fn handle_key_event(key: KeyEvent, game_direction: &Arc<RwLock<Direction>>) {
    if key.kind == KeyEventKind::Press {
        if QUIT_KEYS.contains(&key.code) {
            // Logique pour quitter le jeu (à ajouter)
            //snake.write().unwrap().is_alive = false;
        } else {
            let direction_input = match key.code {
                KeyCode::Left => Some(Direction::Left),
                KeyCode::Right => Some(Direction::Right),
                KeyCode::Down => Some(Direction::Down),
                KeyCode::Up => Some(Direction::Up),
                _ => None,
            };

            if let Some(dir) = direction_input {
                *game_direction.write().unwrap() = dir;
            }
        }
    }
}
