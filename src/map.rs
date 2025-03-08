use crate::snake::graphic_block::Position;
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
    pub fn out_of_map(&self, Position { x, y }: &Position) -> bool {
        let x_max = self.viewport.width - self.case_size;
        let y_max = self.viewport.height - (self.case_size / 2);
        let x_min = self.case_size;
        let y_min = self.case_size / 2;
        *x < x_min || *x > x_max || *y < y_min || *y > y_max
    }
    /*
    If out of map, reverse
     */
    pub fn out_of_map_reverse_position(&self, Position { x, y }: &Position) -> Position {
        let x_max = self.viewport.width - self.case_size;
        let y_max = self.viewport.height - (self.case_size / 2);
        let x_min = self.case_size;
        let y_min = self.case_size / 2;
        if *y > y_max {
            Position { x: *x, y: y_min }
        } else if *y < y_min {
            Position { x: *x, y: y_max }
        } else if *x > x_max {
            Position { x: x_min, y: *y }
        } else if *x < x_min {
            Position { x: x_max, y: *y }
        } else {
            Position { x: *x, y: *y }
        }
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
