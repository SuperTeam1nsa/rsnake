use crate::controls::speed::Velocity;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "Snake Game with CLI arguments. Quick custom run: cargo run -- -z 👾 -b 🪽 -l 10 ",
    long_about = "A simple Snake game where you can configure the velocity, \
    snake appearance, and more using command-line arguments.
    Example for asian vibes: rsnake -z 🐼 -b 🍥"
)]
pub struct Cli {
    /// Velocity of the snake (Slow, Normal, Fast, Tremendous)
    /// Uses derive `ValueEnum` on the enum Velocity, and enforce the type
    /// `clap::ValueEnum`, which automatically handles possible values and displays them in the help message.
    /// Now, clap enforces valid inputs without requiring a manual `FromStr` implementation.
    #[arg(
        short,
        long,
        value_enum, default_value_t = Velocity::Normal,
        help = "Sets the movement speed of the snake."
    )]
    pub velocity: Velocity,

    /// Snake symbol (emoji or character)
    /// Define short value because doublon, as short and long are by default based on the name of the variable
    #[arg(
        short = 'z',
        long,
        default_value = "🎄",
        help = "Symbol used to represent the snake's head. Cool symbol as 😁🤠🤡🥳🥸👺👹👽\
        👾🐼🐉🐍🦀     .
        Do not use emoji displaying on multiple chars as it will badly be rendered"
    )]
    pub head_symbol: String,

    /// Snake trail symbol (emoji or character)
    #[arg(
        short,
        long,
        default_value = "❄️",
        help = "Symbol used to represent the snake's body/trail. Cool symbol as 🍁😋🥑🐾🐢🦎🪽🐥\
        🐣🕸️🦠🦴👣🍥🥮🍪🍩🧊     . 
        Do not use emoji displaying on multiple chars as it will badly be rendered."
    )]
    pub body_symbol: String,

    /// Initial length of the snake
    #[arg(
        short,
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
        short,
        long,
        default_value_t = 5,
        help = "Defines the number of fruits available in the game at once."
    )]
    pub nb_of_fruit: u16,

    /// Caps to 60 FPS or not
    #[arg(
        short,
        long,
        default_value_t = true,
        help = "Set to false to uncaps default FPS limit (to 60 FPS)"
    )]
    pub caps_fps: bool,
    /// Classic game with only growing snake
    #[arg(
        long,
        default_value_t = false,
        help = "Classic game with only growing snake, so fruits with negative size effect will have no size effect"
    )]
    pub classic: bool,
}
