use crate::map::Map;
use crate::snake::body_elements::BodyElement;
use crate::snake::direction::Direction;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::Widget;

#[derive(Clone)]
pub struct SnakeBody<'a> {
    pub(crate) body: Vec<BodyElement<'a>>,
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
        let mut b = SnakeBody {
            body: vec![BodyElement::new(x, y, head_image)], //🐍🐍 //🎄
            case_size,
            posi_ini: (x, y),
            size_ini: nb,
        };
        for i in 1..nb {
            b.body
                .push(BodyElement::new(x + (case_size * i), y, body_image));
        }
        b
    }
    pub fn reset(&mut self) {
        //set snake as long as initially
        self.body.truncate(self.size_ini as usize);
        //reset default position for head
        self.body[0].set_position(self.posi_ini.0, self.posi_ini.1);
        //recalculate default position for body
        //fancy
        //for b in self.body.iter_mut().skip(1){
        for i in 1..self.size_ini {
            self.body[i as usize]
                .set_position(self.posi_ini.0 + (self.case_size * i), self.posi_ini.1);
        }
    }
    pub fn ramping_body(&mut self, previous_head: (u16, u16)) {
        let mut current = previous_head;
        let mut previous: (u16, u16) = current;
        for i in 1..self.body.len() {
            current = (self.body[i].x, self.body[i].y);
            self.body[i].x = previous.0;
            self.body[i].y = previous.1;
            previous = current;
        }
    }
    // Check that our head does not touch a part of the body
    pub fn head_position_and_overlap(&self) -> Result<(u16, u16), ()> {
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
    pub fn ramp(&mut self, direction: &Direction, carte: &Map) -> Result<(u16, u16), ()> {
        match direction {
            Direction::Up => self.up(),
            Direction::Down => self.down(),
            Direction::Left => self.left(),
            Direction::Right => self.right(),
        }
        if carte.out_of_map(self.body[0].x, self.body[0].y) {
            let new_position = carte.out_of_map_reverse_position(self.body[0].x, self.body[0].y);
            self.body[0].x = new_position.0;
            self.body[0].y = new_position.1;
            //Err(())
            Ok(new_position)
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
