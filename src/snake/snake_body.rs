use crate::map::Map;
use crate::snake::direction::Direction;
use crate::snake::graphic_block::{GraphicBlock, Position};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Style, Widget};
use ratatui::style::Color;

#[derive(Clone)]
pub struct SnakeBody<'a> {
    pub(crate) body: Vec<GraphicBlock<'a>>,
    case_size: u16,
    posi_ini: (u16, u16),
    size_ini: u16,
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
        let snake_style = Style::default().fg(Color::Cyan);
        let mut b = SnakeBody {
            body: vec![GraphicBlock::new(
                Position { x, y },
                head_image,
                snake_style,
            )], //🐍🐍 //🎄
            case_size,
            posi_ini: (x, y),
            size_ini: nb,
        };
        for i in 1..nb {
            b.body.push(GraphicBlock::new(
                Position {
                    x: x + (case_size * i),
                    y,
                },
                body_image,
                snake_style,
            ));
        }
        b
    }
    pub fn reset(&mut self) {
        //set snake as long as initially
        self.body.truncate(self.size_ini as usize);
        //reset default position for head
        self.body[0].set_position(Position {
            x: self.posi_ini.0,
            y: self.posi_ini.1,
        });
        //recalculate default position for body
        //fancy
        //for b in self.body.iter_mut().skip(1){
        for i in 1..self.size_ini {
            self.body[i as usize].set_position(Position {
                x: self.posi_ini.0 + (self.case_size * i),
                y: self.posi_ini.1,
            });
        }
    }
    pub fn ramping_body(&mut self, previous_head: &Position) {
        let mut current = previous_head.clone();
        let mut previous = current;
        for i in 1..self.body.len() {
            current = self.body[i].get_position().clone();
            self.body[i].set_position(previous);
            previous = current;
        }
    }
    // Check that our head does not touch a part of the body
    pub fn head_position_and_overlap(&self) -> Result<&Position, ()> {
        let head = self.body[0].get_position();
        //fancy
        //for GraphicBlock { x, y, .. } in self.body.iter().skip(1) {
        for b in self.body.iter().skip(1) {
            if head == b.get_position() {
                return Err(());
            }
        }
        Ok(head)
    }
    pub fn left(&mut self) {
        let current = &self.body[0].get_position().clone();
        self.body[0].position.x -= self.case_size;
        self.ramping_body(current);
    }
    pub fn right(&mut self) {
        let current = &self.body[0].get_position().clone();
        self.body[0].position.x += self.case_size;
        self.ramping_body(current);
    }
    pub fn up(&mut self) {
        let current = &self.body[0].get_position().clone();
        self.body[0].position.y -= self.case_size / 2;
        self.ramping_body(current);
    }
    pub fn down(&mut self) {
        let current = &self.body[0].get_position().clone();
        self.body[0].position.y += self.case_size / 2;
        self.ramping_body(current);
    }
    pub fn ramp(&mut self, direction: &Direction, carte: &Map) -> Result<&Position, ()> {
        match direction {
            Direction::Up => self.up(),
            Direction::Down => self.down(),
            Direction::Left => self.left(),
            Direction::Right => self.right(),
        }
        if carte.out_of_map(self.body[0].get_position()) {
            let new_position = carte.out_of_map_reverse_position(self.body[0].get_position());
            self.body[0].set_position(new_position);
            //Err(())
            Ok(self.body[0].get_position())
        } else {
            self.head_position_and_overlap()
        }
    }
    pub fn get_widget(&self) -> impl Widget + 'a {
        self.clone()
        //as using Widget without consuming them #byRef is still unstable
        //otherwise: impl<'a> ratatui::prelude::WidgetRef for BodyElement<'a> {

        //easier with shapes, but bad definition and not fun enough :p
        /*Canvas::default()
        //.marker(Marker::Block)
        .x_bounds([0.0, 200.0])
        .y_bounds([0.0, 200.0])
        .paint(|ctx: &mut Context| {
            for circle in &self.body {
                ctx.draw(circle);
            }
        })*/
    }
}
impl<'a> ratatui::prelude::Widget for SnakeBody<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        for body_element in self.body {
            body_element.render(area, buf);
        }
    }
}
