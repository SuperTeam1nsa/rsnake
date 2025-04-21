use crate::controls::speed;
use crate::game::Speed;
use crate::graphics::fruit::FRUITS_SCORES_PROBABILITIES;
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, BorderType, Paragraph};
use ratatui::DefaultTerminal;
use std::thread::sleep;
use std::time::Duration;

// /Use AI or a generator as : http://patorjk.com/software/taag/#p=display&f=ANSI%20Shadow&t=Pause
const GAME_OVER_TEXT: &str = "\n\
             ██████╗  █████╗ ███╗   ███╗███████╗       ██████╗ ██╗   ██╗███████╗██████╗ \n\
            ██╔═══   ██╔══██╗████╗ ████║██╔════╝      ██╔═══██╗██║   ██║██╔════╝██╔══██╗\n\
            ██║ ████║███████║██╔████╔██║█████╗        ██║   ██║██║   ██║█████╗  ██████╔╝\n\
            ██║   ██║██╔══██║██║╚██╔╝██║██╔══╝        ██║   ██║██║   ██║██╔══╝  ██╔══██╗\n\
            ╚██████╔╝██║  ██║██║ ╚═╝ ██║███████╗      ╚██████╔╝╚██████╔╝███████╗██║  ██║\n\
             ╚═════╝ ╚═╝  ╚═╝╚═╝     ╚═╝╚══════╝       ╚═════╝  ╚═════╝ ╚══════╝╚═╝  ╚═╝\n\
                               Press R to restart the game ! ";
const PAUSE_TEXT: &str = "\n\
██████╗  █████╗ ██╗   ██╗███████╗███████╗
██╔══██╗██╔══██╗██║   ██║██╔════╝██╔════╝
██████╔╝███████║██║   ██║███████╗█████╗  
██╔═══╝ ██╔══██║██║   ██║╚════██║██╔══╝  
██║     ██║  ██║╚██████╔╝███████║███████╗
╚═╝     ╚═╝  ╚═╝ ╚═════╝ ╚══════╝╚══════╝";
//goodbye
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
/*pub fn center_h(area: Rect, nb_of_max_char: u16) -> Rect {
    //, horizontal: Constraint
    let [area] = Layout::horizontal([Constraint::Length(nb_of_max_char)])
        .flex(Flex::Center)
        .areas(area);
    //let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}
pub fn top_margin(area: Rect, marge: u16) -> Rect {
    let [area] = Layout::vertical([Constraint::Length(1)])
        .margin(marge)
        .areas(area);
    area
}*/
/// Print the welcome screen, reminding controls
/// # Panics
/// Will panic if no suitable terminal for displaying ios provided
pub fn greeting(terminal: &mut DefaultTerminal) {
    //terminal.clear().expect("Unusable terminal clear");
    terminal
        .draw(|frame| {
            let lines = vec![
                Line::from("███████╗ ███╗   ██╗  █████╗  ██╗  ██╗ ███████╗"),
                Line::from("██╔════╝ ████╗  ██║ ██╔══██╗ ██║ ██╔╝ ██╔════╝"),
                Line::from("███████╗ ██╔██╗ ██║ ███████║ █████╔╝  █████╗  "),
                Line::from("╚════██║ ██║╚██╗██║ ██╔══██║ ██╔═██╗  ██╔══╝  "),
                Line::from("███████║ ██║ ╚████║ ██║  ██║ ██║  ██╗ ███████╗"),
                Line::from("╚══════╝ ╚══════╝╚═ ╝  ╚═══╝ ╚═╝  ╚═╝ ╚═╝  ╚═╝"),
                Line::from("Welcome to the craziest Snake ever! 🐍").green(),
                Line::from("+--------+-------------+-----------+------------+"),
                Line::from("|Controls|  ⬆️➡️⬇️⬅️   |    Q      |  P / space |"),
                Line::from("+--------+-------------+-----------+------------+"),
                Line::from("|Effects | Move/start  | Quit game | Pause game |"),
                Line::from("+--------+-------------+-----------+------------+"),
                Line::from("Have a good game !  🎮").green(),
            ];

            frame.render_widget(
                Paragraph::new(Text::from(lines))
                    .centered()
                    .block(Block::bordered().border_type(BorderType::Double)),
                frame.area(),
            );

            // Speed effects
            let mut speed_lines = Vec::new();
            let speed_tab_jonction =
                Line::from("+------------+-------+----------------+---------+");
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
            frame.render_widget(
                Paragraph::new(Text::from(speed_lines))
                    .right_aligned()
                    .block(Block::bordered().border_type(BorderType::Double)),
                frame.area(),
            );

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
            //more idiomatic using lines (as in :https://ratatui.rs/recipes/render/display-text/)
            //can mix style inside the same line using Line::from(vec!["hello".green(), " ".into(), "world".green().bold(), "3".into()]),
            frame.render_widget(
                Paragraph::new(Text::from(fruits_lines))
                    .left_aligned()
                    .block(Block::bordered().border_type(BorderType::Double)),
                frame.area(),
            );
        })
        .expect("Unusable terminal render");
    sleep(Duration::from_millis(100));
}

/// Creates a centered paragraph with text and color style
fn retro_paragraph(text: &'static str, color: Color) -> Paragraph<'static> {
    Paragraph::new(text)
        .style(Style::default().fg(color))
        .alignment(ratatui::layout::Alignment::Center)
}
#[must_use]
pub fn game_over_paragraph() -> Paragraph<'static> {
    retro_paragraph(GAME_OVER_TEXT, Color::Red)
}
#[must_use]
pub fn pause_paragraph() -> Paragraph<'static> {
    retro_paragraph(PAUSE_TEXT, Color::Green)
}
#[must_use]
pub fn byebye_paragraph() -> Paragraph<'static> {
    retro_paragraph(FAREWELL_TEXT, Color::DarkGray)
}
#[must_use]
pub fn restart_paragraph() -> Paragraph<'static> {
    retro_paragraph(RESTART_TEXT, Color::LightYellow)
}
