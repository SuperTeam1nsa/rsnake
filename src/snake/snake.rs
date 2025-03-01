use crate::snake::direction::Direction;
use crate::snake::snake_body::SnakeBody;
use crate::snake::speed::Speed;
use ratatui::layout::Rect;
use ratatui::prelude::Widget;

pub struct Snake<'a> {
    position: Rect,
    size_square: u16,
    pub direction: Direction,
    pub is_alive: bool,
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
        size_square: u16,
        body_image: &'a str,
        head_image: &'a str,
        nb_element_body: u16,
    ) -> Snake<'a> {
        let x_ini: u16 = 50;
        let y_ini: u16 = 5;
        Snake {
            position: Rect::new(x_ini, y_ini, size_square, size_square),
            size_square,
            direction: Direction::Left,
            is_alive: true,
            body: SnakeBody::new(
                body_image,
                head_image,
                nb_element_body,
                x_ini,
                y_ini,
                size_square,
            ),
        }
    }
    pub fn grow(&mut self) {
        //todo
    }
    pub fn ramp(&mut self, direction: &Direction) -> Result<(u16, u16), ()> {
        match direction {
            Direction::Up => self.body.up(),
            Direction::Down => self.body.down(),
            Direction::Left => self.body.left(),
            Direction::Right => self.body.right(),
        }
        self.body.head_overlap()
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
