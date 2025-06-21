use crate::game_logic::fruits_manager::FruitsManager;
use crate::game_logic::state::{GameState, GameStatus};
use crate::graphics::menus;
use crate::graphics::sprites::map::Map;
use crate::graphics::sprites::snake_body::SnakeBody;
use ratatui::layout::Rect;
use ratatui::widgets::Paragraph;
use ratatui::{DefaultTerminal, Frame};
use std::sync::{Arc, RwLock};
use std::thread::sleep;
use std::time::{Duration, Instant};

///Position to render elements
///will be clamped to the frame area border anyway, 9999 to go to the last line allowing easy resizing
const BOTTOM_SPEED_FPS_SCORE_RECT: Rect = Rect::new(1, 9999, 70, 1);
const LIFE_RECT: Rect = Rect::new(1, 0, 60, 1);
const NB_OF_FRAMES_WINDOW: f64 = 1_000.0;
const TOO_MUCH_LIVES_TO_DISPLAY: &str = " life: ❤️❤️❤️❤️❤️... ";
/// # Panics                                                                                              
/// if Arc panics while holding the resources (poisoning), no recovery mechanism implemented better crash  
pub fn playing_render_loop<'a: 'b, 'b>(
    carte: &Arc<RwLock<Map>>,
    fruits_manager: &Arc<RwLock<FruitsManager<'a, 'b>>>,
    state: &Arc<RwLock<GameState>>,
    serpent: &Arc<RwLock<SnakeBody>>,
    uncaps_fps: bool,
    speed_symbol: &str,
    terminal: &mut DefaultTerminal,
) {
    //better to pre-format a string than doing it each time
    let speed_text = format!("Speed: {speed_symbol}");

    //configure display variable with default value
    let mut rendering_break = false;
    let mut need_carte_resize = false;
    let mut frame_count = 0f64;
    let mut start_windows_time = Instant::now();
    let mut start_frame_time: Instant;

    //As quick as efficient as possible
    //Avoid sub functions to limit arc clone, otherwise create a display structure
    'render_loop: loop {
        // for FPS stats
        start_frame_time = Instant::now();
        frame_count += 1.0;
        //windows for frame calcul
        if frame_count >= NB_OF_FRAMES_WINDOW {
            frame_count = 1.0;
            start_windows_time = Instant::now();
        }
        // start rendering game sprites
        terminal
            .draw(|frame| {
                let area = frame.area();
                //maps
                {
                    //sub scope to release the lock faster
                    let map_guard = carte.read().unwrap();
                    let area_map = map_guard.area();
                    frame.render_widget(map_guard.get_widget(), *area_map);
                    if area.height != area_map.height || area.width != area_map.width {
                        need_carte_resize = true;
                    }
                }
                //remember: cannot unlock in the same scope twice (even less write/read),
                // so use boolean to limit the number of unlocking
                if need_carte_resize {
                    carte.write().unwrap().resize_to_terminal(area);
                    fruits_manager.write().unwrap().resize_to_terminal();
                    need_carte_resize = false;
                }
                //sub scope to release the lock faster
                {
                    //snake speed & FPS & Score
                    frame.render_widget(
                        Paragraph::new(format!(
                            "{speed_text} | FPS: {} | Score: {} ",
                            (frame_count / start_windows_time.elapsed().as_secs_f64()).floor(),
                            state.read().unwrap().score
                        )),
                        BOTTOM_SPEED_FPS_SCORE_RECT.clamp(frame.area()),
                    );
                }
                //sub scope to release the lock faster
                {
                    let state_guard = state.read().unwrap();
                    //life
                    let life = state_guard.life as usize;
                    frame.render_widget(
                        Paragraph::new(if life > 5 {
                            TOO_MUCH_LIVES_TO_DISPLAY.to_string()
                        } else {
                            format!(" life: {} ", "❤️ ".repeat(life))
                        }),
                        LIFE_RECT.clamp(frame.area()),
                    );
                }
                //Snake
                // circle bad on not squared terminal => use emoji with position
                {
                    //NB: to have lighter code,we could implement Widget on custom Type wrapper
                    //over RwLock using the NewType Pattern to overcome the Orphan Rule
                    let snake_read = serpent.read().unwrap(); // Read lock
                    frame.render_widget(&*snake_read, frame.area());
                }
                {
                    //NB: to have lighter code, we could implement Widget on custom Type wrapper
                    // over RwLock using the NewType Pattern to overcome the Orphan Rule
                    let fruits_manager_read = fruits_manager.read().unwrap(); // Read lock
                    frame.render_widget(&*fruits_manager_read, frame.area());
                }

                // And game_logic status
                rendering_break = game_state_render(&state.read().unwrap().status, frame);
            })
            .expect("bad rendering, check sprites position");
        if rendering_break {
            //let time for the user to see the farewell/menu screen
            sleep(Duration::from_millis(1000));
            //nice labeled loop :)
            break 'render_loop;
        }
        //If you want to reduce CPU usage, caps to approx 60 FPS (some ms reserved for processing rendering and measured s time elapsed)
        if !uncaps_fps {
            sleep(Duration::from_millis(16).saturating_sub(start_frame_time.elapsed()));
        }
    }
}
/// Return whether stop the rendering
fn game_state_render(state: &GameStatus, frame: &mut Frame) -> bool {
    let mut rendering_break = false;
    match state {
        GameStatus::Paused => {
            menus::status::pause_paragraph(frame);
        }
        GameStatus::GameOver => {
            menus::status::game_over_paragraph(frame);
        }
        GameStatus::ByeBye => {
            menus::status::byebye_paragraph(frame);
            rendering_break = true;
        }
        GameStatus::Playing => (),
        GameStatus::Restarting => {
            menus::status::restart_paragraph(frame);
        }
        GameStatus::Menu => {
            menus::status::menu_paragraph(frame);
            rendering_break = true;
        }
    }
    rendering_break
}
