/// This module contains the core components of the game. It includes the following sub-modules:
///
/// - [`fruit`]: Represents the fruit element, including its position, score, and image.
/// - [`graphic_block`]: Handles the graphical rendering of game blocks in terminal.
/// - [`map`]: Manages the game map, layout, and boundaries.
/// - [`snake_body`]: Tracks and manages the snake's body and its movement.
/// - [`utils`]: Provides utility functions to assist with common texts and graphical tasks
pub mod fruit;

pub mod graphic_block;

pub mod map;

pub mod snake_body;

pub mod utils;

/* Just to have some fun with moving letter
pub fn left(&mut self) {
        let mut current: u16 = self.body[0].x;
        self.body[0].x -= self.case_size;
        let mut previous: u16 = current;
        for i in 1..self.body.len() {
            current = self.body[i].x;
            self.body[i].x = previous;
            previous = current;
        }
    }
    pub fn right(&mut self) {
        let mut current: u16 = self.body[0].x;
        self.body[0].x += self.case_size;
        let mut previous: u16 = current;
        for i in 1..self.body.len() {
            current = self.body[i].x;
            self.body[i].x = previous;
            previous = current;
        }
    }
    pub fn up(&mut self) {
        let mut current: u16 = self.body[0].y;
        self.body[0].y -= self.case_size / 2;
        let mut previous: u16 = current;
        for i in 1..self.body.len() {
            current = self.body[i].y;
            self.body[i].y = previous;
            previous = current;
        }
    }
    pub fn down(&mut self) {
        let mut current: u16 = self.body[0].y;
        self.body[0].y += self.case_size / 2;
        let mut previous: u16 = current;
        for i in 1..self.body.len() {
            current = self.body[i].y;
            self.body[i].y = previous;
            previous = current;
        }
    }
 */
