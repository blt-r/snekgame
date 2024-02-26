use clap::Parser;
use eyre::Context;
use rand::random;

use crate::{game::GameConf, themes};

/// Highly customizable, cross-platform, and blazingly fast terminal snake game
#[derive(Parser, Debug)]
pub struct SnekGameCli {
    // =#= Options:
    /// Generate command line completions for given shell
    #[arg(long, value_name = "SHELL")]
    pub complete: Option<clap_complete::Shell>,

    /// Don't display score during the game
    #[arg(long)]
    hide_score: bool,

    /// Seed for RNG. 0 for random seed.
    #[arg(long, default_value_t = 0)]
    seed: u64,

    // =#= Game config:
    /// Width of the field
    #[arg(long, default_value_t = 25, help_heading = "Game config")]
    width: usize,

    /// Height of the field
    #[arg(long, default_value_t = 15, help_heading = "Game config")]
    height: usize,

    /// Make field the size of the terminal window
    #[arg(long, help_heading = "Game config")]
    fullscreen: bool,

    /// Makes the walls solid
    #[arg(long, short = 'w', help_heading = "Game config")]
    walls: bool,

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
pub fn create_game_conf(a: &SnekGameCli) -> eyre::Result<GameConf> {
    let w: usize;
    let h: usize;

    if a.fullscreen {
        let win = crossterm::terminal::window_size()
            .wrap_err("Unable to get the size of the terminal window")?;
        w = ((win.columns - 2) / 2) as usize;
        h = (win.rows - 2) as usize;
    } else {
        w = a.width;
        h = a.height;
    }

    match () {
        _ if h == 0 => eyre::bail!("Height cannot be zero"),
        _ if w == 0 => eyre::bail!("Width cannot be zero"),
        _ if a.speed == 0 => eyre::bail!("Speed cannot be zero"), // TODO: when snake speed is 0 make the snake move only on input
        _ if a.snake_length == 0 => eyre::bail!("Snake length cannot be zero"),
        _ if a.snake_length > w => {
            eyre::bail!("Initial snake length cannot be larger then the width of the field")
        }
        _ => (),
    }
    Ok(GameConf {
        food_to_speed_up: a.food_to_speed_up,
        food_n: a.food,
        initial_speed: a.speed,
        height: h,
        width: w,
        initial_length: a.snake_length,
        seed: match a.seed {
            0 => random(),
            seed => seed,
        },
        solid_walls: a.walls,
    })
}

pub fn into_theme(args: SnekGameCli) -> themes::FullTheme {
    themes::FullTheme {
        board: args.board_theme.into(),
        snake: args.snake_theme.into(),
        food: args.food_theme.into(),
        display_score: !args.hide_score,
    }
}
