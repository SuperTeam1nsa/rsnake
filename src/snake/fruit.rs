use crate::map::Map;
use crate::snake::graphic_block::GraphicBlock;
use rand::prelude::ThreadRng;
use rand::{rng, Rng};
use ratatui::layout::Positions;
use ratatui::style::Style;
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
struct FruitsManager<'a> {
    fruits: Vec<Fruit<'a>>,
    carte: Map<'a>,
}
// Fruit struct to manage fruits on the map.
struct Fruit<'a> {
    score: i16,
    graphic_block: GraphicBlock<'a>,
}
impl<'a> Fruit<'a> {
    // Create a new fruit at a given position
    pub fn new(score: i16, (x, y): (u16, u16), image: &'a str, style: Style) -> Self {
        Self {
            score,
            graphic_block: GraphicBlock::new(x, y, image, style),
        }
    }
    // Check if the fruit is at a specific position
    pub fn is_at_position(&self, position: (u16, u16)) -> bool {
        self.graphic_block.get_position() == position
    }
    pub fn get_score(&self) -> i16 {
        self.score
    }
}
impl FruitsManager<'_> {
    pub fn new(nb: usize, carte: Map) {
        let mut fruits: std::vec::Vec<Fruit> = Vec::new();
        /*for _ in 0..nb {
            fruits.append()
        }
        //todo finish it !
        Self { score,
            graphic_block: GraphicBlock::new(x,y,image,style)
        }*/
    }
    // Function to randomly spawn a fruit on the map
    pub fn spawn_random<'a>(carte: &Map) -> Fruit<'a> {
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
