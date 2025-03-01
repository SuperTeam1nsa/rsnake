use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::Span;
use ratatui::widgets::{Block, BorderType, Borders, Padding, Paragraph, Wrap};
use ratatui::{symbols, DefaultTerminal};
#[derive(Clone)]
pub struct Map<'a> {
    block: Block<'a>,
    case_size: u16,
    viewport: Rect,
}
impl Map<'_> {
    /// the map is resizable and is the writeable size of the terminal
    pub fn new<'a>(case_size_in_px: u16, viewport: Rect) -> Map<'a> {
        Map {
            block: Block::bordered()
                .border_type(BorderType::Double)
                .title("Snake !"),
            case_size: case_size_in_px,
            viewport,
        }
    }
    pub fn out_of_map(&self, x: u16, y: u16) -> bool {
        let x_max = self.viewport.width - self.case_size;
        let y_max = self.viewport.height - (self.case_size / 2);
        let x_min = self.case_size;
        let y_min = self.case_size / 2;
        x < x_min || x > x_max || y < y_min || y > y_max
    }
    pub fn area(&self) -> &Rect {
        &self.viewport
    }
    pub fn get_widget(&self) -> &Block {
        &self.block
    }
    pub fn get_case_size(&self) -> u16 {
        self.case_size
    }
}
