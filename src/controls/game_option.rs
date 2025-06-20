use crate::controls::speed::Speed;
use crate::graphics::graphic_block::Position;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::Path;

/// Initial position of the snake's head at the start of the game
pub const INI_POSITION: Position = Position { x: 50, y: 5 };

#[allow(clippy::needless_raw_string_hashes)]
pub const PARAMS_HEADER: &str = r#"
# Snake Game Configuration
# ---------------------------
# classic_mode:     true for classic rules (walls kill, no wrapping)
# uncaps_fps:       disables frame limiting (true = no limit)
# life:             starting lives
# fruits_nb:        number of fruits on the screen
# body_symbol:      character for the snake's body
# head_symbol:      character for the snake's head
# snake_length:     initial length of the snake
# case_size:        size of each grid cell (visual size)
# speed:            speed level (Slow, Normal, Fast, Tremendous)
#
"#;

/// Structure holding all the configuration parameters for the game
#[derive(Parser, Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
#[command(
    version,
    about = "Snake Game with CLI arguments. Quick custom run: cargo run -- -z 👾 -b 🪽 -l 10 ",
    long_about = "A simple Snake game where you can configure the speed, \
    snake appearance, and more using command-line arguments.
    Example for asian vibes: rsnake -z 🐼 -b 🍥"
)]
pub struct GameOption {
    /// Speed of the snake (Slow, Normal, Fast, Tremendous)
    #[arg(
        short = 'v',
        long,
        value_enum,
        default_value_t = Speed::Normal,
        help = "Sets the movement speed of the snake."
    )]
    pub speed: Speed,

    /// Snake symbol (emoji or character)
    #[arg(
        short = 'z',
        long,
        default_value = "🎄",
        help = "Symbol used to represent the snake's head. Cool symbols: 😁🤠🤡🥳🥸👺👹👽\
        👾🐼🐉🐍🦀🐳     .
        Do not use one emoji displaying on multiple chars as it will badly be rendered, but multiple characters and classic are allowed as:
        -z ZZ -b aa"
    )]
    pub head_symbol: String,

    /// Snake trail symbol (emoji or character)
    #[arg(
        short = 'b',
        long,
        default_value = "❄️",
        help = "Symbol used to represent the snake's body/trail. Cool symbols: 🍁😋🥑🐾🐢🦎🪽🐥\
        🐣♡🦠🦴👣🍥🥮🍪🍩🧊🏴🧨🦑🐟     . 
        Do not use one emoji displaying on multiple chars as it will badly be rendered, but multiple characters are allowed as: -z 🐳 -b 👽-🦴.
        Be careful, because of unicode width, not all combinations are playable depending on your font/terminal"
    )]
    pub body_symbol: String,

    /// Initial length of the snake
    #[arg(
        short = 's',
        long,
        default_value_t = 10,
        help = "Defines the initial length of the snake."
    )]
    pub snake_length: u16,

    /// Number of lives
    #[arg(
        short = 'l',
        long,
        default_value_t = 3,
        help = "Defines the initial number of lives for the player."
    )]
    pub life: u16,

    /// Number of fruits in the game
    #[arg(
        short = 'f',
        long,
        default_value_t = 5,
        help = "Defines the number of fruits available in the game at once."
    )]
    pub fruits_nb: u16,

    /// Size of each grid cell
    #[arg(
        short = 'c',
        long,
        default_value_t = 20,
        help = "Size of each grid cell (visual size)"
    )]
    pub case_size: u16,

    /// Caps to 60 FPS or not
    #[arg(
        short = 'u',
        long,
        default_value_t = false,
        help = "Set to uncaps default FPS limit (by default limit 60 FPS)"
    )]
    pub uncaps_fps: bool,

    /// Classic game with only growing snake
    #[arg(
        long,
        default_value_t = false,
        help = "Classic game with only growing snake, so fruits with negative size effect will have no size effect"
    )]
    pub classic_mode: bool,
}

impl Default for GameOption {
    fn default() -> Self {
        Self {
            speed: Speed::Normal,
            head_symbol: "@".to_string(),
            body_symbol: "o".to_string(),
            snake_length: 4,
            life: 3,
            fruits_nb: 5,
            case_size: 20,
            uncaps_fps: false,
            classic_mode: true,
        }
    }
}

impl GameOption {
    /// Creates a new GameOption instance with values from command line arguments
    pub fn new() -> Self {
        let mut options = GameOption::parse();
        options.validate();
        options
    }

    /// Returns the initial snake position
    pub fn initial_position() -> Position {
        INI_POSITION
    }

    /// Resets all parameters to default values
    pub fn reset(&mut self) {
        *self = GameOption::default();
    }

    /// Save the current parameters to a TOML file
    pub fn save_to_toml(&self, path: &str) -> io::Result<()> {
        let toml_string =
            toml::to_string_pretty(self).expect("Failed to serialize GameOption to TOML");
        let full_output = format!("{PARAMS_HEADER}\n{toml_string}");
        let mut file = File::create(path)?;
        file.write_all(full_output.as_bytes())?;
        Ok(())
    }

    /// Load parameters from a TOML file
    pub fn load_from_toml<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let mut params = toml::from_str(&contents).expect("Failed to deserialize GameOption from TOML");
        params.validate();
        Ok(params)
    }

    /// Validates and clamps the parameters within allowed limits
    pub fn validate(&mut self) {
        self.fruits_nb = self.fruits_nb.clamp(1, 100);
        self.life = self.life.clamp(1, 99);
        self.snake_length = self.snake_length.clamp(1, 50);
        self.case_size = self.case_size.clamp(5, 100);
    }
}
