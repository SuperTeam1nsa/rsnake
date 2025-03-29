//! # Speed Struct and Velocity Enum
//!
//! Represents different speed levels using an enum (`Velocity`) and a struct (`Speed`).
//!
//! # Example
//! ```rust
//! use game::{Speed, Velocity};
//!
//! let fast_speed = Speed::new(&Velocity::Fast);
//! println!("Speed: {}", fast_speed.get_speed());
//! ```
//!

use clap::ValueEnum;
use std::str::FromStr;

/// Represents a speed entry with a name and a value.
#[derive(Debug, Copy, Clone)]
pub(crate) struct Speed {
    speed_name: &'static str,
    value: u64,
}

impl Speed {
    /// Creates a new `Speed` instance based on the given `Velocity` level.
    pub fn new(level: &Velocity) -> Self {
        match level {
            Velocity::Slow => Speed {
                speed_name: "Slow",
                value: 150,
            },
            Velocity::Normal => Speed {
                speed_name: "Normal",
                value: 125,
            },
            Velocity::Fast => Speed {
                speed_name: "Fast",
                value: 100,
            },
            Velocity::Tremendous => Speed {
                speed_name: "Tremendous",
                value: 80,
            },
        }
    }

    /// Returns the speed value.
    pub fn get_speed(&self) -> u64 {
        self.value
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
