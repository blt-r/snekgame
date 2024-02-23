#![feature(array_windows)]

use std::time::Duration;
use std::time::Instant;

use clap::Args;
use clap::Parser;

mod args;
mod game;
mod input;
mod render;
mod themes;

use game::Game;
use input::{InputAction, InputBuffer};
use render::Renderer;
use themes::FullTheme;

#[derive(PartialEq)]
enum GameResult {
    Win,
    Lose,
    Interrupted,
}

struct Clock {
    frame_start: Instant, // instant at which the current frame started
}

impl Clock {
    fn new() -> Self {
        Self {
            frame_start: Instant::now(),
        }
    }

    fn frame_end(&mut self, expected_frametime: Duration) {
        let now = Instant::now();
        let since_frame_start = now.duration_since(self.frame_start);

        if since_frame_start >= expected_frametime {
            //self.dt = since_frame_start;
            self.frame_start = now;
        } else {
            std::thread::sleep(expected_frametime - since_frame_start);
            let after_sleep = Instant::now();

            //self.dt = after_sleep.duration_since(self.frame_start);
            self.frame_start = after_sleep;
        }
    }
}

fn play(mut game: Game, theme: &FullTheme) -> eyre::Result<()> {
    let mut renderer = Renderer::init(game.conf.w, game.conf.h)?;

    let mut input_buf = InputBuffer::new();

    let mut clock = Clock::new();

    let game_result = loop {
        match input::handle_inputs(&mut input_buf, &game)? {
            InputAction::None => (),
            InputAction::Quit => break GameResult::Interrupted,
            InputAction::Clear => {
                renderer.clear_terminal();
            }
        }

        let turn = input::turn_to_do(&mut input_buf, &game);
        game.make_step(turn);

        if game.is_dead {
            break GameResult::Lose;
        } else if game.is_win {
            break GameResult::Win;
        }

        renderer.render_game(&game, theme)?;

        clock.frame_end(game.expected_frametime());
    };

    drop(renderer);

    if game_result == GameResult::Win {
        println!("You Won!");
    }

    Ok(())
}

fn main() -> eyre::Result<()> {
    let args = args::Cli::parse();

    if let Some(shell) = args.generate_completions {
        let name = env!("CARGO_BIN_NAME");
        let mut cli = args::Cli::augment_args(clap::Command::new(name));
        clap_complete::generate(shell, &mut cli, name, &mut std::io::stdout());

        return Ok(());
    }

    let conf = args::create_game_conf(&args)?;
    let game = Game::new(conf);
    let theme = args::into_theme(args);

    play(game, &theme)?;
    Ok(())
}
