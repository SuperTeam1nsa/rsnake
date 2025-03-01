use ratatui::text::ToLine;
use ratatui::widgets::Widget;
use std::cmp::PartialEq;
use std::ops::Div;
use strum::IntoEnumIterator;
pub mod body_elements;
pub mod direction;
pub mod snake_body;
pub mod snake_moving;
pub mod speed;

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
