use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::Span;
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
            let greeting = Paragraph::new(
                "      __             _        
 _ __/ _\\_ __   __ _| | _____ 
| '__\\ \\| '_ \\ / _` | |/ / _ \\
| |  _\\ \\ | | | (_| |   <  __/
|_|  \\__/_| |_|\\__,_|_|\\_\\___|
",
            )
            .centered()
            .red();
            //NB: easier to use centered line then center_h (my own custom function)
            let sub_title = Span::styled(
                "Welcome to the craziest Snake ever! 🐍",
                Style::default().fg(Color::Yellow),
            )
            .into_centered_line();
            let instructions = Span::styled(
                "Use ⬆️➡️⬇️⬅️  to move and start the Game ! Or 'q' to quit ❌  ",
                Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            )
            .into_centered_line();
            let bordure = Block::bordered().border_type(BorderType::Double);
            frame.render_widget(greeting, frame.area());
            frame.render_widget(bordure, frame.area());
            frame.render_widget(sub_title, top_margin(frame.area(), 5));
            frame.render_widget(instructions, top_margin(frame.area(), 6));
            //if not centered line :
            //frame.render_widget(instructions, center_h(top_margin(frame.area(), 6), 60));

            /*
            let area = center_h(
                frame.area(),
                //span.width buggy with unicode
                Constraint::Length(u16::try_from(45).expect("Too long text to fit in u16")),
            );
            let area_instruct = top_margin(area, 5);
            frame.render_widget(span, area_instruct);
            frame.render_widget(instructions, top_margin(area_instruct, 65));*/
        })
        .expect("Unusable terminal render");
}
