use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::Span;
use ratatui::widgets::{Block, BorderType, Borders, Padding, Paragraph, Wrap};
use ratatui::{symbols, DefaultTerminal};

pub struct Map<'a> {
    rect: Rect,
    block: Block<'a>,
    case_size: u16,
}
impl Map<'_> {
    /// the two first arguments are a number of case of size 'case_size_in_px' to create the map
    pub fn new<'a>(nb_of_width_case: u16, nb_of_height_case: u16, case_size_in_px: u16) -> Map<'a> {
        Map {
            rect: Rect::new(
                0,
                0,
                nb_of_width_case * case_size_in_px,
                nb_of_height_case * case_size_in_px,
            ),
            block: Block::bordered()
                .border_type(BorderType::Double)
                .title("Snake !"),
            case_size: case_size_in_px,
        }
    }
    pub fn out_of_map(&self, x: u16, y: u16) -> bool {
        !(x < self.rect.width && y < self.rect.height)
    }
    pub fn get_rect(&self) -> &Rect {
        &self.rect
    }
    pub fn get_widget(&self) -> &Block {
        &self.block
    }
    pub fn get_case_size(&self) -> u16 {
        self.case_size
    }
}
