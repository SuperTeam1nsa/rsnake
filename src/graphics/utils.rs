use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, BorderType, Paragraph};
use ratatui::DefaultTerminal;
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
pub fn greeting(terminal: &mut DefaultTerminal) {
    //terminal.clear().expect("Unusable terminal clear");
    terminal
        .draw(|frame| {
            //more idiomatic using lines (as in :https://ratatui.rs/recipes/render/display-text/)
            //can mix style inside the same line using Line::from(vec!["hello".green(), " ".into(), "world".green().bold(), "3".into()]),
            let text = Text::from(vec![
                Line::from("███████╗ ███╗   ██╗  █████╗  ██╗  ██╗ ███████╗"),
                Line::from("██╔════╝ ████╗  ██║ ██╔══██╗ ██║ ██╔╝ ██╔════╝"),
                Line::from("███████╗ ██╔██╗ ██║ ███████║ █████╔╝  █████╗  "),
                Line::from("╚════██║ ██║╚██╗██║ ██╔══██║ ██╔═██╗  ██╔══╝  "),
                Line::from("███████║ ██║ ╚████║ ██║  ██║ ██║  ██╗ ███████╗"),
                Line::from("╚══════╝ ╚══════╝╚═ ╝  ╚═══╝ ╚═╝  ╚═╝ ╚═╝  ╚═╝"),
                Line::from("Welcome to the craziest Snake ever! 🐍").green(),
                Line::from("Use ⬆️➡️⬇️⬅️  to move and start the Game ! Or 'q' to quit in game 🎮")
                    .green(),
            ]);
            frame.render_widget(
                Paragraph::new(text)
                    .centered()
                    .block(Block::bordered().border_type(BorderType::Double)),
                frame.area(),
            );
        })
        .expect("Unusable terminal render");
}

/// Creates a centered paragraph with text and color style
fn retro_paragraph(text: &'static str, color: Color) -> Paragraph<'static> {
    Paragraph::new(text)
        .style(Style::default().fg(color))
        .alignment(ratatui::layout::Alignment::Center)
}
pub fn game_over_paragraph() -> Paragraph<'static> {
    retro_paragraph(GAME_OVER_TEXT, Color::Red)
}
pub fn pause_paragraph() -> Paragraph<'static> {
    retro_paragraph(PAUSE_TEXT, Color::Green)
}
pub fn byebye_paragraph() -> Paragraph<'static> {
    retro_paragraph(FAREWELL_TEXT, Color::DarkGray)
}
pub fn restart_paragraph() -> Paragraph<'static> {
    retro_paragraph(RESTART_TEXT, Color::LightYellow)
}
