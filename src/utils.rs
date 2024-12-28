use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::Span;
use ratatui::widgets::Paragraph;
use ratatui::DefaultTerminal;

pub fn center_h(area: Rect, horizontal: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
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
            .red()
            .on_dark_gray();
            let span = Span::styled(
                "('q' to quit), Use ⬆️➡️⬇️⬅️ to move",
                Style::default().fg(Color::Yellow),
            );
            frame.render_widget(greeting, frame.area());
            let area = center_h(
                frame.area(),
                //span.width buggy with unicode
                Constraint::Length(u16::try_from(45).expect("Too long text to fit in u16")),
            );
            let area_instruct = top_margin(area, 5);
            frame.render_widget(span, area_instruct);
        })
        .unwrap();
}
