use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Span, Style};

// &Span::styled("❄️", Style::default().fg(Color::Cyan))
#[derive(Clone)]
pub struct BodyElement<'a> {
    pub(crate) x: u16,
    pub(crate) y: u16,
    image: Span<'a>,
}

impl<'a> BodyElement<'a> {
    pub fn new(x: u16, y: u16, image: &str) -> BodyElement {
        BodyElement {
            x,
            y,
            image: Span::styled(image, Style::default().fg(Color::Cyan)),
        }
    }
}

impl<'a> ratatui::prelude::Widget for BodyElement<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        //if we are outside of the area the lib will panic, so if windows is resize down,
        // part of graphic won't be shown but game won't crash :) (wanted behaviour)
        //and the refreshing loop will refresh up (resizing the affichage) as soon the windows resize
        if area.y + self.y < area.height && area.x + self.x < area.width {
            buf.set_span(area.x + self.x, area.y + self.y, &self.image, area.width);
        }
    }
}
