use crate::snake::body_elements::BodyElement;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;

#[derive(Clone)]
pub struct SnakeBody<'a> {
    body: Vec<BodyElement<'a>>,
    case_size: u16,
}

impl<'a> SnakeBody<'a> {
    pub fn new(
        body_image: &'a str,
        head_image: &'a str,
        nb: u16,
        x: u16,
        y: u16,
        case_size: u16,
    ) -> SnakeBody<'a> {
        let mut b = SnakeBody {
            body: vec![BodyElement::new(x, y, head_image)], //🐍🐍 //🎄
            case_size,
        };
        for i in 1..nb {
            b.body
                .push(BodyElement::new(x + (case_size * i), y, body_image));
        }
        b
    }
    pub fn ramping_body(&mut self, head: (u16, u16)) {
        let mut current = head;
        let mut previous: (u16, u16) = current;
        for i in 1..self.body.len() {
            current = (self.body[i].x, self.body[i].y);
            self.body[i].x = previous.0;
            self.body[i].y = previous.1;
            previous = current;
        }
    }
    // Check that our head does not touch a part of the body
    pub fn head_overlap(&self) -> Result<(u16, u16), ()> {
        let head = (self.body[0].x, self.body[0].y);
        for BodyElement { x, y, .. } in self.body.iter().skip(1) {
            if *x == head.0 && *y == head.1 {
                return Err(());
            }
        }
        Ok(head)
    }
    pub fn left(&mut self) {
        let current = (self.body[0].x, self.body[0].y);
        self.body[0].x -= self.case_size;
        self.ramping_body(current);
    }
    pub fn right(&mut self) {
        let current = (self.body[0].x, self.body[0].y);
        self.body[0].x += self.case_size;
        self.ramping_body(current);
    }
    pub fn up(&mut self) {
        let current = (self.body[0].x, self.body[0].y);
        self.body[0].y -= self.case_size / 2;
        self.ramping_body(current);
    }
    pub fn down(&mut self) {
        let current = (self.body[0].x, self.body[0].y);
        self.body[0].y += self.case_size / 2;
        self.ramping_body(current);
    }
}

impl<'a> ratatui::prelude::Widget for SnakeBody<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        for body_element in self.body {
            body_element.render(area, buf);
        }
    }
}
