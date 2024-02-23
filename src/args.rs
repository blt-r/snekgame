use clap::Parser;
use eyre::eyre;
use rand::random;

use crate::{game::GameConf, themes};

/// Highly customizable, cross-platform, and blazingly fast terminal snake game
#[derive(Parser, Debug)]
pub struct Cli {
    // =#= Options:
    /// Generate command line completions for given shell
    #[arg(long, value_name = "SHELL")]
    pub generate_completions: Option<clap_complete::Shell>,

    /// Don't display score during the game
    #[arg(long)]
    hide_score: bool,

    /// Seed for RNG. 0 for random seed.
    #[arg(long, default_value_t = 0)]
    pub seed: u64,

    // =#= Game config:
    /// Width of the field
    #[arg(long, default_value_t = 25, help_heading = "Game config")]
    width: usize,

    /// Height of the field
    #[arg(long, default_value_t = 15, help_heading = "Game config")]
    height: usize,

    /// Initial length of the snake
    #[arg(
        long,
        short = 'l',
        default_value_t = 3,
        value_name = "LENGTH",
        help_heading = "Game config"
    )]
    snake_length: usize,

    /// Amount of food on the field
    #[arg(long, short = 'f', default_value_t = 1, help_heading = "Game config")]
    food: u32,

    /// Initial speed. Snake will move N times a second
    #[arg(
        long,
        short = 's',
        default_value_t = 6,
        value_name = "N",
        help_heading = "Game config"
    )]
    speed: u32,

    /// Food needed to increece speed
    #[arg(
        long,
        short = 'a',
        default_value_t = 4,
        value_name = "N",
        help_heading = "Game config"
    )]
    food_to_speed_up: u32,

    // =#= Theme config:
    /// Snake theme
    #[arg(long, default_value_t = themes::SnakeBuiltin::Braille, value_enum, help_heading = "Themes")]
    snake_theme: themes::SnakeBuiltin,

    /// Board theme
    #[arg(long, value_enum, default_value_t = themes::BoardBuiltin::Rounded, help_heading = "Themes")]
    board_theme: themes::BoardBuiltin,

    /// Food theme
    #[arg(long, value_enum, default_value_t = themes::FoodBuiltin::Emoji, help_heading = "Themes")]
    food_theme: themes::FoodBuiltin,
}

/// Validates and creates the game config from cli arguments
pub fn create_game_conf(a: &Cli) -> eyre::Result<GameConf> {
    match () {
        _ if a.height == 0 => Err(eyre!("Height cannot be zero")),
        _ if a.width == 0 => Err(eyre!("Width cannot be zero")),
        _ if a.speed == 0 => Err(eyre!("Speed cannot be zero")),
        _ if a.snake_length == 0 => Err(eyre!("Snake length cannot be zero")),
        _ if a.snake_length > a.width => Err(eyre!(
            "Initial snake length cannot be larger then the width of the field"
        )),
        _ => Ok(GameConf {
            food_to_speed_up: a.food_to_speed_up,
            food_n: a.food,
            initial_speed: a.speed,
            h: a.height,
            w: a.width,
            initial_length: a.snake_length,
            seed: match a.seed {
                0 => random(),
                seed => seed,
            },
        }),
    }
}

pub fn into_theme(args: Cli) -> themes::FullTheme {
    themes::FullTheme {
        board: args.board_theme.into(),
        snake: args.snake_theme.into(),
        food: args.food_theme.into(),
        display_score: !args.hide_score,
    }
}
