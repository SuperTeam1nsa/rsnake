//! # Fruits Manager
//!
//! This module defines the `FruitsManager` struct, which is responsible for handling fruit objects within a game.
//! It includes methods for spawning, replacing, and interacting with fruits.
//!
//! # Example
//! ```rust
//! use rsnake::game::FruitsManager;
//! use rsnake::graphics::map::Map;
//! use std::sync::Arc;
//! use ratatui::layout::Rect;
//! use rsnake::graphics::graphic_block::Position;
//!
//! let case_size = 2;
//! let map_size = Rect::new(0, 0, 200, 10);
//! let map = Arc::new(Map::new(case_size, map_size));
//! let number_of_fruits_to_manage = 10;
//! let mut manager = FruitsManager::new(number_of_fruits_to_manage, map.clone());
//!
//! // Simulate eating fruits
//! let position = Position { x: 10, y: 20 };
//! if let Some(eaten) = manager.eat_some_fruits(&position) {
//!     manager.replace_fruits(&eaten);
//! }
//! ```
//!

use crate::graphics::fruit::{Fruit, FRUITS_SCORES_PROBABILITIES};
use crate::graphics::graphic_block::Position;
use crate::graphics::map::Map;
use rand::prelude::ThreadRng;
use rand::{rng, Rng};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::Widget;
use ratatui::widgets::WidgetRef;
use std::sync::Arc;

/// Manages fruit objects within the game.
pub struct FruitsManager<'a, 'b: 'a> {
    fruits: Vec<Fruit<'a>>, // List of fruits currently in the game
    carte: Arc<Map<'b>>,    // Reference to the game map
}

impl<'a, 'b> FruitsManager<'a, 'b> {
    /// Creates a new `FruitsManager` with a given number of fruits.
    ///
    /// # Example
    /// ```
    /// use std::sync::Arc;
    /// use ratatui::layout::Rect;
    /// use rsnake::game::FruitsManager;
    /// use rsnake::graphics::map::Map;
    /// let x =42;
    /// let map = Arc::new(Map::new(2, Rect::new(0,0, 160,10)));
    /// let manager = FruitsManager::new(3, map);
    /// ```
    #[must_use]
    pub fn new(nb: u16, carte: Arc<Map<'b>>) -> Self {
        let mut fruits: Vec<Fruit> = Vec::with_capacity(nb as usize);
        for _ in 0..nb {
            fruits.push(Self::spawn_random(&carte));
        }
        Self { fruits, carte }
    }

    /// Spawns a fruit at a random position on the map.
    fn spawn_random(carte: &Map) -> Fruit<'a> {
        let mut rng = rng();
        let position = Self::generate_position_rounded_by_cs(carte, &mut rng);
        let random_value: u16 = rng.random_range(1..100);
        let mut cumulative_probability = 0;
        for &(image, score, probability, size_effect) in FRUITS_SCORES_PROBABILITIES {
            cumulative_probability += probability;
            if random_value <= cumulative_probability {
                return Fruit::new(score, size_effect, position, image);
            }
        }
        // Default fallback fruit
        let (image, score, _, size_effect) = FRUITS_SCORES_PROBABILITIES[0];
        Fruit::new(score, size_effect, position, image)
    }

    /// Replaces eaten fruits with new random ones and ensures balance.
    pub fn replace_fruits(&mut self, fruits_to_remove: &[Fruit<'a>]) {
        let nb = fruits_to_remove.len();
        self.fruits
            .retain(|fruit| !fruits_to_remove.contains(fruit));
        for _ in 0..nb {
            self.fruits.push(Self::spawn_random(&self.carte));
        }
    }

    /// Returns a list of fruits at the given position, copying them to avoid lock contention.
    #[must_use]
    pub fn eat_some_fruits(&self, position: &Position) -> Option<Vec<Fruit<'a>>> {
        let eaten: Vec<Fruit<'a>> = self
            .fruits
            .iter()
            .filter(|x| x.is_at_position(position))
            .cloned()
            .collect();
        if eaten.is_empty() { None } else { Some(eaten) }
    }

    /// Generates a random valid position for spawning fruits.
    fn generate_position_rounded_by_cs(carte: &Map, rng: &mut ThreadRng) -> Position {
        let cs = carte.get_case_size();
        let csy = cs / 2;
        let width = carte.area().width;
        let height = carte.area().height;
        let max_index_x = (width / cs).saturating_sub(cs);
        let max_index_y = (height / csy).saturating_sub(csy);
        Position {
            x: rng.random_range(1..max_index_x) * cs,
            y: rng.random_range(1..max_index_y) * csy,
        }
    }
}

/// Implements `WidgetRef` for rendering fruits on the screen.
impl<'a> WidgetRef for FruitsManager<'a, 'a> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        for fruit in &self.fruits {
            fruit.render_ref(area, buf);
        }
    }
}

/// Implements `Widget` for compatibility with older versions.
impl<'a> Widget for FruitsManager<'a, 'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_ref(area, buf);
    }
}

impl<'a> Widget for &FruitsManager<'a, 'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_ref(area, buf);
    }
}

/// Test part:
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    // Mock definitions – Not really need there, but for example on how to share ressources for test
    // Arc because tests are by default multi-htreaded (can be changed if need with : )
    fn mock_map() -> Arc<Map<'static>> {
        Arc::new(Map::new(2, Rect::new(0, 0, 160, 12)))
    }

    fn dummy_position() -> Position {
        Position { x: 10, y: 10 }
    }

    #[test]
    fn test_new_creates_correct_number_of_fruits() {
        let map = mock_map();
        let manager = FruitsManager::new(5, map);
        assert_eq!(manager.fruits.len(), 5);
    }

    #[test]
    fn test_replace_fruits_removes_and_adds_new() {
        let map = mock_map();
        let mut manager = FruitsManager::new(3, Arc::clone(&map));
        let fruits_to_remove = vec![manager.fruits[0].clone()];
        manager.replace_fruits(&fruits_to_remove);
        assert_eq!(manager.fruits.len(), 3);
        assert!(!manager.fruits.contains(&fruits_to_remove[0]));
    }

    #[test]
    fn test_eat_some_fruits_returns_correct_fruit() {
        let map = mock_map();
        let mut manager = FruitsManager::new(3, Arc::clone(&map));
        let fruit = Fruit::new(10, 1, dummy_position(), "🍎");
        manager.fruits[0] = fruit.clone();
        let result = manager.eat_some_fruits(&dummy_position());
        assert!(result.is_some());
        assert!(result.unwrap().contains(&fruit));
    }

    #[test]
    fn test_eat_some_fruits_returns_none_if_no_fruit() {
        let map = mock_map();
        let manager = FruitsManager::new(3, Arc::clone(&map));
        let result = manager.eat_some_fruits(&Position { x: 999, y: 999 });
        assert!(result.is_none());
    }
}
