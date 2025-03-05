use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Span, Style};

// &Span::styled("❄️", Style::default().fg(Color::Cyan))
#[derive(Clone)]
pub struct GraphicBlock<'a> {
    pub(crate) x: u16,
    pub(crate) y: u16,
    image: Span<'a>,
}

impl<'a> GraphicBlock<'a> {
    //Style::default().fg(Color::Cyan)
    pub fn new(x: u16, y: u16, image: &str, style: Style) -> GraphicBlock {
        GraphicBlock {
            x,
            y,
            image: Span::styled(image, style),
        }
    }
    pub fn set_position(&mut self, new_x: u16, new_y: u16) {
        self.x = new_x;
        self.y = new_y;
    }
    pub fn get_position(&self) -> (u16, u16) {
        (self.x, self.y)
    }
}

impl<'a> ratatui::prelude::Widget for GraphicBlock<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        //if we are outside of the area the lib will panic, so if windows is resize down,
        // part of graphic won't be shown but game won't crash :) (wanted behaviour)
        //and the refreshing loop will refresh up (resizing the affichage) as soon the windows resize
        if area.y + self.y < area.height && area.x + self.x < area.width {
            buf.set_span(area.x + self.x, area.y + self.y, &self.image, area.width);
        }
    }
}
