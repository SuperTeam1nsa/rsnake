//! # Fruit Management Module
//!
//! This module defines the `Fruit` struct, which represents different fruits in the game and provides the
//!  ability to create, position, and render them.
//!
//! The `FRUITS_SCORES_PROBABILITIES` constant defines various fruits with their respective scores and spawn probabilities.
//!
//! # Example
//! ```rust
//! use game::graphics::fruit::{Fruit, Position};
//!
//! let position = Position { x: 5, y: 10 };
//! let apple = Fruit::new(40, position, "🍎");
//! assert_eq!(apple.get_score(), 40);
//! ```
//!

use crate::graphics::graphic_block::{GraphicBlock, Position};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::Widget;
use ratatui::style::Style;
use ratatui::widgets::WidgetRef;

/// Distribution statistics with weighted lottery / pie chart parts.
pub const FRUITS_SCORES_PROBABILITIES: &[(&str, i32, u16)] = &[
    ("🍇", 5, 5),
    ("🍉", 10, 15),
    ("🍋", 15, 10),
    ("🍌", 20, 15),
    ("🍐", 30, 10),
    ("🍎", 40, 10),
    ("🥝", 50, 10),
    ("🍓", 60, 5),
    ("🍒", 70, 5),
    ("🥥", 0, 5),
    ("🦞", -10, 5),
];

/// Represents a fruit on the map.
/// Fruits have a score value and are displayed as graphical blocks.
#[derive(PartialEq, Debug, Clone)]
pub struct Fruit<'a> {
    score: i32,
    graphic_block: GraphicBlock<'a>,
}

impl<'a> Fruit<'a> {
    /// Creates a new `Fruit` at a given position with an associated score and image.
    pub fn new(score: i32, position: Position, image: &'a str) -> Fruit<'a> {
        Self {
            score,
            graphic_block: GraphicBlock::new(position, image, Style::default()),
        }
    }

    /// Checks if the fruit is at a specific position.
    pub fn is_at_position(&self, position: &Position) -> bool {
        self.graphic_block.get_position() == position
    }

    /// Returns the score of the fruit.
    pub fn get_score(&self) -> i32 {
        self.score
    }
}

/// Enables `Fruit` to be rendered as a widget.
impl Widget for Fruit<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.graphic_block.render(area, buf);
    }
}

/// Enables `Fruit` to be rendered as a reference widget.
impl WidgetRef for Fruit<'_> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        self.graphic_block.render_ref(area, buf);
    }
}
