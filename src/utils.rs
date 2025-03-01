use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, BorderType, Paragraph};
use ratatui::DefaultTerminal;

pub fn center_h(area: Rect, nb_of_max_char: u16) -> Rect {
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
}
pub fn greeting(terminal: &mut DefaultTerminal) {
    terminal.clear().expect("Unusable terminal clear");
    terminal
        .draw(|frame| {
            //more idiomatic using lines (as in :https://ratatui.rs/recipes/render/display-text/)
            //can mix style inside the same line using Line::from(vec!["hello".green(), " ".into(), "world".green().bold(), "3".into()]),
            let text = Text::from(vec![
                Line::from("███████╗ ███╗   ██╗  █████╗  ██╗  ██╗ ███████╗"),
                Line::from("██╔════╝ ████╗  ██║ ██╔══██╗ ██║ ██╔╝ ██╔════╝"),
                Line::from("███████╗ ██╔██╗ ██║ ███████║ █████╔╝  █████╗"),
                Line::from("╚════██║ ██║╚██╗██║ ██╔══██║ ██╔═██╗  ██╔══╝"),
                Line::from("███████║ ██║ ╚████║ ██║  ██║ ██║  ██╗ ███████╗"),
                Line::from("╚══════╝ ╚══════╝╚═ ╝  ╚═══╝ ╚═╝  ╚═╝ ╚═╝  ╚═╝"),
                Line::from("Welcome to the craziest Snake ever! 🐍").green(),
                Line::from("Use ⬆️➡️⬇️⬅️  to move and start the Game ! Or 'q' to quit ❌  ")
                    .bold()
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
