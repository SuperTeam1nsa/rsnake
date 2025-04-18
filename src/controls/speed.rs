//! # Speed Struct and Velocity Enum
//!
//! Represents different speed levels using an enum (`Velocity`) and a struct (`Speed`).
//!
//! # Example
//! ```rust
//! use rsnake::controls::speed::Velocity;
//! use rsnake::controls::speed::Speed;
//!
//! let fast_speed = Speed::new(&Velocity::Fast);
//! println!("Speed: {}", fast_speed.get_speed());
//! assert_eq!("Fast", fast_speed.get_name());
//! ```
//!

use clap::ValueEnum;

/// Represents a speed entry with a name and a value.
#[derive(Debug, Copy, Clone)]
pub struct Speed {
    name: &'static str,
    value: u64,
    score_modifier: u16,
    symbol: &'static str,
}

impl Speed {
    /// Creates a new `Speed` instance based on the given `Velocity` level.
    pub fn new(level: &Velocity) -> Self {
        match level {
            Velocity::Slow => Speed {
                name: "Slow",
                value: 150,
                score_modifier: 1,
                symbol: "🐢",
            },
            Velocity::Normal => Speed {
                name: "Normal",
                value: 125,
                score_modifier: 2,
                symbol: "🐍",
            },
            Velocity::Fast => Speed {
                name: "Fast",
                value: 100,
                score_modifier: 3,
                symbol: "🐉",
            },
            Velocity::Tremendous => Speed {
                name: "Tremendous",
                value: 80,
                score_modifier: 4,
                symbol: "🦖", //🪽
            },
        }
    }

    /// Returns the speed value.
    pub fn get_speed(&self) -> u64 {
        self.value
    }

    pub fn get_name(&self) -> &str {
        self.name
    }
    pub fn get_symbol(&self) -> &str {
        self.symbol
    }
    pub fn get_score_modifier(&self) -> u16 {
        self.score_modifier
    }
}

/// Represents possible velocity levels.
#[derive(Debug, Clone, ValueEnum)]
pub enum Velocity {
    Slow,
    Normal,
    Fast,
    Tremendous,
}
/* Auto managed by clap, no manual implementation need
impl FromStr for Velocity {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "slow" => Ok(Velocity::Slow),
            "normal" => Ok(Velocity::Normal),
            "fast" => Ok(Velocity::Fast),
            "tremendous" => Ok(Velocity::Tremendous),
            _ => Err(format!("Invalid velocity: {s}")),
        }
    }
}
*/
