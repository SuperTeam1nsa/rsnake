use ratatui::buffer::Buffer;
use ratatui::layout::{Positions, Rect};
use ratatui::prelude::{Color, Span, Style};

// &Span::styled("❄️", Style::default().fg(Color::Cyan))
#[derive(Clone, Debug, PartialEq)]
pub struct GraphicBlock<'a> {
    pub(crate) position: Position,
    image: Span<'a>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Position {
    pub(crate) x: u16,
    pub(crate) y: u16,
}

impl<'a> GraphicBlock<'a> {
    //If wanna less lifetime take a static str to allow Span 'static, (but no dynamic text)
    // Or if wanna a dynamic Graphic block loading (I.e., dynamically created text), take a String, and generate spawn
    // (each time, performance issue).
    pub fn new(position: Position, image: &'a str, style: Style) -> GraphicBlock<'a> {
        GraphicBlock {
            position,
            image: Span::styled(image, style),
        }
    }
    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }
    pub fn get_position(&self) -> &Position {
        &self.position
    }
}

impl<'a> ratatui::prelude::Widget for GraphicBlock<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        //if we are outside of the area the lib will panic, so if windows is resize down,
        // part of graphic won't be shown but game won't crash :) (wanted behaviour)
        //and the refreshing loop will refresh up (resizing the affichage) as soon the windows resize
        if area.y + self.position.y < area.height && area.x + self.position.x < area.width {
            buf.set_span(
                area.x + self.position.x,
                area.y + self.position.y,
                &self.image,
                //&Span::raw(&self.image),
                area.width,
            );
        }
    }
}
