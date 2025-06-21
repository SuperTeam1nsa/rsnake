//! # Direction Enum
//!
//! Represents movement directions in the game.
//!
//! # Example
//! ```rust
//! use rsnaker::controls::direction::Direction;
//!
//! let dir = Direction::Up;
//! match dir {
//!     Direction::Up => println!("Moving up"),
//!     Direction::Down => println!("Moving down"),
//!     Direction::Left => println!("Moving left"),
//!     Direction::Right => println!("Moving right"),
//! }
//! ```
//!

/// Represents possible movement directions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
