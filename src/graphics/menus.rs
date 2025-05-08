use crate::controls::speed;
use crate::controls::speed::Speed;
use crate::graphics::sprites::fruit::FRUITS_SCORES_PROBABILITIES;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, BorderType, Paragraph};
use ratatui::{DefaultTerminal, Frame};
use std::thread::sleep;
use std::time::Duration;

// For ASCII art use AI or a generator as:
// http://patorjk.com/software/taag/#p=display&f=ANSI%20Shadow&t=Pause
// For text display: https://ratatui.rs/recipes/render/display-text/
const GAME_OVER_TEXT: &str = "\n\
 ██████╗  █████╗ ███╗   ███╗███████╗       ██████╗ ██╗   ██╗███████╗██████╗ \n\
██╔═══   ██╔══██╗████╗ ████║██╔════╝      ██╔═══██╗██║   ██║██╔════╝██╔══██╗\n\
██║ ████║███████║██╔████╔██║█████╗        ██║   ██║██║   ██║█████╗  ██████╔╝\n\
██║   ██║██╔══██║██║╚██╔╝██║██╔══╝        ██║   ██║██║   ██║██╔══╝  ██╔══██╗\n\
╚██████╔╝██║  ██║██║ ╚═╝ ██║███████╗      ╚██████╔╝╚██████╔╝███████╗██║  ██║\n\
 ╚═════╝ ╚═╝  ╚═╝╚═╝     ╚═╝╚══════╝       ╚═════╝  ╚═════╝ ╚══════╝╚═╝  ╚═╝\n\
                       Press R to restart the game ! \n\
                       Press M to go back to menu ! \n\
                       Press Q to quit ! ";

const PAUSE_TEXT: &str = "\n\
██████╗  █████╗ ██╗   ██╗███████╗███████╗\n\
██╔══██╗██╔══██╗██║   ██║██╔════╝██╔════╝\n\
██████╔╝███████║██║   ██║███████╗█████╗  \n\
██╔═══╝ ██╔══██║██║   ██║╚════██║██╔══╝  \n\
██║     ██║  ██║╚██████╔╝███████║███████╗\n\
╚═╝     ╚═╝  ╚═╝ ╚═════╝ ╚══════╝╚══════╝";
//goodbye
const MENU_TEXT: &str = "\n\
███╗   ███╗███████╗███╗   ██╗██╗   ██╗
████╗ ████║██╔════╝████╗  ██║██║   ██║
██╔████╔██║█████╗  ██╔██╗ ██║██║   ██║
██║╚██╔╝██║██╔══╝  ██║╚██╗██║██║   ██║
██║ ╚═╝ ██║███████╗██║ ╚████║╚██████╔╝
╚═╝     ╚═╝╚══════╝╚═╝  ╚═══╝ ╚═════╝ ";

const FAREWELL_TEXT: &str = "\n\
██████╗ ██╗   ██╗███████╗    ██████╗ ██╗   ██╗███████╗
██╔══██╗╚██╗ ██╔╝██╔════╝    ██╔══██╗╚██╗ ██╔╝██╔════╝
██████╔╝ ╚████╔╝ █████╗█████╗██████╔╝ ╚████╔╝ █████╗  
██╔══██╗  ╚██╔╝  ██╔══╝╚════╝██╔══██╗  ╚██╔╝  ██╔══╝  
██████╔╝   ██║   ███████╗    ██████╔╝   ██║   ███████╗
╚═════╝    ╚═╝   ╚══════╝    ╚═════╝    ╚═╝   ╚══════╝";

const RESTART_TEXT: &str = "\n\
██████╗ ███████╗███████╗████████╗ █████╗ ██████╗ ████████╗
██╔══██╗██╔════╝██╔════╝╚══██╔══╝██╔══██╗██╔══██╗╚══██╔══╝
██████╔╝█████╗  ███████╗   ██║   ███████║██████╔╝   ██║   
██╔══██╗██╔══╝  ╚════██║   ██║   ██╔══██║██╔══██╗   ██║   
██║  ██║███████╗███████║   ██║   ██║  ██║██║  ██║   ██║   
╚═╝  ╚═╝╚══════╝╚══════╝   ╚═╝   ╚═╝  ╚═╝╚═╝  ╚═╝   ╚═╝   ";

/// Print the welcome screen, reminding controls
/// Show Fruit and Speed menus alongside
/// Sadly `slow_blink` and `fast_blink` are rendered anymore on modern terminal...
/// # Panics
/// Will panic if no suitable terminal for displaying ios provided
pub fn greeting(terminal: &mut DefaultTerminal) {
    //terminal.clear().expect("Unusable terminal clear");
    terminal
        .draw(|frame| {
            let area = frame.area();
            let lines = vec![
                Line::from("███████╗ ███╗   ██╗  █████╗  ██╗  ██╗ ███████╗"),
                Line::from("██╔════╝ ████╗  ██║ ██╔══██╗ ██║ ██╔╝ ██╔════╝"),
                Line::from("███████╗ ██╔██╗ ██║ ███████║ █████╔╝  █████╗  "),
                Line::from("╚════██║ ██║╚██╗██║ ██╔══██║ ██╔═██╗  ██╔══╝  "),
                Line::from("███████║ ██║ ╚████║ ██║  ██║ ██║  ██╗ ███████╗"),
                Line::from("╚══════╝ ╚══════╝╚═ ╝  ╚═══╝ ╚═╝  ╚═╝ ╚═╝  ╚═╝"),
                Line::from("Welcome to the craziest Snake ever! 🐍").green(),
                Line::from("+--------+-------------+------+------------+-----+---------+"),
                Line::from("|Controls|  ⬆️➡️⬇️⬅️   |  Q   |  P / space |  M  |    R    | "),
                Line::from("+--------+-------------+------+------------+-----+---------+"),
                Line::from("|Effects | Move/start  | Quit | Pause game | Menu| Restart |"),
                Line::from("+--------+-------------+------+------------+-----+---------+"),
                Line::from("Have a good game !  🎮").green(),
            ];
            let nb_lines = lines.len();
            frame.render_widget(
                //centered horizontally
                Paragraph::new(Text::from(lines)).centered(),
                frame_vertically_centered_rect(area, nb_lines),
            );
            speed_menu(frame);
            fruit_menu(frame);
            //set a border all around the terminal
            frame.render_widget(Block::bordered().border_type(BorderType::Double), area);
        })
        .expect("Unusable terminal render");
    sleep(Duration::from_millis(100));
}
/// Display the fruit menu left aligned, vertically centered
fn fruit_menu(frame: &mut Frame) {
    //adding fruits rules, gonna left aligned for less screen space use
    let mut fruits_lines = Vec::new();
    let tab_jonction = Line::from("+---------+-------+-------------+-------------+");
    fruits_lines.push(tab_jonction.clone());
    fruits_lines.push(Line::from(
        "| Fruit   | Score | Probability | Size Effect |",
    ));
    fruits_lines.push(tab_jonction.clone());
    for (fruit, score, probability, size_effect) in FRUITS_SCORES_PROBABILITIES {
        //:<6 and so on are formating options, e.g., saying aligning left with min 6 chars
        fruits_lines.push(Line::from(format!(
            "| {fruit:<6} | {score:>5} | {probability:>11} | {size_effect:>11} |"
        )));
    }
    fruits_lines.push(tab_jonction);
    let nb_lines = fruits_lines.len();
    //more idiomatic using lines (as in: https://ratatui.rs/recipes/render/display-text/)
    //can mix style inside the same line using Line::from(vec!["hello".green(),
    // " ".into(), "world".green().bold(), "3".into()]),
    frame.render_widget(
        Paragraph::new(Text::from(fruits_lines)).left_aligned(),
        frame_vertically_centered_rect(frame.area(), nb_lines),
    );
}
/// Display the speed menu right aligned, vertically centered
fn speed_menu(frame: &mut Frame) {
    // Speed effects
    let mut speed_lines = Vec::new();
    let speed_tab_jonction = Line::from("+------------+-------+----------------+---------+");
    speed_lines.push(speed_tab_jonction.clone());
    speed_lines.push(Line::from(
        "| Speed Name | Value | Score Modifier | Symbol  |",
    ));
    speed_lines.push(speed_tab_jonction.clone());
    for Speed {
        name,
        value,
        score_modifier,
        symbol,
    } in speed::iter_speed_variants()
    {
        //:<10 and so on are formating options, e.g., saying aligning left with min 10 chars
        speed_lines.push(Line::from(format!(
            "| {name:<10} | {value:>5} | {score_modifier:>14} | {symbol:<6} |"
        )));
    }
    speed_lines.push(speed_tab_jonction);
    let nb_lines = speed_lines.len();
    frame.render_widget(
        Paragraph::new(Text::from(speed_lines)).right_aligned(),
        frame_vertically_centered_rect(frame.area(), nb_lines),
    );
}

fn frame_vertically_centered_rect(area: Rect, lines: usize) -> Rect {
    // Auto center vertically, elsewhere could be manually calculated with y offset
    // by taking height from frame.area
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),                                                // top space
            Constraint::Length(u16::try_from(lines).expect("length overflow")), // content
            Constraint::Fill(1),                                                // bottom space
        ])
        // take second rect to render content
        .split(area)[1]
    // if we want to do manual center horizontally also
    /*Layout::default()
    .direction(Direction::Horizontal)
    .constraints([
        Constraint::Fill(1),                                            // top space
        Constraint::Length(lines.lines().last().unwrap().len() as u16), // content
        Constraint::Fill(1),                                            // bottom space
    ])
    // take second rect to render content
    .split(vertical_layout)[1]*/
}
/// Render a horizontally and vertically centered paragraph with text and color style
/// With the game external border set to the same color as the text
fn render_full_centered_paragraph(frame: &mut Frame, text: &'static str, color: Option<Color>) {
    let area = frame.area();
    frame.render_widget(
        Paragraph::new(text)
            //.style(Style::default().fg(color))
            .alignment(ratatui::layout::Alignment::Center),
        frame_vertically_centered_rect(area, text.lines().count()),
    );
    // Set all screen items in the same color as the menu.
    // Applying style break position of emojis
    // even by previously setting a full style to item to avoid them to inherit the overall one
    // so keep global style to the final situations without the snake moving after
    if let Some(color) = color {
        frame.render_widget(
            Block::default().style(Style::default().fg(color)),
            frame.area(),
        );
    }
}
pub fn game_over_paragraph(frame: &mut Frame) {
    render_full_centered_paragraph(frame, GAME_OVER_TEXT, Some(Color::Red));
}
pub fn pause_paragraph(frame: &mut Frame) {
    render_full_centered_paragraph(frame, PAUSE_TEXT, None);
}
pub fn byebye_paragraph(frame: &mut Frame) {
    render_full_centered_paragraph(frame, FAREWELL_TEXT, Some(Color::DarkGray));
}
pub fn restart_paragraph(frame: &mut Frame) {
    render_full_centered_paragraph(frame, RESTART_TEXT, Some(Color::LightYellow));
}
pub fn menu_paragraph(frame: &mut Frame) {
    //Rgb Purple badly managed by old terminals Color::Rgb(128, 0, 128)
    render_full_centered_paragraph(frame, MENU_TEXT, Some(Color::Gray));
}
