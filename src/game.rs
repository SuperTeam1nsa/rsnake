use crate::map::Map;
use crate::snake::{Direction, Snake};
use crate::utils::greeting;
use crossterm::event;
use crossterm::event::{poll, KeyCode, KeyEventKind};
use ratatui::style::{Color, Style};
use ratatui::text::Span;
use ratatui::widgets::Paragraph;
use ratatui::{DefaultTerminal, Terminal};
use std::io;
use std::thread::sleep;
use std::time::Duration;

pub struct Game<'a> {
    serpent: Snake<'a>,
    carte: Map<'a>,
    life: u16,
    score: u32,
    terminal: DefaultTerminal,
}
impl<'a> Game<'a> {
    pub fn new(
        serpent: Snake<'a>,
        carte: Map<'a>,
        life: u16,
        terminal: DefaultTerminal,
    ) -> Game<'a> {
        Game {
            serpent,
            carte,
            life,
            score: 0,
            terminal,
        }
    }
    pub fn run(&mut self) -> io::Result<()> {
        loop {
            //must decorelate ramping avance with keycode entry
            //thread it for the serpent and graphic update, main loop only for input
            if (poll(Duration::from_millis(100)))? {
                if let event::Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        if key.code == KeyCode::Char('q') || key.code == KeyCode::Char('Q') {
                            return Ok(());
                        } else if key.code == KeyCode::Left {
                            self.serpent.direction = Direction::Left;
                        } else if key.code == KeyCode::Right {
                            self.serpent.direction = Direction::Right;
                        } else if key.code == KeyCode::Down {
                            self.serpent.direction = Direction::Down;
                        } else if key.code == KeyCode::Up {
                            self.serpent.direction = Direction::Up;
                        }
                    }
                }
            }
            self.serpent.ramp();
            self.terminal.draw(|frame| {
                //let span = Span::styled("❄️", Style::default().fg(Color::Cyan));
                //circle bad on not squared terminal => use emoji with position
                frame.render_widget(self.carte.get_widget(), frame.area());
                frame.render_widget(self.serpent.get_widget(), frame.area());
            })?;
            sleep(Duration::from_millis(1000));
        }
    }
    pub fn greeting(&mut self) {
        greeting(&mut self.terminal);
    }
    pub fn start(&mut self) {
        self.greeting();
        match self.run() {
            Ok(..) => (),
            Err(e) => {}
        }
    }
}
