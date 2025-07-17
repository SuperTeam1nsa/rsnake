use crate::controls::direction::Direction;
use crate::controls::input::GreetingOption::StartGame;
use crate::controls::input::{greeting_screen_manage_input, GreetingOption};
use crate::game_logic::fruits_manager::FruitsManager;
use crate::game_logic::state::{GameState, GameStatus};
use crate::graphics::menus::greeting_menu::main_greeting_menu;
use crate::graphics::sprites::fruit::Fruit;
use crate::graphics::sprites::map::Map;
use crate::graphics::sprites::snake_body::SnakeBody;
use ratatui::DefaultTerminal;
use std::sync::{Arc, RwLock};
use std::thread::sleep;
use std::time::Duration;

/// # Panics
/// if Arc panic while holding the resources (poisoning), no recovery mechanism implemented better crash  
pub fn playing_logic_loop(
    direction: &Arc<RwLock<Direction>>,
    snake: &Arc<RwLock<SnakeBody>>,
    gs: &Arc<RwLock<GameState>>,
    carte: &Arc<RwLock<Map>>,
    fruits_manager: &Arc<RwLock<FruitsManager>>,
    (game_speed, speed_score_modifier, classic_mode): (u64, u16, bool),
) {
    let mut gsc;
    loop {
        //do not want to keep the lock for too long + cannot hold in the same thread 2 times the same hold
        // so match a clone, or use a let
        gsc = gs.read().unwrap().status.clone();
        //dead snakes tell no tales, nor move :p
        match gsc {
            GameStatus::Playing => {
                let mut write_guard = snake.write().unwrap();
                //Move the snake!
                let position = write_guard.ramp(&direction.read().unwrap(), &carte.read().unwrap());
                // Check if we eat a fruit
                let fruits = fruits_manager.read().unwrap().eat_some_fruits(position);
                // fruits effects
                if let Some(fruits) = fruits {
                    let score_fruits = fruits.iter().map(Fruit::get_score).sum::<i32>();
                    let size_effect = fruits.iter().map(Fruit::get_grow_snake).sum::<i16>();
                    // in all cases except classic mode with negative size, we always apply size modifiers
                    if !(classic_mode && size_effect <= 0) {
                        write_guard.relative_size_change(size_effect);
                    }
                    //NB:Converting an u16 to an i32 is always safe in Rust because the range of u16 (0 to 65,535)
                    // fits entirely within the range of i32 (−2,147,483,648 to 2,147,483,647).
                    //So no need to do: speed_score_modifier.try_into().expect("too much")/match for conversion
                    gs.write().unwrap().score += score_fruits * i32::from(speed_score_modifier);
                    fruits_manager.write().unwrap().replace_fruits(&fruits);
                }
                // Check if the gamer will lose one life
                if write_guard.is_snake_eating_itself() {
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
                *direction.write().unwrap() = Direction::Right;
                fruits_manager.write().unwrap().reset();
                //graphical resize on rendering part (not really a game_logic constant)
            }
            GameStatus::ByeBye | GameStatus::Menu => break,
            GameStatus::Paused | GameStatus::GameOver => {}
        }
        sleep(Duration::from_millis(game_speed));
    }
}
/// Control part of the main menu
/// allows to switch to sub menu (Fruits, Speed, Parameters etc.)
///
/// # Return true if the player want to play, false otherwise
///
/// # Panics                                                                                              
/// if Terminal writing is not possible
pub fn controls_greeting_screen(terminal: &mut DefaultTerminal) -> bool {
    let mut stay = true;
    let mut action: Option<GreetingOption> = None;
    main_greeting_menu(terminal, &None, &StartGame);
    // To manage keys to switch selected item
    //begin with start
    let mut selected = 3;
    let selection: [GreetingOption; 6] = [
        GreetingOption::MainMenu,
        GreetingOption::Fruits,
        GreetingOption::Velocity,
        GreetingOption::StartGame,
        GreetingOption::Parameters,
        GreetingOption::Help,
    ];
    while stay {
        let current = greeting_screen_manage_input();
        //to keep refreshing old menu otherwise and not come backing
        if current.is_some() {
            // if it is a key, manage the selected menu button change
            if current == Some(GreetingOption::Next) {
                selected = (selected + 1) % selection.len();
            } else if current == Some(GreetingOption::Previous) {
                //smart trick to loop over
                selected = (selected + selection.len() - 1) % selection.len();
            }
            // if enter set the selected option
            else if current == Some(GreetingOption::Enter) {
                action = Some(selection[selected].clone());
            } else {
                action = current;
            }
        }
        //display the menu
        main_greeting_menu(terminal, &action, &selection[selected]);
        if let Some(GreetingOption::StartGame | GreetingOption::QuitGame) = action {
            stay = false;
        }
    }
    action == Some(GreetingOption::StartGame)
}
