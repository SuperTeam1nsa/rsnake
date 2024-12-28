use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Span, Style};
use ratatui::symbols::Marker;
use ratatui::text::ToLine;
use ratatui::widgets::canvas::{Canvas, Circle, Context};
use ratatui::widgets::Widget;
use std::char::from_u32;
use std::cmp::PartialEq;
use std::ops::Div;
use strum::IntoEnumIterator;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
//only integer this way (but possible to fake it with 100, 75 etc)
#[derive(Debug, PartialEq, strum::Display, strum::EnumIter, Clone)]
pub enum PredDefinedSpeed {
    Slow = 75,
    Normal = 100,
    Fast = 150,
    Tremendous = 200,
}
pub struct Speed {
    current: f32,
    pas: f32,
}
#[allow(clippy::cast_precision_loss)]
impl From<PredDefinedSpeed> for f32 {
    fn from(s: PredDefinedSpeed) -> Self {
        (s as i32) as f32
    }
}

impl Speed {
    pub fn new(speed_ini: PredDefinedSpeed, increase: f32) -> Speed {
        Speed {
            current: f32::from(speed_ini) / (100.0f32),
            pas: increase,
        }
    }
    pub fn speed_up(&mut self) {
        self.current += self.pas;
    }
    pub fn speed_down(&mut self) {
        self.current -= self.pas;
    }
    pub fn get_speed_level(&self) -> String {
        for pred_defined_speed in PredDefinedSpeed::iter() {
            if self.current < f32::from(pred_defined_speed.clone()) {
                return pred_defined_speed.to_string();
            }
        }
        PredDefinedSpeed::Tremendous.to_string()
    }
}
pub struct Snake<'a> {
    position: Rect,
    size_square: u16,
    pub direction: Direction,
    //should be a Game attribute
    speed: Speed,
    body: SnakeBody<'a>, // easier but ugly: Vec<Circle>,
                         /*
                         body: vec![Circle {
                                     x: f64::from(position_ini.0),
                                     y: f64::from(position_ini.1),
                                     radius: f64::from(size_square),
                                     color: Color::Cyan,
                                 }]
                          */
}
impl<'a> Snake<'a> {
    pub fn new(
        position_ini: (u16, u16),
        direction: Direction,
        speed: Speed,
        size_square: u16,
        image: &'a str,
        nb_element_body: u16,
    ) -> Snake<'a> {
        Snake {
            position: Rect::new(position_ini.0, position_ini.1, size_square, size_square),
            size_square,
            direction,
            speed,
            body: SnakeBody::new(
                image,
                nb_element_body,
                position_ini.0,
                position_ini.1,
                size_square,
            ),
        }
    }
    pub fn grow(&mut self) {
        //todo
    }
    pub fn ramp(&mut self) {
        match self.direction {
            Direction::Up => self.body.up(),
            Direction::Down => self.body.down(),
            Direction::Left => self.body.left(),
            Direction::Right => self.body.right(),
        }
    }
    pub fn get_widget(&self) -> impl Widget + 'a {
        self.body.clone()
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
// &Span::styled("❄️", Style::default().fg(Color::Cyan))
#[derive(Clone)]
pub struct BodyElement<'a> {
    x: u16,
    y: u16,
    image: Span<'a>,
}
impl<'a> BodyElement<'a> {
    pub fn new(x: u16, y: u16, image: &str) -> BodyElement {
        BodyElement {
            x,
            y,
            image: Span::styled(image, Style::default().fg(Color::Cyan)),
        }
    }
}
impl<'a> ratatui::prelude::Widget for BodyElement<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_span(area.x + self.x, area.y + self.y, &self.image, area.width);
    }
}
#[derive(Clone)]
pub struct SnakeBody<'a> {
    body: Vec<BodyElement<'a>>,
    case_size: u16,
}

impl<'a> SnakeBody<'a> {
    pub fn new(image: &'a str, nb: u16, x: u16, y: u16, case_size: u16) -> SnakeBody<'a> {
        let mut b = SnakeBody {
            body: vec![BodyElement::new(x, y, "🎄")], //🐍🐍 //🎄
            case_size,
        };
        for i in 1..nb {
            b.body.push(BodyElement::new(x + (case_size * i), y, image));
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
