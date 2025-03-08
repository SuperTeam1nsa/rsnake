use crate::map;
use crate::map::Map;
use crate::snake::graphic_block::{GraphicBlock, Position};
use rand::prelude::ThreadRng;
use rand::{rng, Rng};
use ratatui::buffer::Buffer;
use ratatui::layout::{Positions, Rect};
use ratatui::prelude::Widget;
use ratatui::style::Style;
use std::sync::Arc;
//Distribution alea with Monte Cristo / pie chart parts
const FRUITS_SCORES_PROBABILITIES: &[(&str, i16, u16)] = &[
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
//Map outlive fruit that are recreated dynamically
pub struct FruitsManager<'a, 'b: 'a> {
    fruits: Vec<Fruit<'a>>,
    carte: Arc<Map<'b>>,
}
// Fruit struct to manage fruits on the map.
#[derive(PartialEq, Debug)]
struct Fruit<'a> {
    score: i16,
    graphic_block: GraphicBlock<'a>,
}
impl<'a> Fruit<'a> {
    // Create a new fruit at a given position
    pub fn new(score: i16, (x, y): (u16, u16), image: &'a str, style: Style) -> Fruit<'a> {
        Self {
            score,
            graphic_block: GraphicBlock::new(Position { x, y }, image, style),
        }
    }
    // Check if the fruit is at a specific position
    pub fn is_at_position(&self, position: &Position) -> bool {
        self.graphic_block.get_position() == position
    }
    pub fn get_score(&self) -> i16 {
        self.score
    }
}
impl<'a, 'b> FruitsManager<'a, 'b> {
    pub fn new(nb: usize, carte: Arc<Map<'b>>) -> FruitsManager<'a, 'b> {
        let mut fruits: std::vec::Vec<Fruit> = Vec::with_capacity(nb);
        for _ in 0..nb {
            fruits.push(Self::spawn_random(&carte));
        }
        FruitsManager { fruits, carte }
    }
    // Function to randomly spawn a fruit on the map
    //Lazy one, can spawn on others fruits or snake, it's bonus
    fn spawn_random(carte: &Map) -> Fruit<'a> {
        let mut rng = rng();
        let position = Self::generate_position_rounded_by_cs(carte, &mut rng);
        let random_value: u16 = rng.random_range(1..100);
        let mut cumulative_probability = 0;
        for &(image, score, probability) in FRUITS_SCORES_PROBABILITIES {
            cumulative_probability += probability;
            if random_value <= cumulative_probability {
                return Fruit::new(
                    score,
                    position,
                    image,
                    Style::default().add_modifier(ratatui::style::Modifier::BOLD),
                );
            }
        }
        // Fallback, though this should not happen if probabilities sum to 100
        Fruit::new(
            10,
            position,
            "🍎",
            Style::default().add_modifier(ratatui::style::Modifier::BOLD),
        )
    }
    pub fn replace_fruits(&mut self, fruits_to_remove: Vec<Fruit<'a>>) {
        let nb = fruits_to_remove.len();
        self.fruits
            .retain(|fruit| !fruits_to_remove.contains(fruit));
        for _ in 0..nb {
            self.fruits.push(Self::spawn_random(&self.carte));
        }
    }
    //pub fn eat_fruits(&self, positions: Position) -> Vec<Fruit<'a>> {
    //
    // }
    //Check if some fruits has been eaten
    //If so return the summed score and generate new ones
    /*pub fn fruit_regenerate_and_score(&mut self, position: (u16, u16)) -> Option<i16> {
        //-> Option<i16> {
        let mut score = None;
        let mut fruits_to_generate = 0;
        //In case we eat two or more fruit at the same time (as we allow it at random position generation time)
        //filter with predicate to keep iterating over the vec while having a mut ref on it: using retain filter
        self.fruits.retain(|fruit| {
            //if we eat a fruit
            if fruit.is_at_position(position) {
                score = match score {
                    None => Some(fruit.get_score()),
                    Some(prev) => Some(prev + fruit.get_score()),
                };
                fruits_to_generate += 1;
                false // Remove the fruit from the vector
            } else {
                true // Keep the fruit in the vector
            }
        });
        for _ in 0..fruits_to_generate {
            self.fruits.push(Self::spawn_random(&self.carte));
        }
        score
    }*/
    fn generate_position_rounded_by_cs(carte: &Map, rng: &mut ThreadRng) -> (u16, u16) {
        //NB: if do not want an external dependency, a Xorshift32 can be enought for alea an quickly implemented too
        let cs = carte.get_case_size();
        let width = carte.area().width;
        let height = carte.area().height;
        //in the rare case
        let max_index_x = (width / cs).saturating_sub(1);
        let max_index_y = (height / cs).saturating_sub(1);
        (
            rng.random_range(1..max_index_x) * cs,
            rng.random_range(1..max_index_y) * cs,
        )
    }
}

impl<'a> Widget for FruitsManager<'a, 'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        for fruit in self.fruits {
            fruit.render(area, buf);
        }
    }
}
impl<'a> Widget for Fruit<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.graphic_block.render(area, buf);
    }
}
