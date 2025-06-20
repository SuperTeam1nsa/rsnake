use crate::controls::speed::Speed;
use crate::graphics::graphic_block::Position;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::Path;
/// Initial position of the snake's head at the start of the game
const INI_POSITION: Position = Position { x: 50, y: 5 };
#[allow(clippy::needless_raw_string_hashes)]
const PARAMS_HEADER: &str = r#"
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
# speed.delay_ms:   time in milliseconds between moves
#
"#;
/// Structure holding all the configuration parameters for the game
#[derive(Parser, Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
#[command(
    version,
    about = "Snake Game with CLI arguments. Quick custom run: cargo run -- -z 👾 -b 🪽 -l 10 ",
    long_about = "A simple Snake game_logic where you can configure the velocity, \
    snake appearance, and more using command-line arguments.
    Example for asian vibes: rsnake -z 🐼 -b 🍥"
)]
#[derive(Default)]
pub struct GameOptions {
    /// Speed of the snake (Slow, Normal, Fast, Tremendous)
    /// Derives `ValueEnum` on the enum Speed and enforces the type
    /// `clap::ValueEnum`, which automatically handles possible values and displays them in the help message.
    /// Now, clap enforces valid inputs without requiring a manual `FromStr` implementation.
    #[arg(
        short,
        long,
        value_enum, default_value_t = Speed::Normal,
        help = "Sets the movement speed of the snake."
    )]
    pub speed: Speed,

    /// Snake symbol (emoji or character)
    /// Defines short value because doublon, as short and long are by default based on the name of the variable
    #[arg(
        short = 'z',
        long,
        default_value = "🎄",
        help = "Symbol used to represent the snake's head. Cool symbol as 😁🤠🤡🥳🥸👺👹👽\
        👾🐼🐉🐍🦀🐳     .
        Do not use one emoji displaying on multiple chars as it will badly be rendered, but multiple characters and classic are allowed as:
        -z ZZ -b aa"
    )]
    pub head_symbol: String,

    /// Snake trail symbol (emoji or character)
    #[arg(
        short,
        long,
        default_value = "❄️",
        help = "Symbol used to represent the snake's body/trail. Cool symbol as 🍁😋🥑🐾🐢🦎🪽🐥\
        🐣♡🦠🦴👣🍥🥮🍪🍩🧊🏴🧨🦑🐟     . 
        Do not use one emoji displaying on multiple chars as it will badly be rendered, but multiple characters are allowed as: -z 🐳 -b 👽-🦴.
        Be careful, because of unicode width, not all combinaison are playable depending of your font/terminal"
    )]
    pub body_symbol: String,

    /// Initial length of the snake
    #[arg(
        short = 'n',
        long,
        default_value_t = 10,
        help = "Defines the initial length of the snake."
    )]
    pub snake_length: u16,

    /// Number of lives
    #[arg(
        short,
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
        help = "Defines the number of fruits available in the game_logic at once."
    )]
    pub nb_of_fruit: u16,

    /// Caps to 60 FPS or not
    #[arg(
        short,
        long,
        default_value_t = false,
        help = "Set to uncaps default FPS limit (by default limit 60 FPS)"
    )]
    pub uncaps_fps: bool,
    /// Classic game with only growing snake
    #[arg(
        long,
        default_value_t = false,
        help = "Classic game_logic with only growing snake, so fruits with negative size effect will have no size effect"
    )]
    pub classic_mode: bool,
}

impl GameOptions {
    /// Resets all parameters to default values
    pub fn reset(&mut self) {
        *self = GameOptions {
            classic_mode: true,
            uncaps_fps: false,
            life: 3,
            nb_of_fruit: 5,
            body_symbol: "o".to_string(),
            head_symbol: "@".to_string(),
            snake_length: 4,
            speed: Speed::Normal, // Assumes Speed has a Default implementation
        }
    }

    /// Returns the initial snake position
    #[must_use]
    pub fn initial_position() -> Position {
        INI_POSITION
    }

    /// Gets the current speed setting
    #[must_use]
    pub fn get_speed(&self) -> Speed {
        self.speed
    }

    /// Sets the game speed
    pub fn set_speed(&mut self, new_speed: Speed) {
        self.speed = new_speed;
    }

    /// Save the current parameters to a TOML file
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be created or written to.
    ///
    /// # Panics
    ///
    /// Panics if the game parameters cannot be serialized to TOML.
    pub fn save_to_toml(&self, path: &str) -> io::Result<()> {
        let toml_string =
            toml::to_string_pretty(self).expect("Failed to serialize GameParameters to TOML");
        let full_output = format!("{PARAMS_HEADER}\n{toml_string}");
        let mut file = File::create(path)?;
        file.write_all(full_output.as_bytes())?;
        Ok(())
    }

    /// Load parameters from a TOML file
    ///     // Save to TOML
    //     params.save_to_toml("config.toml")?;
    //
    //     // Load from TOML
    //     let loaded = GameParameters::load_from_toml("config.toml")?;
    //     println!("{:#?}", loaded);
    //println!("✅ Game loaded with configuration:\n{:#?}", params);
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be opened or read.
    ///
    /// # Panics
    ///
    /// Panic if the file contents cannot be deserialized as valid TOML.
    pub fn load_from_toml<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let params =
            toml::from_str(&contents).expect("Failed to deserialize GameParameters from TOML");
        Ok(params)
    }
    // In real Life, we will validate data
    pub fn validate(&mut self) {
        self.nb_of_fruit = self.nb_of_fruit.clamp(1, 10000);
        self.life = self.life.clamp(1, 9999);
        self.snake_length = self.snake_length.clamp(1, 999);
    }
}
