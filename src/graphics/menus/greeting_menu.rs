use crate::controls::input::GreetingOption;
use crate::controls::speed;
use crate::controls::speed::SpeedConfig;
use crate::graphics::menus::layout_utils::frame_vertically_centered_rect;
use crate::graphics::sprites::fruit::FRUITS_SCORES_PROBABILITIES;
use clap::ValueEnum;
use ratatui::style::Stylize;
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, BorderType, Paragraph};
use ratatui::{DefaultTerminal, Frame};
use std::thread::sleep;
use std::time::Duration;

/// Print the welcome screen, reminding controls
/// Show Fruit and Speed menus alongside
/// Sadly `slow_blink` and `fast_blink` are rendered anymore on modern terminal...
/// # Panics
/// Will panic if no suitable terminal for displaying ios provided
pub fn main_greeting_menu(
    terminal: &mut DefaultTerminal,
    opt: &Option<GreetingOption>,
    selected: &GreetingOption,
) {
    //terminal.clear().expect("Clearing terminal fail ");
    terminal
        .draw(|frame| {
            let area = frame.area();
            match opt {
                None | Some(GreetingOption::MainMenu) => {
                    let lines = vec![
                        Line::from("███████╗ ███╗   ██╗  █████╗  ██╗  ██╗ ███████╗"),
                        Line::from("██╔════╝ ████╗  ██║ ██╔══██╗ ██║ ██╔╝ ██╔════╝"),
                        Line::from("███████╗ ██╔██╗ ██║ ███████║ █████╔╝  █████╗  "),
                        Line::from("╚════██║ ██║╚██╗██║ ██╔══██║ ██╔═██╗  ██╔══╝  "),
                        Line::from("███████║ ██║ ╚████║ ██║  ██║ ██║  ██╗ ███████╗"),
                        Line::from("╚══════╝ ╚══════╝╚═ ╝  ╚═══╝ ╚═╝  ╚═╝ ╚═╝  ╚═╝"),
                        Line::from(get_button_span(selected)),
                        Line::from("+--------+------+------+-------+-----+-------+"),
                        Line::from("|Controls| ←↕→  |  Q   | P / ⎵ |  M  |   R   | "),
                        Line::from("+--------+------+------+-------+-----+-------+"),
                        Line::from("|Effects | Move | Quit | Pause | Menu| Start |"),
                        Line::from("+--------+------+------+-------+-----+-------+"),
                        Line::from("Have a good 🐍 game ! 🎮".green()),
                    ];
                    let nb_lines = lines.len();

                    frame.render_widget(
                        //centered horizontally
                        Paragraph::new(Text::from(lines)).centered(),
                        frame_vertically_centered_rect(area, nb_lines),
                    );
                }
                Some(GreetingOption::Fruits) => {
                    fruit_menu(frame, selected);
                }
                Some(GreetingOption::Velocity) => {
                    speed_menu(frame, selected);
                }
                Some(GreetingOption::StartGame | GreetingOption::QuitGame) => {
                    //Log Game is started/quit :p
                }
                Some(GreetingOption::Parameters) => {
                    parameters_menu(frame, selected);
                }
                Some(GreetingOption::Help) => {
                    help_menu(frame, selected);
                }
                Some(GreetingOption::Enter | GreetingOption::Next | GreetingOption::Previous) => {
                    let lines = vec![
                        Line::from(
                            "You find an easter egg here! Not yet playable, please report\
                          Guess some edge cases are possible :p ",
                        ),
                        Line::from(get_button_span(selected)),
                    ];
                    let nb_lines = lines.len();
                    frame.render_widget(
                        Paragraph::new(Text::from(lines)).centered(),
                        frame_vertically_centered_rect(area, nb_lines),
                    );
                }
            }
            //buttons_menu(frame, &app);
            //set a border all around the terminal
            frame.render_widget(Block::bordered().border_type(BorderType::Double), area);
        })
        .expect("Unusable terminal render");
    sleep(Duration::from_millis(100));
}

/// Display the fruit menu, vertically centered
fn fruit_menu(frame: &mut Frame, selected: &GreetingOption) {
    //adding fruits rules, gonna left aligned for less screen space use
    let mut fruits_lines = Vec::new();
    let tab_jonction = Line::from("+---------+---------------------+--------+-------------+");
    fruits_lines.push(tab_jonction.clone());
    fruits_lines.push(Line::from(
        "| Fruit   | Base Score (xSpeed) | Chance | Size Effect |",
    ));
    fruits_lines.push(tab_jonction.clone());
    for (fruit, score, probability, size_effect) in FRUITS_SCORES_PROBABILITIES {
        //:<6 and so on are formating options, e.g., saying aligning left with min 6 chars
        fruits_lines.push(Line::from(format!(
            "| {fruit:<6} | {score:>19} | {probability:>5}% | {size_effect:>11} |"
        )));
    }
    fruits_lines.push(tab_jonction);
    fruits_lines.push(Line::from(get_button_span(selected)));
    let nb_lines = fruits_lines.len();
    //more idiomatic using lines (as in: https://ratatui.rs/recipes/render/display-text/)
    //can mix style inside the same line using Line::from(vec!["hello".green(),
    // " ".into(), "world".green().bold(), "3".into()]),
    frame.render_widget(
        Paragraph::new(Text::from(fruits_lines)).centered(),
        frame_vertically_centered_rect(frame.area(), nb_lines),
    );
}
/// Display the speed menu center aligned, vertically centered
fn speed_menu(frame: &mut Frame, selected: &GreetingOption) {
    // Speed effects
    let mut speed_lines = Vec::new();
    let speed_tab_jonction = Line::from("+------------+------------+----------------+---------+ ");
    speed_lines.push(speed_tab_jonction.clone());
    speed_lines.push(Line::from(
        "| Speed Name | Value (ms) | Score Modifier | Symbol  | ",
    ));
    speed_lines.push(speed_tab_jonction.clone());
    for s in speed::Speed::value_variants() {
        let SpeedConfig {
            name,
            ms_value,
            score_modifier,
            symbol,
        } = s.config();
        //:<10 and so on are formating options, e.g., saying aligning left with min 10 chars
        speed_lines.push(Line::from(format!(
            "| {name:<10} | {ms_value:>10} | {score_modifier:>14} | {symbol:<6} | "
        )));
    }
    speed_lines.push(speed_tab_jonction);
    speed_lines.push(Line::from(get_button_span(selected)));
    let nb_lines = speed_lines.len();
    frame.render_widget(
        Paragraph::new(Text::from(speed_lines)).centered(),
        frame_vertically_centered_rect(frame.area(), nb_lines),
    );
}
/// Display the parameters menu center aligned, vertically centered
fn parameters_menu(frame: &mut Frame, selected: &GreetingOption) {
    let lines = vec![
        Line::from("Parameters are not yet implemented 😜"),
        Line::from("For now use the command line parameter or load a parameters file"),
        Line::from("Upvote this issue for faster release or bring me a coffee on kofi🥤 "),
        Line::from(get_button_span(selected)),
    ];
    let nb_lines = lines.len();
    frame.render_widget(
        Paragraph::new(Text::from(lines)).centered(),
        frame_vertically_centered_rect(frame.area(), nb_lines),
    );
}
fn help_menu(frame: &mut Frame, selected: &GreetingOption) {
    //formating reminder: Where:
    // - `<10` means left-aligned with width 10
    // - `>5` means right-aligned with width 5
    let lines = vec![
        Line::from("Snake Game Rules:".bold().yellow()),
        Line::from(format!("• {:<80}", "Eat fruits to score points")),
        Line::from(format!(
            "    • {:<80}",
            "Different fruits give various scores / effects, some even reduce size (and score)"
        )),
        Line::from(format!(
            "   • {:<80}",
            "Game speeds can be changed to increase difficulty and score multipliers"
        )),
        Line::from(format!(
            "   • {:<80}",
            "Check the Fruit and Speed menus for more details on game mechanics!"
        )),
        Line::from(format!(
            "• {:<80}",
            "Control the snake using the arrow keys (←↕→)"
        )),
        Line::from(format!(
            "   • {:<80}",
            "Avoid hitting yourself or your own tail"
        )),
        Line::from(format!(
            "   • {:<80}",
            "Walls are circulars so your head will appear on the other side of the screen"
        )),
        Line::from(format!("• {:<80}", "Press P or Space to pause the game")),
        Line::from(format!("• {:<80}", "Press Q to quit anytime")),
        Line::from(format!("• {:<80}", "Press R to start a new game")),
        Line::from(format!(
            "• {:<80}",
            "Press M to return to the menu, and maybe consider support this game on kofi🥤"
        )),
        Line::from(get_button_span(selected)),
    ];

    let nb_lines = lines.len();
    frame.render_widget(
        Paragraph::new(Text::from(lines)).centered(),
        frame_vertically_centered_rect(frame.area(), nb_lines),
    );
}

fn get_button_span(selected: &GreetingOption) -> Vec<Span> {
    let selected_main: Vec<Span> = vec![" [ ".red(), "Main".into(), " ] ".red().bold()];
    let default_main: Vec<Span> = vec![" [".into(), "M".yellow(), "ain]".into()];
    let selected_fruit: Vec<Span> = vec![" [ ".red(), "Fruit".into(), " ] ".red().bold()];
    let default_fruit: Vec<Span> = vec![" [".into(), "F".yellow(), "ruit]".into()];
    let selected_velocity: Vec<Span> = vec![" [ ".red(), "Speed".into(), " ] ".red().bold()];
    let default_velocity: Vec<Span> = vec![" [".into(), "S".yellow(), "peed]".into()];
    let selected_start: Vec<Span> = vec![" [ ".red(), "Run".into(), " ] ".red().bold()];
    let default_start: Vec<Span> = vec![" [".into(), "R".yellow(), "un]".into()];
    let selected_setup: Vec<Span> = vec![" [ ".red(), "Edit⚙️".into(), " ] ".red().bold()];
    let default_setup: Vec<Span> = vec![" [".into(), "E".yellow(), "dit⚙️]".into()];
    let selected_help: Vec<Span> = vec![" [ ".red(), "Help".into(), " ] ".red().bold()];
    let default_help: Vec<Span> = vec![" [".into(), "H".yellow(), "elp]".into()];
    let mut vec_line_button = vec!["↔".into()];
    if selected == &GreetingOption::MainMenu {
        vec_line_button.extend(selected_main);
    } else {
        vec_line_button.extend(default_main);
    }
    if selected == &GreetingOption::Fruits {
        vec_line_button.extend(selected_fruit);
    } else {
        vec_line_button.extend(default_fruit);
    }
    if selected == &GreetingOption::Velocity {
        vec_line_button.extend(selected_velocity);
    } else {
        vec_line_button.extend(default_velocity);
    }
    if selected == &GreetingOption::StartGame {
        vec_line_button.extend(selected_start);
    } else {
        vec_line_button.extend(default_start);
    }
    if selected == &GreetingOption::Parameters {
        vec_line_button.extend(selected_setup);
    } else {
        vec_line_button.extend(default_setup);
    }
    if selected == &GreetingOption::Help {
        vec_line_button.extend(selected_help);
    } else {
        vec_line_button.extend(default_help);
    }
    vec_line_button
}
